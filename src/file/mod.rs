/// Read file to a Vec<String>
pub fn read_lines(filename: &str) -> std::io::Result<Vec<String>> {
    Ok(std::fs::read_to_string(filename)?  
    .lines()  
    .map(String::from)  
    .collect())  
}