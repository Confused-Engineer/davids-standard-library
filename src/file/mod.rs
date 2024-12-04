#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

/// Read file to a Vec<String>
pub fn read_lines(filename: &str) -> std::io::Result<Vec<String>> {
    Ok(std::fs::read_to_string(filename)?  
    .lines()  
    .map(String::from)  
    .collect())  
}