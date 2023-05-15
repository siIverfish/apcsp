use std::fs::read_to_string;

pub fn load_code(path: &str) -> String {
    read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read file at path: {:?}", path))
}