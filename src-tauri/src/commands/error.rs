use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

use crate::recipes::error::RecipesError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandError {
    pub reason: String,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl From<ConfyError> for CommandError {
    fn from(value: ConfyError) -> Self {
        CommandError {
            reason: format!("Failed to read config: {:?}", value),
        }
    }
}

impl From<RecipesError> for CommandError {
    fn from(value: RecipesError) -> Self {
        CommandError {
            reason: format!("Failed perform recipes operation: {:?}", value),
        }
    }
}
impl Error for CommandError {}
