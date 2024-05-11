use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub struct AIError {
    pub reason: String,
}

impl Display for AIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl From<serde_json::Error> for AIError {
    fn from(value: serde_json::Error) -> Self {
        AIError {
            reason: format!(
                "Failed to de-serialize recipe. Invalid recipe received from LLM: {:?}",
                value
            ),
        }
    }
}

impl From<reqwest::Error> for AIError {
    fn from(value: reqwest::Error) -> Self {
        AIError {
            reason: format!("Invalid response received from LLM service: {:?}", value),
        }
    }
}

impl Error for AIError {}
