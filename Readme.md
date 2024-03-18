# csvrow
===
A small, fast utility library that allows you to wrap a string slice representing a line from a CSV file and iterate over its fields.  Complies with RFC 4180 (CSV format) and UTF8.

### Usage

Add `csvrow` to your `Cargo.toml` file directly, or alternatively type `cargo add csvrow` at a terminal prompt in your project root.

### Example

Creating a CSV Row from a String slice and collecting the results into a Vec:

```rust
use CsvRow::*;

fn get_fields() {

    let row = "rust,is,awesome";
    let csv = CsvRow::new(row, ',', false);
    let vec_t: Vec<_> = csv.collect();
}
```

Fields wrapped in quotes, or containing escaped quotes (in accordance with RFC 4180) will by default be parsed and un-escaped.
This behavior can be overridden with the 'literal' parameter of CsvRow::new

```rust
use CsvRow::*;

fn get_fields() {

    let row = r#""rust",is,"Awesome ""bring me the"" Sauce""#;
    let csv = CsvRow::new(row, ',', false);
    let vec_t: Vec<_> = csv.collect();

    // Will yield:
    // rust
    // is
    // Awesome "bring me the" Sauce

    let row = r#""rust",is,"Awesome ""bring me the"" Sauce""#;
    let csv = CsvRow::new(row, ',', true);
    let vec_t: Vec<_> = csv.collect();

    // Will yield:
    // "rust"
    // is
    // "Awesome ""bring me the"" Sauce"
}
```


License: Unlicense/MIT
