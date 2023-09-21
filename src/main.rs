use std::io;

use rinha::interp::eval_file;

fn main() {
    let mut file_buffer = String::new();
    io::stdin().read_line(&mut file_buffer).unwrap();

    let file_ast = serde_json::from_str(&file_buffer)
        .map_err(|err| err.to_string())
        .unwrap();

    let _ = eval_file(file_ast).unwrap();
}
