use crate::ast::File;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

pub fn parse(json: String) -> Result<File, ParseError> {
    serde_json::from_str(&json).map_err(|err| ParseError {
        message: err.to_string(),
    })
}
