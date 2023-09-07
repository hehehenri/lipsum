#[derive(Debug, Clone)]
pub struct Location {
    pub start: usize,
    pub end: usize,
    pub filename: String,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub text: String,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct Let {
    pub name: Text,
    pub value: Box<Term>,
    pub next: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<Text>,
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: Box<Term>,
    pub arguments: Vec<Term>,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<Term>,
    pub then: Box<Term>,
    pub otherwise: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Term>,
    pub op: BinaryOperator,
    pub right: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct Var(pub Text);

#[derive(Debug, Clone)]
pub struct Int {
    pub value: isize,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct Str(pub Text);

#[derive(Debug, Clone)]
pub struct Print {
    pub value: Box<Term>,
    pub location: Location,
}

#[derive(Debug, Clone)]
pub enum Term {
    Let(Let),
    Function(Function),
    Call(Call),
    If(If),
    Binary(Binary),
    Var(Var),
    Int(Int),
    Str(Str),
    Print(Print),
}

// TODO: this is gross. define a trait for it or some shit
impl Term {
    pub fn location(&self) -> &Location {
        match self {
            Self::Let(let_) => &let_.location,
            Self::Function(function) => &function.location,
            Self::Call(call) => &call.location,
            Self::If(if_) => &if_.location,
            Self::Binary(binary) => &binary.location,
            Self::Var(var) => &var.0.location,
            Self::Int(int) => &int.location,
            Self::Str(str) => &str.0.location,
            Self::Print(print) => &print.location,
        }
    }
}

pub struct Program {
    pub name: String,
    pub expression: Term,
}
