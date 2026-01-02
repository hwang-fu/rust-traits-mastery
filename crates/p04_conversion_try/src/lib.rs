use std::fmt;

// ---------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percentage(u8);

impl Percentage {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl TryFrom<i32> for Percentage {
    type Error = PercentageError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(PercentageError {
                invalid_value: value,
                message: format!("Percentage cannot be negative: {}", value),
            })
        } else if value > 100 {
            Err(PercentageError {
                invalid_value: value,
                message: format!("Percentage cannot exceed 100: {}", value),
            })
        } else {
            Ok(Percentage(value as u8))
        }
    }
}

impl TryFrom<u8> for Percentage {
    type Error = PercentageError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 100 {
            Err(PercentageError {
                invalid_value: value as i32,
                message: format!("Percentage cannot exceed 100: {}", value),
            })
        } else {
            Ok(Percentage(value))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PercentageError {
    pub invalid_value: i32,
    pub message: String,
}

impl fmt::Display for PercentageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// ---------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Port(u16);

impl Port {
    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn is_privileged(&self) -> bool {
        self.0 < 1024
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PortError(pub String);

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u16> for Port {
    type Error = PortError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PortError(String::from("Port 0 is reserved")))
        } else {
            Ok(Port(value))
        }
    }
}

impl TryFrom<i32> for Port {
    type Error = PortError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value <= 0 {
            Err(PortError(format!("Port must be positive: {}", value)))
        } else if value > 65535 {
            Err(PortError(format!("Port exceeds max 65535: {}", value)))
        } else {
            Ok(Port(value as u16))
        }
    }
}

// ---------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_valid_i32() {
        let p1 = Percentage::try_from(0).unwrap();
        let p2 = Percentage::try_from(50).unwrap();
        let p3 = Percentage::try_from(100).unwrap();

        assert_eq!(p1.value(), 0);
        assert_eq!(p2.value(), 50);
        assert_eq!(p3.value(), 100);
    }

    #[test]
    fn test_try_from_invalid_negative() {
        let p = Percentage::try_from(-1_i32);
        assert!(p.is_err());

        let e = p.unwrap_err();
        assert_eq!(e.invalid_value, -1);
        assert!(e.message.contains("negative"));
    }

    #[test]
    fn test_try_from_invalid_too_large() {
        let p = Percentage::try_from(101);
        assert!(p.is_err());

        let e: PercentageError = p.unwrap_err();
        assert_eq!(e.invalid_value, 101);
        assert!(e.message.contains("exceed"));
    }

    #[test]
    fn test_try_from_u8() {
        let p = Percentage::try_from(75u8).unwrap();
        assert_eq!(p.value(), 75);

        let p = Percentage::try_from(150u8);
        assert!(p.is_err());
    }

    #[test]
    fn test_try_into_syntax() {
        let n = 42;
        let p: Result<Percentage, _> = n.try_into();

        assert!(p.is_ok());
        assert_eq!(p.unwrap().value(), 42);
    }

    #[test]
    fn test_try_into_with_question_mark() {
        fn calc_discount(n: i32) -> Result<Percentage, PercentageError> {
            let p: Percentage = n.try_into()?;
            Ok(p)
        }

        assert!(calc_discount(24).is_ok());
        assert!(calc_discount(111000).is_err());
    }

    #[test]
    fn test_port_valid() {
        let p = Port::try_from(80_i32).unwrap();
        assert!(p.is_privileged());
        assert_eq!(p.value(), 80);

        let p = Port::try_from(8080_u16).unwrap();
        assert!(!p.is_privileged());
        assert_eq!(p.value(), 8080);
    }

    #[test]
    fn test_port_invalid() {
        // Port 0 is reserved
        assert!(Port::try_from(0_u16).is_err());

        // Negative ports invalid
        assert!(Port::try_from(-1_i32).is_err());

        // Over 65535 invalid
        assert!(Port::try_from(70000_i32).is_err());
    }

    #[test]
    fn test_try_into_flexible_function() {
        fn apply_discount<T>(value: T) -> Result<Percentage, PercentageError>
        where
            T: TryInto<Percentage, Error = PercentageError>,
        {
            value.try_into()
        }

        assert_eq!(apply_discount(20).unwrap().value(), 20);
        assert_eq!(apply_discount(50).unwrap().value(), 50);
        assert!(apply_discount(2000).is_err());
    }

    #[test]
    fn test_port_try_into() {
        let port: Result<Port, _> = 8080.try_into();
        assert!(port.is_ok());
        assert_eq!(port.unwrap().value(), 8080);

        let port: Result<Port, _> = (-120).try_into();
        assert!(port.is_err());
    }

    #[test]
    fn test_chained_try_into() {
        fn create_server_config(port: i32, discount: i32) -> Result<(Port, Percentage), String> {
            let port: Port = port.try_into().map_err(|e: PortError| e.0)?;
            let percentage: Percentage = discount
                .try_into()
                .map_err(|e: PercentageError| e.message)?;
            Ok((port, percentage))
        }

        let conf = create_server_config(8080, 15);
        assert!(conf.is_ok());

        let (port, discount) = conf.unwrap();
        assert_eq!(port.value(), 8080);
        assert_eq!(discount.value(), 15);

        // Invalid port fails
        assert!(create_server_config(0, 15).is_err());

        // Invalid discount fails
        assert!(create_server_config(8080, 150).is_err());
    }
}
