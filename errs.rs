use std::fmt::{Debug, Display};

/// 字符串异常
#[derive(Debug, Clone)]
pub struct SError(String);

impl SError {
    pub fn to_sresult<T>(&self) -> SResult<T> {
        new_err(&self.0)
    }
}

impl From<std::io::Error> for SError {
    fn from(e: std::io::Error) -> Self {
        Self(e.to_string())
    }
}

impl From<&str> for SError {
    fn from(e: &str) -> Self {
        Self(e.to_string())
    }
}

impl From<&String> for SError {
    fn from(e: &String) -> Self {
        Self(e.clone())
    }
}

impl From<String> for SError {
    fn from(e: String) -> Self {
        Self(e)
    }
}

impl From<Vec<u8>> for SError {
    fn from(e: Vec<u8>) -> Self {
        Self(String::from_utf8_lossy(&e).to_string())
    }
}

impl ToString for SError {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

pub type SResult<T> = std::result::Result<T, SError>;

pub fn new_err<T, U: ToString>(u: &U) -> SResult<T> {
    Err(SError(u.to_string()))
}

pub fn to_err<E: Into<SErrs>, T>(e: E) -> SResult<T> {
    e.into().to_sresult()
}

pub fn new<T>(t: T) -> SResult<T> {
    Ok(t)
}

pub fn sresult_to_string<T: std::fmt::Debug>(res: SResult<T>) -> String {
    if res.is_err() {
        res.unwrap_err().to_string()
    } else {
        String::default()
    }
}

#[derive(Debug)]
pub enum SErrs {
    IoError(std::io::Error),
    FromU8Error(std::string::FromUtf8Error),
    SError(SError),
    NullError,
}

impl SErrs {
    pub fn to_sresult<T>(self) -> SResult<T> {
        match &self {
            SErrs::IoError(e) => new_err(e),
            SErrs::FromU8Error(e) => new_err(e),
            SErrs::SError(e) => new_err(&e.0),
            SErrs::NullError => new_err::<T, &str>(&""),
        }
    }
}

impl From<std::io::Error> for SErrs {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<SError> for SErrs {
    fn from(e: SError) -> Self {
        Self::SError(e)
    }
}

impl<T: Debug> From<std::io::Result<T>> for SErrs {
    fn from(r: std::io::Result<T>) -> Self {
        Self::from(r.unwrap_err())
    }
}

impl From<&str> for SErrs {
    fn from(s: &str) -> Self {
        Self::SError(SError::from(s))
    }
}

impl From<&String> for SErrs {
    fn from(s: &String) -> Self {
        Self::SError(SError::from(s))
    }
}

impl From<String> for SErrs {
    fn from(s: String) -> Self {
        Self::SError(SError::from(s))
    }
}

impl From<std::string::FromUtf8Error> for SErrs {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::FromU8Error(e)
    }
}

impl Display for SErrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl Default for SErrs {
    fn default() -> Self {
        Self::NullError
    }
}
