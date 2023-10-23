use std::fs;

pub fn get_input(file_path : &str) -> String {
    
    let contents = fs::read_to_string(file_path)
        .expect(format!("Should have been able to read the file {file_path}").as_str());
    
    return contents;
}