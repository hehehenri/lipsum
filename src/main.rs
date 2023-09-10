fn main() {
    let file_name = "fib.json";
    let mut source = String::new();

    std::fs::read_to_string(&mut source);

    let ast = parser::parse(source).unwrap();

    dbg!(ast);
}
