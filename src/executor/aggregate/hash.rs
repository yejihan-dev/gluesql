use std::convert::{TryFrom, TryInto};

use crate::data::Value;
use crate::executor::evaluate::Evaluated;
use crate::result::{Error, Result};

#[derive(PartialEq, Eq, Hash, Clone, std::fmt::Debug)]
pub enum GroupKey {
    I64(i64),
    Bool(bool),
    Str(String),
    None,
}

impl TryFrom<&Evaluated<'_>> for GroupKey {
    type Error = Error;

    fn try_from(evaluated: &Evaluated<'_>) -> Result<Self> {
        match evaluated {
            Evaluated::Literal(l) => Value::try_from(l.as_ref())?.try_into(),
            Evaluated::Value(v) => v.as_ref().try_into(),
        }
    }
}
