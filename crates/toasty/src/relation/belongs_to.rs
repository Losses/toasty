use crate::Model;

use toasty_core::stmt::Value;

use std::fmt;

pub struct BelongsTo<T> {
    value: Option<Box<T>>,
}

impl<T: Model> BelongsTo<T> {
    pub fn load(input: Value) -> crate::Result<Self> {
        Ok(match input {
            Value::Null => Self::default(),
            Value::Record(record) => Self {
                value: Some(Box::new(T::load(record)?)),
            },
            _ => todo!(),
        })
    }

    #[track_caller]
    pub fn get(&self) -> &T {
        self.value.as_ref().expect("association not loaded")
    }
}

impl<T> Default for BelongsTo<T> {
    fn default() -> Self {
        Self { value: None }
    }
}

impl<T: fmt::Debug> fmt::Debug for BelongsTo<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value.as_ref() {
            Some(t) => t.fmt(fmt),
            None => {
                write!(fmt, "<not loaded>")?;
                Ok(())
            }
        }
    }
}
