use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Display,
    hash::{Hash, Hasher},
    rc::Rc,
};
use tailcall::tailcall;

use crate::ast::{
    Binary, Call, Element, First, Function, If, Let, Location, Print, Second, Term, Var,
};

#[derive(Clone, Debug)]
pub struct Closure {
    parameters: Vec<Var>,
    body: Box<Term>,
    context: Rc<RefCell<Context>>,
}

#[derive(Clone, Debug)]
pub struct Tuple {
    first: Box<Value>,
    second: Box<Value>,
}

impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first = self.first.clone();
        let second = self.second.clone();

        write!(f, "({first}, {second})")
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Closure(Closure),
    Int(i64),
    Str(String),
    Bool(bool),
    Tuple(Tuple),
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Closure(_closure) => panic!("this should never be executed"),
            Self::Int(int) => format!("Int({int})").hash(state),
            Self::Str(string) => format!("Str({string})").hash(state),
            Self::Bool(bool) => format!("Bool({bool})").hash(state),
            Self::Tuple(tuple) => format!("Tuple({tuple})").hash(state),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Closure(_closure) => String::from("[closure]"),
            Self::Int(int) => int.to_string(),
            Self::Str(str) => str.to_string(),
            Self::Bool(bool) => bool.to_string(),
            Self::Tuple(tuple) => {
                format!(
                    "({}, {})",
                    tuple.first.to_string(),
                    tuple.second.to_string()
                )
            }
        };

        f.write_str(&value)
    }
}

pub type Cache = std::collections::HashMap<String, Value>;
pub type Context = HashMap<String, Value>;

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub full_text: String,
    pub location: Location,
}

fn eval_let(let_: Let, context: &mut Context, cache: &mut Cache) -> Result<Value, RuntimeError> {
    let name = let_.name.text;

    match eval(let_.value, context, cache)? {
        Value::Closure(closure) => {
            let self_ = Value::Closure(Closure {
                parameters: closure.parameters,
                body: closure.body,
                context: closure.context.clone(),
            });

            closure
                .context
                .borrow_mut()
                .insert(name.clone(), self_.clone());

            context.insert(name, self_.clone());
        }
        value => {
            context.insert(name, value);
        }
    }

    eval(let_.next, context, cache)
}

fn eval_call(call: Call, context: &mut Context, cache: &mut Cache) -> Result<Value, RuntimeError> {
    match eval(call.callee, context, cache)? {
        Value::Closure(closure) => {
            let mut new_context = closure.context.borrow_mut().clone();
            let mut arguments = Vec::new();

            for (parameter, argument) in closure.parameters.into_iter().zip(call.arguments) {
                let argument = eval(Box::new(argument), context, cache)?;
                arguments.push(argument.clone());

                new_context.insert(parameter.text, argument);
            }

            eval(closure.body, &mut new_context, cache)
        }
        value => Err(RuntimeError {
            message: String::from("invalid function call"),
            full_text: format!("{} cannot be called as a function", value),
            location: call.location,
        }),
    }
}

fn eval_if(if_: If, context: &mut Context, cache: &mut Cache) -> Result<Value, RuntimeError> {
    let condition_result = eval(if_.condition.clone(), context, cache)?;
    let condition = match condition_result {
        Value::Bool(bool) => Ok(bool),
        _ => Err(RuntimeError {
            message: String::from("invalid if condition"),
            full_text: format!(
                "{} can't be used as an if condition. use a boolean instead",
                condition_result
            ),
            location: if_.condition.location().clone(),
        }),
    }?;

    match condition {
        true => eval(if_.then, context, cache),
        false => eval(if_.otherwise, context, cache),
    }
}

fn eval_binary(
    binary: Binary,
    context: &mut Context,
    cache: &mut Cache,
) -> Result<Value, RuntimeError> {
    let lhs = eval(binary.lhs.clone(), context, cache)?;
    let rhs = eval(binary.rhs.clone(), context, cache)?;

    lhs.binary_op(binary, rhs)
}

fn eval_var(var: Var, context: &mut Context) -> Result<Value, RuntimeError> {
    context
        .get(&var.text)
        .ok_or(RuntimeError {
            message: format!("unbound variable \"{}\"", var.text),
            full_text: format!(
                "variable \"{}\" was not defined in the current scope",
                var.text
            ),
            location: var.location,
        })
        .map(|value| value.clone())
}

fn eval_tuple(
    tuple: crate::ast::Tuple,
    context: &mut Context,
    cache: &mut Cache,
) -> Result<Value, RuntimeError> {
    let first = eval(tuple.first, context, cache)?;
    let second = eval(tuple.second, context, cache)?;

    Ok(Value::Tuple(Tuple {
        first: Box::new(first),
        second: Box::new(second),
    }))
}

fn eval_first(
    first: First,
    context: &mut Context,
    cache: &mut Cache,
) -> Result<Value, RuntimeError> {
    match eval(first.value, context, cache)? {
        Value::Tuple(Tuple { first, second: _ }) => Ok(*first),
        _value => Err(RuntimeError {
            message: String::from("invalid expression"),
            full_text: String::from("cannot use first operation from anything but a tuple"),
            location: first.location,
        }),
    }
}

fn eval_second(
    second: Second,
    context: &mut Context,
    cache: &mut Cache,
) -> Result<Value, RuntimeError> {
    match eval(second.value, context, cache)? {
        Value::Tuple(Tuple { first: _, second }) => Ok(*second),
        _value => Err(RuntimeError {
            message: String::from("invalid expression"),
            full_text: String::from("cannot use second operation from anything but a tuple"),
            location: second.location,
        }),
    }
}

fn eval_print(
    print: Print,
    context: &mut Context,
    cache: &mut Cache,
) -> Result<Value, RuntimeError> {
    let print_value = eval(print.value, context, cache)?;
    println!("{}", print_value.clone());

    Ok(print_value)
}

fn eval_function(function: Function, context: &mut Context) -> Result<Value, RuntimeError> {
    let context = Rc::new(RefCell::new(context.clone()));

    Ok(Value::Closure(Closure {
        parameters: function.parameters,
        body: function.value.clone(),
        context,
    }))
}

#[tailcall]
pub fn eval(
    term: Box<Term>,
    context: &mut Context,
    cache: &mut Cache,
) -> Result<Value, RuntimeError> {
    match *term {
        Term::Let(let_) => eval_let(let_, context, cache),
        Term::Int(int) => Ok(Value::Int(int.value)),
        Term::Str(str) => Ok(Value::Str(str.value)),
        Term::Bool(bool) => Ok(Value::Bool(bool.value)),
        Term::Function(function) => eval_function(function, context),
        Term::Call(call) => eval_call(call, context, cache),
        Term::If(if_) => eval_if(if_, context, cache),
        Term::Binary(binary) => eval_binary(binary, context, cache),
        Term::Var(var) => eval_var(var, context),
        Term::Tuple(tuple) => eval_tuple(tuple, context, cache),
        Term::First(first) => eval_first(first, context, cache),
        Term::Second(second) => eval_second(second, context, cache),
        Term::Print(print) => eval_print(print, context, cache),
    }
}
