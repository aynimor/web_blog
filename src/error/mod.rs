use std::{error::Error as StdError, fmt::{Display, self}};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Default Error
    E(u64, String),
}


pub trait CustomFrom<T>: Sized {
    /// Performs the conversion.
    fn from_res(_: u64, _: T) -> Self;
}


impl Display for Error {
    // IntellijRust does not understand that [non_exhaustive] applies only for downstream crates
    // noinspection RsMatchCheck
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::E(code, error) => write!(f, "{} {}", code, error),
        }
    }
}

impl StdError for Error {}

impl CustomFrom<String> for Error {
    fn from_res(code: u64, arg: String) -> Self {
        Error::E(code, arg)
    }
}

impl CustomFrom<&str> for Error {
    fn from_res(code: u64, arg: &str) -> Self {
        Error::E(code, arg.to_string())
    }
}

impl CustomFrom<&dyn std::error::Error> for Error {
    fn from_res(code: u64, arg: &dyn std::error::Error) -> Self {
        return Error::E(code, arg.to_string());
    }
}

impl From<Error> for std::io::Error {
    fn from(arg: Error) -> Self {
        arg.into()
    }
}

impl CustomFrom<rbatis::core::Error> for Error {
    fn from_res(code: u64, arg: rbatis::core::Error) -> Self {
        Error::E(code, arg.to_string())
    }
}

impl CustomFrom<actix_web::error::Error> for Error {
    fn from_res(code: u64, arg: actix_web::error::Error) -> Self {
        Error::E(code, arg.to_string())
    }
}
