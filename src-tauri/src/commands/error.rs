use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

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
impl Error for CommandError {}
