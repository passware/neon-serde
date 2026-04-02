//! Defines error handling types used by the create
//! uses the `error-chain` create for generation

use neon;
use neon::prelude::Context;
use neon::result::{NeonResult, Throw};
use serde::{de, ser};
use std::convert::From;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    /// nodejs has a hard coded limit on string length
    /// trying to serialize a string that is too long will result in an error
    StringTooLong {
        len: usize,
    },

    /// when deserializing to a boolean `false` `undefined` `null` `number`
    /// are valid inputs
    /// any other types will result in error
    UnableToCoerce {
        to_type: &'static str,
    },

    /// occurs when deserializing a char from an empty string
    EmptyString,

    /// occurs when deserializing a char from a sting with
    /// more than one character
    StringTooLongForChar {
        len: usize,
    },

    /// occurs when a deserializer expects a `null` or `undefined`
    /// property and found another type
    ExpectingNull,

    /// occurs when deserializing to an enum and the source object has
    /// a none-1 number of properties
    InvalidKeyType {
        key: String,
    },

    /// an internal deserialization error from an invalid array
    ArrayIndexOutOfBounds {
        index: u32,
        length: u32,
    },

    /// This type of object is not supported
    NotImplemented {
        name: &'static str,
    },

    /// A JS exception was thrown
    Js {
        throw: Throw,
    },

    SerializationError {
        message: String,
    },

    DeserializationError {
        message: String,
    },

    // failed to convert something to f64
    CastError,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::StringTooLong { len } => write!(f, "String too long for nodejs len: {}", len),
            Error::UnableToCoerce { to_type } => {
                write!(f, "Unable to coerce value to type: {}", to_type)
            }
            Error::EmptyString => write!(f, "Empty string"),
            Error::StringTooLongForChar { len } => write!(
                f,
                "String too long to be a char expected len: 1 got len: {}",
                len
            ),
            Error::ExpectingNull => write!(f, "ExpectingNull"),
            Error::InvalidKeyType { key } => write!(f, "Invalid key type '{}'", key),
            Error::ArrayIndexOutOfBounds { index, length } => write!(
                f,
                "ArrayIndexOutOfBounds: attempt to access ({}) size: ({})",
                index, length
            ),
            Error::NotImplemented { name } => write!(f, "Not Implemented: '{}'", name),
            Error::Js { throw: _ } => write!(f, "JS exception"),
            Error::SerializationError { message } => write!(f, "Serialization error: {}", message),
            Error::DeserializationError { message } => {
                write!(f, "Deserialization error: {}", message)
            }
            Error::CastError => write!(f, "CastError"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::SerializationError {
            message: msg.to_string(),
        }
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::DeserializationError {
            message: msg.to_string(),
        }
    }
}

impl From<neon::result::Throw> for Error {
    fn from(throw: neon::result::Throw) -> Self {
        Error::Js { throw }
    }
}

impl Error {
    pub fn into_throw<'cx, C: Context<'cx>>(self, cx: &mut C) -> Throw {
        match self {
            Error::Js { throw } => throw,
            _ => cx
                .throw_error::<String, ()>(self.to_string())
                .err()
                .unwrap(),
        }
    }

    pub fn into_neon_result<'cx, T, C: Context<'cx>>(self, cx: &mut C) -> NeonResult<T> {
        Err(self.into_throw(cx))
    }
}
