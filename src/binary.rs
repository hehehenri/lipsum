use crate::{
    ast::{Binary, BinaryOp, Element, Location},
    interpreter::{RuntimeError, Value},
};

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

    pub fn binary_op(self, binary: Binary, rhs: Value) -> Result<Value, RuntimeError> {
        match binary.op {
            BinaryOp::Eq => self.eq(&rhs, binary.lhs.location()),
            BinaryOp::Neq => self.neq(&rhs, binary.lhs.location()),
            BinaryOp::Lt => self.lt(&rhs, binary.lhs.location()),
            BinaryOp::Lte => self.lte(&rhs, binary.lhs.location()),
            BinaryOp::Gt => self.gt(&rhs, binary.lhs.location()),
            BinaryOp::Gte => self.gte(&rhs, binary.lhs.location()),
            BinaryOp::And => self.and(&rhs, binary.lhs.location()),
            BinaryOp::Or => self.or(&rhs, binary.lhs.location()),
            BinaryOp::Add => self.add(&rhs, binary.lhs.location()),
            BinaryOp::Sub => self.sub(&rhs, binary.lhs.location()),
            BinaryOp::Mul => self.mul(&rhs, binary.lhs.location()),
            BinaryOp::Div => self.div(&rhs, binary.lhs.location()),
            BinaryOp::Rem => self.rem(&rhs, binary.lhs.location()),
        }
    }
}
