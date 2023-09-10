use crate::ast::Term;

struct ParseError {
    pub message: String,
}

pub fn parse(json: String) -> Result<Term, ParseError> {
    serde_json::from_str(&json).map_err(|err| ParseError {
        message: err.to_string(),
    })
}
