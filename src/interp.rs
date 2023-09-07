use std::{error::Error, fmt::Display};

use im::hashmap::HashMap;

use crate::ast::{
    Binary, BinaryOperator, Call, Function, If, Let, Location, Operator, Term, Text, Var,
};

#[derive(Clone)]
pub enum Value {
    Closure {
        parameters: Vec<Text>,
        body: Term,
        context: Context,
    },
    Int(isize),
    Str(String),
    Bool(bool),
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
            Self::Unit => String::from("unit"),
        };

        f.write_str(&value)
    }
}

type Context = HashMap<String, Value>;

pub struct TypeError {
    pub message: String,
    pub location: Location,
}

fn apply(callee: Term, arguments: Vec<Term>, context: &Context) -> Result<Value, TypeError> {
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
                let argument = eval_arguments.get(index).ok_or(TypeError {
                    message: String::from("missing function parameter"),
                    location: callee.location().clone(),
                })?;

                new_context.insert(parameter.text.clone(), argument.clone());
            }

            let context = new_context.union(context);

            eval(body, &context)
        }
        Value::Int(_) => Err(TypeError {
            message: String::from("int cannot be called as a function"),
            location: callee.location().clone(),
        }),
        Value::Str(_) => Err(TypeError {
            message: String::from("str cannot be called as a function"),
            location: callee.location().clone(),
        }),
        Value::Bool(_) => Err(TypeError {
            message: String::from("bool cannot be called as a function"),
            location: callee.location().clone(),
        }),
        Value::Unit => Err(TypeError {
            message: String::from("unit cannot be called as a function"),
            location: callee.location().clone(),
        }),
    }
}

impl Value {
    pub fn eq(&self, value: &Value, location: &Location) -> Result<Value, TypeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(l_bool == r_bool)),
            (Value::Str(l_str), Value::Str(r_str)) => Ok(Value::Bool(l_str == r_str)),
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Bool(l_int == r_int)),
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
            ) => Err(TypeError {
                message: String::from("closures cannot be compared"),
                location: location.clone(),
            }),
            (l_val, r_val) => Err(TypeError {
                message: String::from("different types cannot be compared"),
                location: location.clone(),
            }),
        }
    }

    pub fn neq(&self, value: &Value, location: &Location) -> Result<Value, TypeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(l_bool != r_bool)),
            (Value::Str(l_str), Value::Str(r_str)) => Ok(Value::Bool(l_str != r_str)),
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Bool(l_int != r_int)),
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
            ) => Err(TypeError {
                message: String::from("closures cannot be compared"),
                location: location.clone(),
            }),
            (l_val, r_val) => Err(TypeError {
                message: String::from("different types cannot be compared"),
                location: location.clone(),
            }),
        }
    }

    pub fn lt(&self, value: &Value, location: &Location) -> Result<Value, TypeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(l_bool < r_bool)),
            (Value::Str(l_str), Value::Str(r_str)) => Ok(Value::Bool(l_str < r_str)),
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Bool(l_int < r_int)),
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
            ) => Err(TypeError {
                message: String::from("closures cannot be compared"),
                location: location.clone(),
            }),
            (l_val, r_val) => Err(TypeError {
                message: String::from("different types cannot be compared"),
                location: location.clone(),
            }),
        }
    }

    pub fn lte(&self, value: &Value, location: &Location) -> Result<Value, TypeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(l_bool <= r_bool)),
            (Value::Str(l_str), Value::Str(r_str)) => Ok(Value::Bool(l_str <= r_str)),
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Bool(l_int <= r_int)),
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
            ) => Err(TypeError {
                message: String::from("closures cannot be compared"),
                location: location.clone(),
            }),
            (l_val, r_val) => Err(TypeError {
                message: String::from("different types cannot be compared"),
                location: location.clone(),
            }),
        }
    }

    pub fn gt(&self, value: &Value, location: &Location) -> Result<Value, TypeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(l_bool > r_bool)),
            (Value::Str(l_str), Value::Str(r_str)) => Ok(Value::Bool(l_str > r_str)),
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Bool(l_int > r_int)),
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
            ) => Err(TypeError {
                message: String::from("closures cannot be compared"),
                location: location.clone(),
            }),
            (l_val, r_val) => Err(TypeError {
                message: String::from("different types cannot be used on the same operation"),
                location: location.clone(),
            }),
        }
    }

    pub fn gte(&self, value: &Value, location: &Location) -> Result<Value, TypeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(l_bool >= r_bool)),
            (Value::Str(l_str), Value::Str(r_str)) => Ok(Value::Bool(l_str >= r_str)),
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Bool(l_int >= r_int)),
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
            ) => Err(TypeError {
                message: String::from("closures cannot be compared"),
                location: location.clone(),
            }),
            (l_val, r_val) => Err(TypeError {
                message: String::from("different types cannot be used on the same operation"),
                location: location.clone(),
            }),
        }
    }

    pub fn and(&self, value: &Value, location: &Location) -> Result<Value, TypeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(*l_bool && *r_bool)),
            (l_val, r_val) => Err(TypeError {
                message: String::from(
                    "only booleans can be used on && (AND) short-circuit operation",
                ),
                location: location.clone(),
            }),
        }
    }

    pub fn or(&self, value: &Value, location: &Location) -> Result<Value, TypeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(*l_bool || *r_bool)),
            (l_val, r_val) => Err(TypeError {
                message: String::from(
                    "only booleans can be used on || (OR) short-circuit operation",
                ),
                location: location.clone(),
            }),
        }
    }

    pub fn add(&self, value: &Value, location: &Location) -> Result<Value, TypeError> {
        match (self, value) {
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Int(l_int + r_int)),
            (Value::Str(l_bool), Value::Str(r_bool)) => Err(TypeError {
                message: String::from("strings cannot be added"),
                location: location.clone(),
            }),
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Err(TypeError {
                message: String::from("booleans cannot be added"),
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
            ) => Err(TypeError {
                message: String::from("closures cannot be added"),
                location: location.clone(),
            }),
            (l_val, r_val) => Err(TypeError {
                message: String::from("different types cannot be used on the same operation"),
                location: location.clone(),
            }),
        }
    }

    pub fn sub(&self, value: &Value, location: &Location) -> Result<Value, TypeError> {
        match (self, value) {
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Int(l_int - r_int)),
            (Value::Str(l_bool), Value::Str(r_bool)) => Err(TypeError {
                message: String::from("strings cannot be subtracted"),
                location: location.clone(),
            }),
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Err(TypeError {
                message: String::from("booleans cannot be subtracted"),
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
            ) => Err(TypeError {
                message: String::from("closures cannot be subtracted"),
                location: location.clone(),
            }),
            (l_val, r_val) => Err(TypeError {
                message: String::from("different types cannot be used on the same operation"),
                location: location.clone(),
            }),
        }
    }

    pub fn mul(&self, value: &Value, location: &Location) -> Result<Value, TypeError> {
        match (self, value) {
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Int(l_int - r_int)),
            (Value::Str(l_bool), Value::Str(r_bool)) => Err(TypeError {
                message: String::from("strings cannot be multiplied"),
                location: location.clone(),
            }),
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Err(TypeError {
                message: String::from("booleans cannot be multiplied"),
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
            ) => Err(TypeError {
                message: String::from("closures cannot be multiplied"),
                location: location.clone(),
            }),
            (l_val, r_val) => Err(TypeError {
                message: String::from("different types cannot be used on the same operation"),
                location: location.clone(),
            }),
        }
    }

    pub fn div(&self, value: &Value, location: &Location) -> Result<Value, TypeError> {
        match (self, value) {
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Int(l_int / r_int)),
            (Value::Str(l_bool), Value::Str(r_bool)) => Err(TypeError {
                message: String::from("strings cannot be divided"),
                location: location.clone(),
            }),
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Err(TypeError {
                message: String::from("booleans cannot be divided"),
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
            ) => Err(TypeError {
                message: String::from("closures cannot be divided"),
                location: location.clone(),
            }),
            (l_val, r_val) => Err(TypeError {
                message: String::from("different types cannot be used on the same operation"),
                location: location.clone(),
            }),
        }
    }

    pub fn rem(&self, value: &Value, location: &Location) -> Result<Value, TypeError> {
        match (self, value) {
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Int(l_int / r_int)),
            (Value::Str(l_bool), Value::Str(r_bool)) => Err(TypeError {
                message: String::from("strings cannot be used with rem"),
                location: location.clone(),
            }),
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Err(TypeError {
                message: String::from("booleans cannot be used with rem"),
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
            ) => Err(TypeError {
                message: String::from("closures cannot be used with rem"),
                location: location.clone(),
            }),
            (l_val, r_val) => Err(TypeError {
                message: String::from("different types cannot be used on the same operation"),
                location: location.clone(),
            }),
        }
    }
}

