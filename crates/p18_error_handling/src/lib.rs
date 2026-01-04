use std::{
    error::Error,
    fmt::{Display, Formatter},
    fs, io,
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl From<ConfigError> for AppError {
    fn from(err: ConfigError) -> Self {
        AppError::Config(err)
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

// -----------------------------------------------

pub fn load_config(path: &str) -> std::result::Result<String, AppError> {
    let content = fs::read_to_string(path)?;

    if content.is_empty() {
        // Return our own error
        Err(ConfigError::MissingField {
            field: "content".to_string(),
        })?;
        // ? converts ConfigError to AppError
    }
    Ok(content)
}

pub fn parse_port(s: &str) -> std::result::Result<u16, AppError> {
    let port: i32 = s.parse().map_err(|e| AppError::Parse {
        context: format!("invalid port: '{}'", s),
        source: e,
    })?;

    if port < 1 || port > 65535 {
        return Err(AppError::Config(ConfigError::OutOfRange {
            field: "port".to_string(),
            value: port,
            min: 1,
            max: 65535,
        }));
    }

    Ok(port as u16)
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

    #[test]
    fn test_error_source_chain() {
        let inner = ConfigError::MissingField {
            field: "database".to_string(),
        };
        let outer = AppError::Config(inner);

        // Display shows our message
        assert!(outer.to_string().contains("configuration error"));

        // source() returns the underlying error
        let source = outer.source().unwrap();
        assert!(source.to_string().contains("missing required field"));
    }

    #[test]
    fn test_parse_error_chain() {
        let parse_err = "not_a_number".parse::<i32>().unwrap_err();
        let app_err = AppError::Parse {
            context: "parsing port number".to_string(),
            source: parse_err,
        };

        assert!(app_err.to_string().contains("parse error"));
        assert!(app_err.source().is_some());
    }

    #[test]
    fn test_walk_error_chain() {
        let inner = ConfigError::MissingField {
            field: "port".to_string(),
        };
        let outer = AppError::Config(inner);

        // Walk the error chain
        let mut current: Option<&(dyn Error + 'static)> = Some(&outer);
        let mut messages = Vec::new();

        while let Some(err) = current {
            messages.push(err.to_string());
            current = err.source();
        }

        assert_eq!(messages.len(), 2);
        assert!(messages[0].contains("configuration error"));
        assert!(messages[1].contains("missing required field"));
    }

    #[test]
    fn test_from_io_error() {
        let result = load_config("/nonexistent/path/config.txt");
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(matches!(err, AppError::Io(_)));
        assert!(err.to_string().contains("IO error"));
    }

    #[test]
    fn test_from_config_error() {
        let config_err = ConfigError::MissingField {
            field: "test".to_string(),
        };
        let app_err: AppError = config_err.into();

        assert!(matches!(app_err, AppError::Config(_)));
    }

    #[test]
    fn test_parse_port_valid() {
        assert_eq!(parse_port("8080").unwrap(), 8080);
        assert_eq!(parse_port("443").unwrap(), 443);
    }

    #[test]
    fn test_parse_port_invalid() {
        // Not a number
        let err = parse_port("abc").unwrap_err();
        assert!(matches!(err, AppError::Parse { .. }));

        // Out of range
        let err = parse_port("70000").unwrap_err();
        assert!(matches!(
            err,
            AppError::Config(ConfigError::OutOfRange { .. })
        ));
    }
}

// -----------------------------------------------
