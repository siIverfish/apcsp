// #![no_std]

#[macro_use] extern crate alloc;

pub mod basic_types;
pub mod load;
pub mod lines;
pub mod basics;
pub mod values;

/// This is the central parsing function.
/// 
/// It delegates tasks to each module, which handle all of the actual logic.
pub fn run(path: &str) {
    let code = load::load_code(path);
    let line_token = lines::code_to_lines(code);
    let basic_statements = basics::lines_to_statements(line_token);

    let with_values = values::parse_values(basic_statements);

    println!("{:#?}", with_values);
}