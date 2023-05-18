// use apcsp::run;

fn main() {
    let tokens = apcsp::values::parse_value("foo(4 - 7, bar(8))");

    println!("{:#?}", tokens);
}
