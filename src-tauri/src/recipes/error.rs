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

impl From<serde_json::Error> for RecipesError {
    fn from(value: serde_json::Error) -> Self {
        RecipesError {
            reason: format!("Failed to serialize recipe: {:?}", value),
        }
    }
}

impl Error for RecipesError {}
