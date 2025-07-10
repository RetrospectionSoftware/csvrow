use super::*;

#[test]
fn can_parse_tiny_csv() {
    let row = "a,b,c,d";
    let csv = CsvRow::new(row, ',', false);

    // for field in csv {
    //     println!("{field:?}");
    // }

    let vec_t: Vec<_> = vec!["a", "b", "c", "d"];
    let vec_r: Vec<_> = csv.collect();

    assert_eq!(vec_t[..], vec_r[..])
}

#[test]
fn can_parse_empty_row() {
    let row = "";

    let csv = CsvRow::new(row, ',', false);

    let vec_t: Vec<Cow<'_, str>> = vec![];
    let vec_r: Vec<_> = csv.collect();

    assert_eq!(vec_t[..], vec_r[..])
}

#[test]
fn can_parse_simple_csv() {
    let row = r#"january,february,march,april"#;

    let csv = CsvRow::new(row, ',', false);

    let vec_t: Vec<_> = vec!["january", "february", "march", "april"];
    let vec_r: Vec<_> = csv.collect();

    assert_eq!(vec_t[..], vec_r[..])
}

#[test]
fn can_parse_tiny_csv_with_non_ascii() {
    let row = r#"è,b,c,d"#;

    let csv = CsvRow::new(row, ',', false);

    let vec_t: Vec<_> = vec!["è", "b", "c", "d"];
    let vec_r: Vec<_> = csv.collect();

    assert_eq!(vec_t[..], vec_r[..])
}

#[test]
fn can_parse_one_field_csv() {
    let row = "january";

    let csv = CsvRow::new(row, ',', false);

    let vec_t: Vec<_> = vec!["january"];
    let vec_r: Vec<_> = csv.collect();

    assert_eq!(vec_t[..], vec_r[..])
}

#[test]
fn blank_field_mid_string() {
    let row = "january,,april";

    let csv = CsvRow::new(row, ',', false);

    let vec_t: Vec<_> = vec!["january", "", "april"];
    let vec_r: Vec<_> = csv.collect();

    assert_eq!(vec_t[..], vec_r[..])
}

#[test]
fn trailing_comma_yields_empty_string() {
    let row = "january,";

    let csv = CsvRow::new(row, ',', false);

    let vec_t: Vec<_> = vec!["january", ""];
    let vec_r: Vec<_> = csv.collect();

    assert_eq!(vec_t[..], vec_r[..])
}

#[test]
fn can_parse_simple_csv_with_quoted_field() {
    let row = r#"january,february,"leap day",march,april"#;

    let csv = CsvRow::new(row, ',', false);

    let vec_t: Vec<_> = vec!["january", "february", "leap day", "march", "april"];
    let vec_r: Vec<_> = csv.collect();

    assert_eq!(vec_t[..], vec_r[..])
}

#[test]
fn can_parse_csv_with_quoted_field_containing_delim() {
    let row = r#"january,february,"leap day, the",march,april"#;

    let csv = CsvRow::new(row, ',', false);

    let vec_t: Vec<_> = vec!["january", "february", "leap day, the", "march", "april"];
    let vec_r: Vec<_> = csv.collect();

    assert_eq!(vec_t[..], vec_r[..])
}

#[test]
fn can_parse_csv_with_quoted_field_containing_quote() {
    let row = r#"january,february,"The ""Coder"" Man",march,april"#;

    let csv = CsvRow::new(row, ',', false);

    let vec_t: Vec<_> = vec!["january", "february", "The \"Coder\" Man", "march", "april"];
    let vec_r: Vec<_> = csv.collect();

    assert_eq!(vec_t[..], vec_r[..])
}

#[test]
fn can_parse_csv_with_orphaned_quote() {
    let row = r#"january,feb"ruary,march,april"#;

    let csv = CsvRow::new(row, ',', false);

    let vec_t: Vec<_> = vec!["january", "feb\"ruary", "march", "april"];
    let vec_r: Vec<_> = csv.collect();

    assert_eq!(vec_t[..], vec_r[..])
}

#[test]
fn can_parse_csv_with_premature_close_quote() {
    let row = r#"january,"feb"ruary,march,april"#;

    let csv = CsvRow::new(row, ',', false);

    let vec_t: Vec<_> = vec!["january", "\"feb\"ruary,march,april"];
    let vec_r: Vec<_> = csv.collect();

    assert_eq!(vec_t[..], vec_r[..])
}

#[test]
fn can_parse_csv_with_empty_quoted_field() {
    let row = r#"january,"""#;

    let csv = CsvRow::new(row, ',', false);

    let vec_t: Vec<_> = vec!["january", ""];
    let vec_r: Vec<_> = csv.collect();

    assert_eq!(vec_t[..], vec_r[..])
}

#[test]
fn can_parse_simple_csv_with_spaces() {
    let row = r#"january, "february", march, april"#;

    let csv = CsvRow::new(row, ',', false);

    let vec_t: Vec<_> = vec!["january", " \"february\"", " march", " april"];
    let vec_r: Vec<_> = csv.collect();

    assert_eq!(vec_t[..], vec_r[..])
}

#[test]
fn escapes_complex_string() {
    let expression = "this is a \"test\", of course...";
    let result = escape(&expression, ',');

    assert_eq!("\"this is a \"\"test\"\", of course...\"", result)
}

#[test]
fn does_not_escape_simple_string() {
    let expression = "chupacabra";
    let result = escape(&expression, ',');

    assert_eq!(expression, result)
}

#[test]
fn trailing_field_is_comma() {
    let expression = "\"Times-Roman\",\",\"";
    
    let result = CsvRow::new(expression, ',', false);
    let result = result.collect::<Vec<_>>();

    assert_eq!("Times-Roman", result[0]);
}