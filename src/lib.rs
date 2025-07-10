use std::borrow::Cow;

#[cfg(test)]
mod tests;

pub struct CsvRow<'a> {
    pub line: &'a str,
    pub delimiter: char,
    pub literal: bool,
    char_pos: usize,
    byte_pos: usize,
    prev_char: Option<char>,
}

impl<'a> CsvRow<'a> {
    /// Creates a new CsvRow
    ///
    /// # Arguments
    ///
    /// * `line` - A string slice that holds the delimited fields
    /// * `delimiter` - A char that represents the delimiter
    /// * `literal` - A bool that indicates whether the parsed fields should be unescaped or read literally. If true, enclosing and escaping quotes will be included in the results.
    ///
    /// # Examples
    ///
    /// ```
    /// use CsvRow::*;
    /// let row = "a,b,c,d";
    /// let csv = CsvRow::new(row, ',', false);
    /// let vec_t: Vec<_> = vec!["a", "b", "c", "d"];
    /// let vec_r: Vec<_> = csv.collect();
    ///
    /// assert_eq!(vec_t[..], vec_r[..])
    /// ```
    pub fn new(line: &str, delimiter: char, literal: bool) -> CsvRow {
        CsvRow {
            line,
            delimiter,
            literal,
            byte_pos: 0,
            char_pos: 0,
            prev_char: None,
        }
    }
}

impl<'a> Iterator for CsvRow<'a> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.byte_pos > self.line.len() || self.line.len() == 0 {
            return None;
        }

        let charenum = self.line.char_indices().into_iter().skip(self.char_pos);

        let mut byte_length: usize = 0;
        let mut quoted = false;

        for (_, c) in charenum {
            if byte_length == 0 && c == '"' {
                quoted = true;
            }

            if c == self.delimiter {
                if !quoted || (quoted && byte_length > 1 && self.prev_char == Some('"')) {
                    break;
                }
            }

            byte_length += c.len_utf8();
            self.prev_char = Some(c);
        }

        // Get the full field from start to finish
        let mut result = match byte_length {
            0 => "",
            _ => &self.line[self.byte_pos..self.byte_pos + byte_length],
        };

        // Confirm that the field ends with a " as well.
        // (Rust does not have a shortcircuited boolean assignment operator, so no &&= here.)
        // Must be more than just one " also.  
        quoted = quoted && result.len() > 1 && result.ends_with('"');

        self.char_pos += result.chars().count() + 1;
        self.byte_pos += result.len() + self.delimiter.len_utf8();

        if self.literal {
            return Some(Cow::Borrowed(result));
        } else {
            // If the field is in quotes, trim them off
            if quoted {
                result = &result[1..result.len() - 1];
            }

            let result = match result.contains("\"\"") {
                true => Some(Cow::Owned(result.replace("\"\"", "\""))),
                false => Some(Cow::Borrowed(result)),
            };

            return result;
        };
    }
}

/// Returns `Cow::Owned<str> if `expression` requires escaping to be RFC-4180 compliant.
/// 
/// Returns `Cow::Borrowed<str>` referencing `expression` if it does not.
///
/// # Arguments
///
/// * `expression` - A string slice that holds the value to escape
/// * `delimiter` - A char that represents the delimiter used in the CSV document
///
/// # Examples
///
/// ```
/// use csvrow::escape;
/// let expression = "chupacabra";
/// let result = escape(&expression, ',');
/// 
/// assert_eq!(expression, result);
/// 
/// let expression = "this is a \"test\", of course...";
/// let result = escape(&expression, ',');
/// 
/// assert_eq!("\"this is a \"\"test\"\", of course...\"", result)
/// ```
pub fn escape(expression: &str, delimiter: char) -> Cow<str> {
    
    match expression.contains(delimiter) || expression.contains("\"") {
        true => Cow::Owned (format!("\"{}\"", expression.replace("\"", "\"\""))),
        false => Cow::Borrowed(expression),
    }
}

