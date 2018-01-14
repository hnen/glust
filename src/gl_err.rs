extern crate gl;
use std::error::Error;
use std::option::NoneError;
use std::fmt;
use std;

#[derive(Debug)]
pub struct GlError(String);

pub type Result<T> = std::result::Result<T, GlError>;

pub fn validate_gl() -> Result<()> {
    if let Some(err) = get_error() {
        Err(GlError::new(err))
    } else {
        Ok(())
    }
}

fn get_error() -> Option<String> {
    let err = unsafe { gl::GetError() };
    if err != gl::NO_ERROR {
        Some(err.to_string())
    } else {
        None
    }
}

impl fmt::Display for GlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &GlError(ref s) = self;
        write!(f, "{}", s)
    }
}

impl Error for GlError {
    fn description(&self) -> &str {
        let &GlError(ref s) = self;
        s.as_str()
    }
}

impl GlError {
    pub fn new(st : String) -> GlError {
        GlError(st)
    }
}


impl From<NoneError> for GlError {
    fn from(err : NoneError) -> GlError {
        GlError(format!("{:?}", err))
    }
}

impl From<std::string::FromUtf8Error> for GlError {
    fn from(err : std::string::FromUtf8Error) -> GlError {
        GlError(err.description().to_string())
    }
}

impl From<std::ffi::NulError> for GlError {
    fn from(err : std::ffi::NulError) -> GlError {
        GlError(err.description().to_string())
    }
}







