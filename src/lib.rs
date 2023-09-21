use ast::File;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub mod ast;
pub mod interp;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

#[wasm_bindgen]
extern "C" {
    type Buffer;

    #[wasm_bindgen(method, getter)]
    pub fn buffer(this: &Buffer) -> js_sys::ArrayBuffer;

    #[wasm_bindgen(method, getter, js_name = byteOffset)]
    pub fn byte_offset(this: &Buffer) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn length(this: &Buffer) -> u32;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[wasm_bindgen(js_name = readFileSync, catch)]
    fn readFileSync(path: &str) -> Result<Buffer, JsValue>;
}

fn read_file(path: &str) -> String {
    let buffer = readFileSync(path).unwrap();
    let buffer =
        js_sys::Uint8Array::new_with_byte_offset(&buffer.buffer(), buffer.byte_offset()).to_vec();

    std::str::from_utf8(buffer.as_slice()).unwrap().to_string()
}

pub fn parse(path: &str) -> Result<File, ParseError> {
    let file_json = read_file(&path);

    serde_json::from_str(&file_json).map_err(|err| ParseError {
        message: err.to_string(),
    })
}

fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn exec(path: String) {
    set_panic_hook();

    let file = parse(&path).unwrap();

    let _ = interp::eval_file(file).unwrap();
}
