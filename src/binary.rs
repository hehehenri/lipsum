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
                message: String::from("invalid AND operation"),
                full_text: format!("only booleans can be used on short-circuit operations"),
                location: location.clone(),
            }),
        }
    }

    pub fn or(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => Ok(Value::Bool(*l_bool || *r_bool)),
            (_l_val, _r_val) => Err(RuntimeError {
                message: String::from("invalid OR operation"),
                full_text: format!("only booleans can be used on short-circuit operations"),
                location: location.clone(),
            }),
        }
    }

    pub fn add(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Int(l_int + r_int)),
            (Value::Str(l_str), Value::Str(r_str)) => Ok(Value::Str(format!("{l_str}{r_str}"))),
            (Value::Str(l_str), Value::Int(r_int)) => Ok(Value::Str(format!("{l_str}{r_int}"))),
            (Value::Int(l_int), Value::Str(r_str)) => Ok(Value::Str(format!("{l_int}{r_str}"))),
            (l_val, r_val) => Err(RuntimeError {
                message: String::from("invalid addition"),
                full_text: format!("{l_val} cannot be added to {r_val}",),
                location: location.clone(),
            }),
        }
    }

    pub fn sub(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Int(l_int - r_int)),
            (l_val, r_val) => Err(RuntimeError {
                message: String::from("invalid subtraction"),
                full_text: format!("{l_val} cannot be subtracted by {r_val}",),
                location: location.clone(),
            }),
        }
    }

    pub fn mul(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Int(l_int * r_int)),
            (l_val, r_val) => Err(RuntimeError {
                message: String::from("invalid multiplication"),
                full_text: format!("{l_val} cannot be multiplied by {r_val} ",),
                location: location.clone(),
            }),
        }
    }

    pub fn div(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Int(_l_int), Value::Int(0)) => Err(RuntimeError {
                message: String::from("division by zero"),
                full_text: String::from("zero cannot be divised"),
                location: location.clone(),
            }),
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Int(l_int / r_int)),
            (l_val, r_val) => Err(RuntimeError {
                message: String::from("invalid division"),
                full_text: format!("{l_val} cannot be divised by {r_val}",),
                location: location.clone(),
            }),
        }
    }

    pub fn rem(&self, value: &Value, location: &Location) -> Result<Value, RuntimeError> {
        match (self, value) {
            (Value::Int(_l_val), Value::Int(0)) => Err(RuntimeError {
                message: String::from("division by zero"),
                full_text: String::from("cannot get remainder from a zero division"),
                location: location.clone(),
            }),
            (Value::Int(l_int), Value::Int(r_int)) => Ok(Value::Int(l_int % r_int)),
            (l_val, r_val) => Err(RuntimeError {
                message: String::from("invalid remainder operation"),
                full_text: format!("cannot get remainder from {l_val} and {r_val} division"),
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

#[cfg(test)]
mod tests {
    use crate::{ast::Location, interpreter::Value};

    fn int(int: i64) -> Value {
        Value::Int(int)
    }

    fn str(str: &str) -> Value {
        Value::Str(str.to_string())
    }

    fn location() -> Location {
        Location {
            start: 0,
            end: 0,
            filename: "tests".to_string(),
        }
    }

    fn eq(l: &Value, r: &Value) -> bool {
        match l.eq(r, &location()).unwrap() {
            Value::Bool(bool) => bool,
            _ => panic!("fail to compare {l} and {r}"),
        }
    }

    #[test]
    fn add_int_int() {
        let three_add_five = int(3).add(&int(5), &location()).unwrap();
        assert!(eq(&three_add_five, &int(8)));
    }

    #[test]
    fn add_str_int() {
        let a_add_two = str("a").add(&int(2), &location()).unwrap();
        assert!(eq(&a_add_two, &str("a2")));
    }

    #[test]
    fn add_int_str() {
        let two_add_a = int(2).add(&str("a"), &location()).unwrap();
        assert!(eq(&two_add_a, &str("2a")));
    }

    #[test]
    fn add_str_str() {
        let a_add_a = str("a").add(&str("b"), &location()).unwrap();
        assert!(eq(&a_add_a, &str("ab")));
    }

    #[test]
    fn sub() {
        let zero_sub_one = int(0).sub(&int(1), &location()).unwrap();
        assert!(eq(&zero_sub_one, &int(-1)));
    }

    #[test]
    fn mul() {
        let two_mul_two = int(2).mul(&int(2), &location()).unwrap();
        assert!(eq(&two_mul_two, &int(4)));
    }

    #[test]
    fn div() {
        let three_div_two = int(3).div(&int(2), &location()).unwrap();
        assert!(eq(&three_div_two, &int(1)));
    }

    #[test]
    fn div_by_zero() {
        let is_err = int(1).div(&int(0), &location()).is_err();

        assert!(is_err);
    }

    #[test]
    fn rem() {
        let four_rem_two = int(4).rem(&int(2), &location()).unwrap();
        assert!(eq(&four_rem_two, &int(0)));
    }

    #[test]
    fn rem_with_zero() {
        let is_err = int(1).rem(&int(0), &location()).is_err();

        assert!(is_err);
    }

    #[test]
    fn eq_str() {
        let a_eq_a = str("a").eq(&str("a"), &location()).unwrap();
        assert!(eq(&a_eq_a, &Value::Bool(true)));
    }

    #[test]
    fn eq_int() {
        let two_eq_one_plus_one = int(2)
            .eq(&int(1).add(&int(1), &location()).unwrap(), &location())
            .unwrap();
        assert!(eq(&two_eq_one_plus_one, &Value::Bool(true)));
    }

    #[test]
    fn eq_bool() {
        let true_eq_true = Value::Bool(true)
            .eq(&Value::Bool(true), &location())
            .unwrap();
        assert!(eq(&true_eq_true, &Value::Bool(true)));
    }

    #[test]
    fn neq_str() {
        let a_neq_b = str("a").neq(&str("b"), &location()).unwrap();
        assert!(eq(&a_neq_b, &Value::Bool(true)));
    }

    #[test]
    fn neq_int() {
        let three_neq_one_plus_one = int(3)
            .neq(&int(1).add(&int(1), &location()).unwrap(), &location())
            .unwrap();
        assert!(eq(&three_neq_one_plus_one, &Value::Bool(true)));
    }

    #[test]
    fn neq_bool() {
        let true_neq_false = Value::Bool(true)
            .neq(&Value::Bool(false), &location())
            .unwrap();
        assert!(eq(&true_neq_false, &Value::Bool(true)));
    }

    #[test]
    fn lt() {
        let one_lt_two = int(1).lt(&int(2), &location()).unwrap();
        assert!(eq(&one_lt_two, &Value::Bool(true)));
    }

    #[test]
    fn gt() {
        let two_gt_three = int(2).gt(&int(3), &location()).unwrap();
        assert!(eq(&two_gt_three, &Value::Bool(false)));
    }

    #[test]
    fn lte() {
        let one_lte_two = int(1).lte(&int(2), &location()).unwrap();
        assert!(eq(&one_lte_two, &Value::Bool(true)));
    }

    #[test]
    fn gte() {
        let one_gte_two = int(1).gte(&int(2), &location()).unwrap();
        assert!(eq(&one_gte_two, &Value::Bool(false)));
    }

    #[test]
    fn and_bool() {
        let true_and_false = Value::Bool(true)
            .and(&Value::Bool(false), &location())
            .unwrap();
        assert!(eq(&true_and_false, &Value::Bool(false)));
    }

    #[test]
    fn or_bool() {
        let false_or_true = Value::Bool(false)
            .or(&Value::Bool(true), &location())
            .unwrap();
        assert!(eq(&false_or_true, &Value::Bool(true)));
    }
}
