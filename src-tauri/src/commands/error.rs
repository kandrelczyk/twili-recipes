use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

use crate::ai::AIError;
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

impl From<RecipesError> for CommandError {
    fn from(value: RecipesError) -> Self {
        CommandError {
            reason: format!("Failed perform recipes operation: {:?}", value),
        }
    }
}

impl From<tauri_plugin_store::Error> for RecipesError {
    fn from(value: tauri_plugin_store::Error) -> Self {
        RecipesError {
            reason: format!("Failed to store recipe: {:?}", value),
        }
    }
}

impl From<AIError> for CommandError {
    fn from(value: AIError) -> Self {
        CommandError {
            reason: format!("Failed perform AI operation: {:?}", value),
        }
    }
}

impl From<tauri_plugin_store::Error> for CommandError {
    fn from(value: tauri_plugin_store::Error) -> Self {
        CommandError {
            reason: format!("Failed to store config: {:?}", value),
        }
    }
}

impl Error for CommandError {}
