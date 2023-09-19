pub mod ast;
pub mod interp;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

pub fn parse(file_path: &str) -> Result<ast::File, ParseError> {
    let file_json = std::fs::read_to_string(file_path).unwrap();

    serde_json::from_str(&file_json).map_err(|err| ParseError {
        message: err.to_string(),
    })
}
