// use apcsp::run;

fn main() {
    let tokens = apcsp::values::parse_value("(4 + 8 * (6 - 3)) / 4 = 7".into());

    println!("{:#?}", tokens);
}
