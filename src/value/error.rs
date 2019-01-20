use super::Value;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    UnaryOp(Value, &'static str),
    BinaryOp(Value, Value, &'static str),
    Comparison(Value, Value),
    Indexing(Value, Value),
    IndexingMut(Value, Value),
    WrongType(Value, Value),
}

fn kind(v: &Value) -> &'static str {
    match v {
        Value::Void => "void",
        Value::Num(_) => "num",
        Value::Str(_) => "string",
        Value::Bool(_) => "bool",
        Value::Array(_) => "array",
        Value::Fn(_) => "function",
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::UnaryOp(val, op) => write!(
                f,
                "Cannot apply operator '{}' to value of type '{}'",
                op,
                kind(val)
            ),
            Error::BinaryOp(a, b, op) => write!(
                f,
                "Cannot apply operator '{}' to values of the given types ('{}' and '{}')",
                op,
                kind(a),
                kind(b)
            ),
            Error::Comparison(a, b) => write!(
                f,
                "Cannot compare values of the given types ('{}' and '{}')",
                kind(a),
                kind(b)
            ),
            Error::Indexing(a, b) => write!(
                f,
                "Cannot index value of type '{}' with value of type '{}'",
                kind(a),
                kind(b)
            ),
            Error::IndexingMut(a, b) => write!(
                f,
                "Cannot index mutably a value of type '{}' with value of type '{}'",
                kind(a),
                kind(b)
            ),
            Error::WrongType(a, b) => write!(
                f,
                "Type mismatch: expected '{}', got '{}'",
                kind(a),
                kind(b)
            ),
        }
    }
}
