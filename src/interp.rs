use im::hashmap::HashMap;
use std::fmt::Display;

use crate::ast::{
    Binary, BinaryOp, Call, Element, File, First, Function, If, Let, Location, Print, Second, Term,
    Var,
};

#[derive(Clone, Debug)]
pub struct Closure {
    parameters: Vec<Var>,
    body: Box<Term>,
    context: Context,
}

#[derive(Clone, Debug)]
pub struct Tuple {
    first: Box<Value>,
    second: Box<Value>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Closure(Closure),
    Int(i32),
    Str(String),
    Bool(bool),
    Tuple(Tuple),
    Unit,
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
            Self::Unit => String::from("unit"),
        };

        f.write_str(&value)
    }
}

type Context = HashMap<String, Value>;

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub full_text: String,
    pub location: Location,
}

fn invalid_comparison(l_value: &Value, r_value: &Value, location: &Location) -> RuntimeError {
    RuntimeError {
        message: String::from("invalid comparison"),
        full_text: format!("{} and {} cannot be compared", l_value, r_value),
        location: location.clone(),
    }
}

impl Value {
    pub fn eq(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(l_bool == r_bool)),
            (Value::Str(l_str), Value::Str(r_str)) => Ok(Value::Bool(l_str == r_str)),
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Bool(l_int == r_int)),
            (l_value, r_value) => Err(invalid_comparison(l_value, r_value, location)),
        }
    }

    pub fn neq(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(l_bool != r_bool)),
            (Value::Str(l_str), Value::Str(r_str)) => Ok(Value::Bool(l_str != r_str)),
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Bool(l_int != r_int)),
            (l_value, r_value) => Err(invalid_comparison(l_value, r_value, location)),
        }
    }

    pub fn lt(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(l_bool < r_bool)),
            (Value::Str(l_str), Value::Str(r_str)) => Ok(Value::Bool(l_str < r_str)),
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Bool(l_int < r_int)),
            (l_value, r_value) => Err(invalid_comparison(l_value, r_value, location)),
        }
    }

    pub fn lte(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(l_bool <= r_bool)),
            (Value::Str(l_str), Value::Str(r_str)) => Ok(Value::Bool(l_str <= r_str)),
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Bool(l_int <= r_int)),
            (l_value, r_value) => Err(invalid_comparison(l_value, r_value, location)),
        }
    }

    pub fn gt(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(l_bool > r_bool)),
            (Value::Str(l_str), Value::Str(r_str)) => Ok(Value::Bool(l_str > r_str)),
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Bool(l_int > r_int)),
            (l_value, r_value) => Err(invalid_comparison(l_value, r_value, location)),
        }
    }

    pub fn gte(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(l_bool >= r_bool)),
            (Value::Str(l_str), Value::Str(r_str)) => Ok(Value::Bool(l_str >= r_str)),
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Bool(l_int >= r_int)),
            (l_value, r_value) => Err(invalid_comparison(l_value, r_value, location)),
        }
    }

    pub fn and(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(*l_bool && *r_bool)),
            (_l_val, _r_val) => Err(RuntimeError {
                message: String::from("invalid binary operation"),
                full_text: format!("only booleans can be used on short-circuit operations"),
                location: location.clone(),
            }),
        }
    }

    pub fn or(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(*l_bool || *r_bool)),
            (_l_val, _r_val) => Err(RuntimeError {
                message: String::from("invalid binary operation"),
                full_text: format!("only booleans can be used on short-circuit operations"),
                location: location.clone(),
            }),
        }
    }

    pub fn add(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Int(l_int + r_int)),
            (Value::Str(_l_bool), Value::Str(_r_bool)) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("strings cannot be added"),
                location: location.clone(),
            }),
            (Value::Bool(_l_bool), Value::Bool(_r_bool)) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("booleans cannot be added"),
                location: location.clone(),
            }),
            (Value::Closure(_l_closure), Value::Closure(_r_closure)) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("closures cannot be added"),
                location: location.clone(),
            }),
            (_l_val, _r_val) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("different types cannot be used on the same operation"),
                location: location.clone(),
            }),
        }
    }

    pub fn sub(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Int(l_int - r_int)),
            (Value::Str(_l_bool), Value::Str(_r_bool)) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("strings cannot be subtracted"),
                location: location.clone(),
            }),
            (Value::Bool(_l_bool), Value::Bool(_r_bool)) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("booleans cannot be subtracted"),
                location: location.clone(),
            }),
            (Value::Closure(_l_closure), Value::Closure(_r_closure)) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("closures cannot be subtracted"),
                location: location.clone(),
            }),
            (_l_val, _r_val) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("different types cannot be used on the same operation"),
                location: location.clone(),
            }),
        }
    }

    pub fn mul(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Int(l_int - r_int)),
            (Value::Str(_l_bool), Value::Str(_r_bool)) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("strings cannot be multiplied"),
                location: location.clone(),
            }),
            (Value::Bool(_l_bool), Value::Bool(_r_bool)) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("booleans cannot be multiplied"),
                location: location.clone(),
            }),
            (Value::Closure(_l_closure), Value::Closure(_r_closure)) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("closures cannot be multiplied"),
                location: location.clone(),
            }),
            (_l_val, _r_val) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("different types cannot be used on the same operation"),
                location: location.clone(),
            }),
        }
    }

    pub fn div(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Int(l_int / r_int)),
            (Value::Str(_l_bool), Value::Str(_r_bool)) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("strings cannot be divided"),
                location: location.clone(),
            }),
            (Value::Bool(_l_bool), Value::Bool(_r_bool)) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("booleans cannot be divided"),
                location: location.clone(),
            }),
            (Value::Closure(_l_closure), Value::Closure(_r_closure)) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("closures cannot be divided"),
                location: location.clone(),
            }),
            (_l_val, _r_val) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("different types cannot be used on the same operation"),
                location: location.clone(),
            }),
        }
    }

    pub fn rem(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Int(l_int / r_int)),
            (Value::Str(_l_bool), Value::Str(_r_bool)) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("strings cannot be used with rem"),
                location: location.clone(),
            }),
            (Value::Bool(_l_bool), Value::Bool(_r_bool)) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("booleans cannot be used with rem"),
                location: location.clone(),
            }),
            (Value::Closure(_l_closure), Value::Closure(_r_closure)) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("closures cannot be used with rem"),
                location: location.clone(),
            }),
            (_l_val, _r_val) => Err(RuntimeError {
                message: String::from("invalid numeric operation"),
                full_text: String::from("different types cannot be used on the same operation"),
                location: location.clone(),
            }),
        }
    }
}

