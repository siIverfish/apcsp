// #![no_std]

#[macro_use] extern crate alloc;

pub mod basic_types;

pub mod load;

pub mod lines;

pub mod basics;

pub mod values;

pub fn run(path: &str) {
    let code = load::load_code(path);
    let line_token = lines::code_to_lines(code);
    let basic_statements = basics::lines_to_statements(line_token);

    println!("{:#?}", basic_statements);
}