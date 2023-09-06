pub struct Location {
    start: usize,
    end: usize,
    filename: String,
}

pub struct Text {
    text: String,
    location: Location,
}

pub struct Let {
    name: Text,
    value: Box<Expression>,
    next: Box<Expression>,
    location: Location,
}

pub struct Function {
    parameters: Vec<Text>,
    value: Box<Expression>,
    location: Location,
}

pub struct Call {
    callee: Box<Expression>,
    arguments: Vec<Box<Expression>>,
    location: Location,
}

pub struct If {
    condition: Box<Expression>,
    then: Box<Expression>,
    otherwise: Box<Expression>,
    location: Location,
}

pub enum Operator {
    Eq,
    Lt,
    Lte,
    Gt,
    Gte,
    Add,
    Mul,
}

pub struct Binary {
    left: Box<Expression>,
    op: Operator,
    right: Box<Expression>,
    location: Location,
}

pub struct Var(Text);

pub struct Int {
    value: usize,
    location: Location,
}

pub struct Str(Text);

pub struct Print {
    value: Box<Expression>,
    location: Location,
}

pub enum Expression {
    Let(Let),
    Function(Function),
    Call(Call),
    If(If),
    Binary(If),
    Var(Var),
    Int(Int),
    Str(Str),
    Print(Print),
}

pub enum Term {}

pub struct Program {
    name: String,
    expression: Expression,
}