fn eval_let(let_: Let, context: &Context) -> Result<Value, RuntimeError> {
    let value = eval(let_.value, context)?;
    let context = context.update(let_.name.text, value);

    eval(let_.next, &context)
}

fn update_context(
    parameters: &[Var],
    arguments: &[Term],
    acc: Context,
    location: Location,
) -> Result<Context, RuntimeError> {
    match (parameters, arguments) {
        ([], [_]) | ([_], []) | ([], [_, ..]) | ([_, ..], []) => Err(RuntimeError {
            message: String::from("invalid arguments"),
            full_text: format!(
                "expecting {} arguments but got {}",
                parameters.len(),
                arguments.len()
            ),
            location,
        }),
        ([], []) => Ok(acc),
        ([parameter], [argument]) => {
            let argument = eval(Box::new(argument.clone()), &acc)?;

            Ok(acc.update(parameter.text.clone(), argument))
        }
        ([parameter, parameters @ ..], [argument, arguments @ ..]) => {
            let argument = eval(Box::new(argument.clone()), &acc)?;

            let acc = acc.update(parameter.text.clone(), argument);

            update_context(parameters, arguments, acc, location)
        }
    }
}

fn eval_call(call: Call, context: Context) -> Result<Value, RuntimeError> {
    match eval(call.callee, &context)? {
        Value::Closure(closure) => {
            // TODO: using this approach, closure would have access to values defined before and
            // after the current scope, i.e:
            //
            // let x = 3;
            // let function = () => {y};
            // let y = 4;
            // print(function()): 4

            let context = closure.context.union(context);

            let context = update_context(
                closure.parameters.as_slice(),
                call.arguments.as_slice(),
                context,
                call.location,
            )?;

            eval(closure.body, &context)
        }
        value => Err(RuntimeError {
            message: String::from("invalid function call"),
            full_text: format!("{} cannot be called as a function", value),
            location: call.location,
        }),
    }
}

