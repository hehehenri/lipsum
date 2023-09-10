use im::hashmap::HashMap;
use std::fmt::Display;

use crate::ast::{
    Binary, BinaryOp, Call, Element, Error, First, Function, If, Let, Location, Second, Term,
    Tuple, Var,
};

#[derive(Clone)]
pub enum Value {
    Closure {
        parameters: Vec<Var>,
        body: Term,
        context: Context,
    },
    Int(i32),
    Str(String),
    Bool(bool),
    Tuple {
        first: Box<Value>,
        second: Box<Value>,
    },
    Unit,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Closure {
                parameters: _,
                body: _,
                context: _,
            } => String::from("[closure]"),
            Self::Int(int) => int.to_string(),
            Self::Str(str) => str.to_string(),
            Self::Bool(bool) => bool.to_string(),
            Self::Tuple { first, second } => {
                format!("({}, {})", first.to_string(), second.to_string())
            }
            Self::Unit => String::from("unit"),
        };

        f.write_str(&value)
    }
}

type Context = HashMap<String, Value>;

pub struct RuntimeError {
    pub message: String,
    pub full_text: String,
    pub location: Location,
}

impl From<crate::ast::Error> for RuntimeError {
    fn from(error: Error) -> Self {
        RuntimeError {
            message: error.message,
            full_text: error.full_text,
            location: error.location,
        }
    }
}

fn invalid_comparison(l_value: &Value, r_value: &Value, location: &Location) -> RuntimeError {
    RuntimeError {
        message: String::from("invalid comparison"),
        full_text: format!("{} and {} cannot be compared", l_value, r_value),
        location: location.clone(),
    }
}

