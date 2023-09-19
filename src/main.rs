use rinha::{interp::eval_file, parse};

fn main() {
    let file_ast = parse("./files/fib.json").unwrap();

    let _ = eval_file(file_ast).unwrap();
}