fn eval_if(if_: If, context: &Context) -> Result<Value, RuntimeError> {
    let condition_result = eval(if_.condition.clone(), context)?;
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
        true => eval(if_.then, context),
        false => eval(if_.otherwise, context),
    }
}

fn eval_binary(binary: Binary, context: &Context) -> Result<Value, RuntimeError> {
    let l_value = eval(binary.lhs.clone(), context)?;
    let r_value = eval(binary.rhs, context)?;

    match binary.op {
        BinaryOp::Eq => l_value.eq(&r_value, binary.lhs.location()),
        BinaryOp::Neq => l_value.neq(&r_value, binary.lhs.location()),
        BinaryOp::Lt => l_value.lt(&r_value, binary.lhs.location()),
        BinaryOp::Lte => l_value.lte(&r_value, binary.lhs.location()),
        BinaryOp::Gt => l_value.gt(&r_value, binary.lhs.location()),
        BinaryOp::Gte => l_value.gte(&r_value, binary.lhs.location()),
        BinaryOp::And => l_value.and(&r_value, binary.lhs.location()),
        BinaryOp::Or => l_value.or(&r_value, binary.lhs.location()),
        BinaryOp::Add => l_value.add(&r_value, binary.lhs.location()),
        BinaryOp::Sub => l_value.sub(&r_value, binary.lhs.location()),
        BinaryOp::Mul => l_value.mul(&r_value, binary.lhs.location()),
        BinaryOp::Div => l_value.div(&r_value, binary.lhs.location()),
        BinaryOp::Rem => l_value.rem(&r_value, binary.lhs.location()),
    }
}

fn eval_var(var: Var, context: &Context) -> Result<Value, RuntimeError> {
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

fn eval_tuple(tuple: crate::ast::Tuple, context: &Context) -> Result<Value, RuntimeError> {
    let first = eval(tuple.first, context)?;
    let second = eval(tuple.second, context)?;

    Ok(Value::Tuple(Tuple {
        first: Box::new(first),
        second: Box::new(second),
    }))
}

fn eval_first(first: First, context: &Context) -> Result<Value, RuntimeError> {
    match eval(first.value, context)? {
        Value::Tuple(Tuple { first, second: _ }) => Ok(*first),
        _value => Err(RuntimeError {
            message: String::from("invalid expression"),
            full_text: String::from("cannot use first operation from anything but a tuple"),
            location: first.location,
        }),
    }
}

fn eval_second(second: Second, context: &Context) -> Result<Value, RuntimeError> {
    match eval(second.value, context)? {
        Value::Tuple(Tuple { first: _, second }) => Ok(*second),
        _value => Err(RuntimeError {
            message: String::from("invalid expression"),
            full_text: String::from("cannot use second operation from anything but a tuple"),
            location: second.location,
        }),
    }
}

fn eval_print(print: Print, context: &Context) -> Result<Value, RuntimeError> {
    let print_value = eval(print.value, context)?;
    println!("{}", print_value);

    Ok(Value::Unit)
}

fn eval(term: Box<Term>, context: &Context) -> Result<Value, RuntimeError> {
    match *term {
        Term::Let(let_) => eval_let(let_, context),
        Term::Int(int) => Ok(Value::Int(int.value)),
        Term::Str(str) => Ok(Value::Str(str.value)),
        Term::Bool(bool) => Ok(Value::Bool(bool.value)),
        Term::Function(Function {
            parameters,
            value,
            location: _,
        }) => Ok(Value::Closure(Closure {
            parameters,
            body: value,
            context: context.clone(),
        })),
        Term::Call(call) => eval_call(call, context.clone()),
        Term::If(if_) => eval_if(if_, context),
        Term::Binary(binary) => eval_binary(binary, context),
        Term::Var(var) => eval_var(var, context),
        Term::Tuple(tuple) => eval_tuple(tuple, context),
        Term::First(first) => eval_first(first, context),
        Term::Second(second) => eval_second(second, context),
        Term::Print(print) => eval_print(print, context),
    }
}

pub fn eval_file(file: File) -> Result<Value, RuntimeError> {
    let context = Context::default();

    eval(Box::new(file.expression), &context)
}