fn apply(callee: Term, arguments: Vec<Term>, context: &Context) -> Result<Value, RuntimeError> {
    let eval_arguments: Vec<Value> = arguments
        .iter()
        .map(|arg| eval(arg.clone(), context))
        .collect::<Result<_, _>>()?;

    match eval(callee.clone(), context)? {
        Value::Closure {
            parameters,
            body,
            context,
        } => {
            let mut new_context = Context::new();

            for (index, parameter) in parameters.iter().enumerate() {
                let argument = eval_arguments.get(index).ok_or(RuntimeError {
                    message: String::from("missing function argument"),
                    full_text: format!(
                        "no argument supplied for the `{}` parameter",
                        parameter.text
                    ),
                    location: callee.location().clone(),
                })?;

                new_context.insert(parameter.text.clone(), argument.clone());
            }

            let context = new_context.union(context);

            eval(body, &context)
        }
        value => Err(RuntimeError {
            message: String::from("invalid function call"),
            full_text: format!("{} cannot be called as a function", value),
            location: callee.location().clone(),
        }),
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
            (
                Value::Closure {
                    parameters: _,
                    body: _,
                    context: _,
                },
                Value::Closure {
                    parameters: _,
                    body: _,
                    context: _,
                },
            ) => Err(RuntimeError {
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
            (
                Value::Closure {
                    parameters: _,
                    body: _,
                    context: _,
                },
                Value::Closure {
                    parameters: _,
                    body: _,
                    context: _,
                },
            ) => Err(RuntimeError {
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
            (
                Value::Closure {
                    parameters: _,
                    body: _,
                    context: _,
                },
                Value::Closure {
                    parameters: _,
                    body: _,
                    context: _,
                },
            ) => Err(RuntimeError {
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
            (
                Value::Closure {
                    parameters: _,
                    body: _,
                    context: _,
                },
                Value::Closure {
                    parameters: _,
                    body: _,
                    context: _,
                },
            ) => Err(RuntimeError {
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
            (
                Value::Closure {
                    parameters: _,
                    body: _,
                    context: _,
                },
                Value::Closure {
                    parameters: _,
                    body: _,
                    context: _,
                },
            ) => Err(RuntimeError {
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

pub fn eval(term: Term, context: &Context) -> Result<Value, RuntimeError> {
    match term {
        Term::Error(err) => Err(RuntimeError::from(err)),
        Term::Let(Let {
            name,
            value,
            next,
            location: _,
        }) => {
            let value = eval(*value, context)?;
            let context = context.update(name.text, value);

            eval(*next, &context)
        }
        Term::Function(Function {
            parameters,
            value,
            location: _,
        }) => Ok(Value::Closure {
            parameters,
            body: *value,
            context: context.clone(),
        }),
        Term::Call(Call {
            callee,
            arguments,
            location: _,
        }) => apply(*callee, arguments, context),
        Term::If(If {
            condition,
            then,
            otherwise,
            location: _,
        }) => {
            let condition_value = eval(*condition.clone(), context)?;
            let condition = match condition_value {
                Value::Bool(bool) => Ok(bool),
                _ => Err(RuntimeError {
                    message: String::from("invalid if condition"),
                    full_text: format!(
                        "{} can't be used as an if condition. use a boolean instead",
                        condition_value
                    ),
                    location: condition.location().clone(),
                }),
            }?;

            match condition {
                true => eval(*then, context),
                false => eval(*otherwise, context),
            }
        }
        Term::Binary(Binary {
            lhs,
            op,
            rhs,
            location: _,
        }) => {
            let l_value = eval(*lhs.clone(), context)?;
            let r_value = eval(*rhs.clone(), context)?;

            match op {
                BinaryOp::Eq => l_value.eq(&r_value, lhs.location()),
                BinaryOp::Neq => l_value.neq(&r_value, lhs.location()),
                BinaryOp::Lt => l_value.lt(&r_value, lhs.location()),
                BinaryOp::Lte => l_value.lte(&r_value, lhs.location()),
                BinaryOp::Gt => l_value.gt(&r_value, lhs.location()),
                BinaryOp::Gte => l_value.gte(&r_value, lhs.location()),
                BinaryOp::And => l_value.and(&r_value, lhs.location()),
                BinaryOp::Or => l_value.or(&r_value, lhs.location()),
                BinaryOp::Add => l_value.add(&r_value, lhs.location()),
                BinaryOp::Sub => l_value.sub(&r_value, lhs.location()),
                BinaryOp::Mul => l_value.mul(&r_value, lhs.location()),
                BinaryOp::Div => l_value.div(&r_value, lhs.location()),
                BinaryOp::Rem => l_value.rem(&r_value, lhs.location()),
            }
        }
        Term::Var(var) => {
            let var_value = context.get(&var.text).ok_or(RuntimeError {
                message: String::from("unbound variabe"),
                full_text: format!("no variable `{}` found on the current context", var.text),
                location: var.location,
            })?;

            Ok(var_value.clone())
        }
        Term::Int(int) => Ok(Value::Int(int.value)),
        Term::Str(str) => Ok(Value::Str(str.value)),
        Term::Bool(bool) => Ok(Value::Bool(bool.value)),
        Term::Tuple(Tuple {
            first,
            second,
            location: _,
        }) => {
            let first = eval(*first, context)?;
            let second = eval(*second, context)?;

            Ok(Value::Tuple {
                first: Box::new(first),
                second: Box::new(second),
            })
        }
        Term::First(First { value, location }) => match eval(*value, context)? {
            Value::Tuple { first, second: _ } => Ok(*first),
            _value => Err(RuntimeError {
                message: String::from("invalid expression"),
                full_text: String::from("cannot use first operation from anything but a tuple"),
                location,
            }),
        },
        Term::Second(Second { value, location }) => match eval(*value, context)? {
            Value::Tuple { first: _, second } => Ok(*second),
            _value => Err(RuntimeError {
                message: String::from("invalid expression"),
                full_text: String::from("cannot use second operation from anything but a tuple"),
                location,
            }),
        },
        Term::Print(print) => {
            let print_value = eval(*print.value, context)?;
            println!("{}", print_value);

            Ok(Value::Unit)
        }
    }
}
