use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

// -----------------------------------------------
#[derive(Debug)]
pub enum ConfigError {
    /// Missing required field
    MissingField { field: String },

    /// Invalid value for a field
    InvalidValue {
        field: String,
        value: String,
        reason: String,
    },

    /// Value out of allowed range
    OutOfRange {
        field: String,
        value: i32,
        min: i32,
        max: i32,
    },
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingField { field } => {
                write!(f, "missing required field: '{}'", field)
            }
            ConfigError::InvalidValue {
                field,
                value,
                reason,
            } => {
                write!(
                    f,
                    "invalid value '{}' for field '{}': {}",
                    value, field, reason
                )
            }
            ConfigError::OutOfRange {
                field,
                value,
                min,
                max,
            } => {
                write!(
                    f,
                    "value {} for '{}' out of range [{}, {}]",
                    value, field, min, max
                )
            }
        }
    }
}

impl Error for ConfigError {}

// -----------------------------------------------
#[derive(Debug)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        let field = field.into();
        let message = message.into();
        ValidationError { field, message }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "validation failed for '{}': {}",
            self.field, self.message
        )
    }
}

impl Error for ValidationError {}

// -----------------------------------------------
#[derive(Debug)]
pub enum AppError {
    /// Configuration error
    Config(ConfigError),
    /// IO error (wraps std::io::Error)
    Io(std::io::Error),
    /// Parse error with context
    Parse {
        context: String,
        source: std::num::ParseIntError,
    },
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(e) => write!(f, "configuration error: {}", e),
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Parse { context, .. } => write!(f, "parse error: {}", context),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Config(e) => Some(e),
            AppError::Io(e) => Some(e),
            AppError::Parse { source, .. } => Some(source),
        }
    }
}

// -----------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_error_display() {
        let err = ConfigError::MissingField {
            field: "username".to_string(),
        };
        assert_eq!(err.to_string(), "missing required field: 'username'");

        let err = ConfigError::OutOfRange {
            field: "port".to_string(),
            value: 70000,
            min: 1,
            max: 65535,
        };
        assert_eq!(
            err.to_string(),
            "value 70000 for 'port' out of range [1, 65535]"
        );
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::new("email", "must contain @");
        assert_eq!(
            err.to_string(),
            "validation failed for 'email': must contain @"
        );
    }

    #[test]
    fn test_error_is_debug() {
        let err = ConfigError::MissingField {
            field: "host".to_string(),
        };
        // Debug should work (required by Error trait)
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("MissingField"));
    }
}

// -----------------------------------------------
