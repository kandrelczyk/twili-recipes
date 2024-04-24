use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct RecipesError {
    pub reason: String,
}

impl fmt::Display for RecipesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl Error for RecipesError {}