pub fn eval(term: Term, context: &Context) -> Result<Value, TypeError> {
    match term {
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
                Value::Closure {
                    parameters: _,
                    body: _,
                    context: _,
                } => Err(TypeError {
                    message: String::from("closure cannot be used as if conditions"),
                    location: condition.location().clone(),
                }),
                Value::Int(_int) => Err(TypeError {
                    message: String::from("if cannot be used as if condition"),
                    location: condition.location().clone(),
                }),
                Value::Str(_str) => Err(TypeError {
                    message: String::from("string cannot be used as if condition"),
                    location: condition.location().clone(),
                }),
                Value::Unit => Err(TypeError {
                    message: String::from("unit cannot be used as if condition"),
                    location: condition.location().clone(),
                }),
            }?;

            match condition {
                true => eval(*then, context),
                false => eval(*otherwise, context),
            }
        }
        Term::Binary(Binary {
            left,
            op,
            right,
            location,
        }) => {
            let l_value = eval(*left, context)?;
            let r_value = eval(*right, context)?;

            match op {
                BinaryOperator::Eq => l_value.eq(&r_value, left.location()),
                BinaryOperator::Neq => l_value.neq(&r_value, left.location()),
                BinaryOperator::Lt => l_value.lt(&r_value, left.location()),
                BinaryOperator::Lte => l_value.lte(&r_value, left.location()),
                BinaryOperator::Gt => l_value.gt(&r_value, left.location()),
                BinaryOperator::Gte => l_value.gte(&r_value, left.location()),
                BinaryOperator::And => l_value.and(&r_value, left.location()),
                BinaryOperator::Or => l_value.or(&r_value, left.location()),
                BinaryOperator::Add => l_value.add(&r_value, left.location()),
                BinaryOperator::Sub => l_value.sub(&r_value, left.location()),
                BinaryOperator::Mul => l_value.mul(&r_value, left.location()),
                BinaryOperator::Div => l_value.div(&r_value, left.location()),
                BinaryOperator::Rem => l_value.rem(&r_value, left.location()),
            }
        }
        Term::Var(var) => {
            let var_value = context.get(&var.0.text).ok_or(TypeError {
                message: format!("unbound variable {}", &var.0.text),
                location: var.0.location,
            })?;

            Ok(var_value.clone())
        }
        Term::Int(int) => Ok(Value::Int(int.value)),
        Term::Str(str) => Ok(Value::Str(str.0.text)),
        Term::Print(print) => {
            let print_value = eval(*print.value, context)?;
            println!("{}", print_value);

            Ok(Value::Unit)
        }
    }
}
