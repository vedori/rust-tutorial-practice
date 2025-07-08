// Vecs can only contain one type of item
// However, you can use enums to represent an item that could be one of many types

/*
 * Gets values from a row in a spreadsheet
 * some cof the columns in the row contain integers, floating-point numbers, and strings
 *
 * An enum can be defined with those variants in mind
 * The enum forms one type which can be used to construct a vector of said type
 */
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn example() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
