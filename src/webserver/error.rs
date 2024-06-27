use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct TemplateError {
    pub message: String
}

impl Error for TemplateError {}

impl Display for TemplateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}