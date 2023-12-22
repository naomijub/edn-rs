use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::fmt::{self, Debug, Display};

pub struct Error {
    code: Code,
    // line: usize,   // TODO walker
    // column: usize, // TODO walker
}

#[non_exhaustive]
pub enum Code {
    /// Catchall/placeholder error messages
    Message(Box<str>),
    Io(std::io::Error),
    TryFromInt(std::num::TryFromIntError),
    #[doc(hidden)]
    Infallable(), // Makes the compiler happy for converting u64 to u64 and i64 to i64
}

impl Error {
    pub(crate) fn ParseEdn(owned_string: String) -> Self {
        // TODO remove, just so things can compile with less changes
        Self {
            code: Code::Message(owned_string.into_boxed_str()),
        }
    }
    pub(crate) fn Deserialize(owned_string: String) -> Self {
        // TODO remove, just so things can compile with less changes
        Self {
            code: Code::Message(owned_string.into_boxed_str()),
        }
    }
    pub(crate) fn Iter(owned_string: String) -> Self {
        // TODO remove, just so things can compile with less changes
        Self {
            code: Code::Message(owned_string.into_boxed_str()),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.code {
            Code::Message(m) => write!(f, "{}", m.as_ref()),
            _ => todo!(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.code {
            Code::Message(m) => write!(f, "{}", m.as_ref()),
            Code::TryFromInt(e) => write!(f, "{}", e),
            _ => todo!(),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(s: std::num::ParseIntError) -> Self {
        Self::ParseEdn(s.to_string())
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(s: std::num::ParseFloatError) -> Self {
        Self::ParseEdn(s.to_string())
    }
}

impl From<std::str::ParseBoolError> for Error {
    fn from(s: std::str::ParseBoolError) -> Self {
        Self::ParseEdn(s.to_string())
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(e: std::num::TryFromIntError) -> Self {
        Self {
            code: Code::TryFromInt(e),
        }
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(_: std::convert::Infallible) -> Self {
        Self {
            code: Code::Infallable(),
        }
    }
}
