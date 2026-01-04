use std::fmt::{Debug, Display};

// -----------------------------------
#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format the content first
        let content = format!("({}, {})", self.x, self.y);
        // Use pad() to handle width, alignment, fill
        f.pad(&content)
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

// -----------------------------------
#[derive(Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Check if alternate/pretty mode was requested ({:#?})
        if f.alternate() {
            write!(
                f,
                "Color {{\n    r: {},\n    g: {},\n    b: {},\n    hex: \"{}\"\n}}",
                self.r,
                self.g,
                self.b,
                self // self uses Display
            )
        } else {
            // Compact mode ({:?})
            write!(f, "Color {{ r: {}, g: {}, b: {} }}", self.r, self.g, self.b)
        }
    }
}

// -----------------------------------
pub struct Measurement {
    pub value: f64,
    pub unit: &'static str,
}

impl Measurement {
    pub fn new(value: f64, unit: &'static str) -> Self {
        Measurement { value, unit }
    }
}

impl Display for Measurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Respect precision if specified
        if let Some(precision) = f.precision() {
            write!(f, "{:.prec$} {}", self.value, self.unit, prec = precision)
        } else {
            write!(f, "{} {}", self.value, self.unit)
        }
    }
}

impl Debug for Measurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Measurement")
            .field("value", &self.value)
            .field("unit", &self.unit)
            .finish()
    }
}

// -----------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_display() {
        let p = Point::new(3.0, 4.0);
        assert_eq!(format!("{}", p), "(3, 4)");

        let p2 = Point::new(-1.5, 2.5);
        assert_eq!(format!("{}", p2), "(-1.5, 2.5)");
    }

    #[test]
    fn test_color_display() {
        let red = Color::new(255, 0, 0);
        assert_eq!(format!("{}", red), "#FF0000");

        let white = Color::new(255, 255, 255);
        assert_eq!(format!("{}", white), "#FFFFFF");

        let dark = Color::new(16, 32, 48);
        assert_eq!(format!("{}", dark), "#102030");
    }

    #[test]
    fn test_to_string() {
        // Display gives us ToString for free!
        let p = Point::new(1.0, 2.0);
        let s: String = p.to_string();
        assert_eq!(s, "(1, 2)");
    }
    #[test]
    fn test_point_debug() {
        let p = Point::new(3.0, 4.0);

        // Compact debug
        assert_eq!(format!("{:?}", p), "Point { x: 3.0, y: 4.0 }");
    }

    #[test]
    fn test_point_debug_pretty() {
        let p = Point::new(3.0, 4.0);

        // Pretty debug (uses debug_struct formatting)
        let pretty = format!("{:#?}", p);
        assert!(pretty.contains("Point"));
        assert!(pretty.contains("x:"));
        assert!(pretty.contains("y:"));
    }

    #[test]
    fn test_color_debug() {
        let c = Color::new(255, 128, 0);

        // Compact
        assert_eq!(format!("{:?}", c), "Color { r: 255, g: 128, b: 0 }");
    }

    #[test]
    fn test_color_debug_alternate() {
        let c = Color::new(255, 128, 0);
        println!("\n{:#?}\n", c);

        // Pretty mode includes hex
        let pretty = format!("{:#?}", c);
        assert!(pretty.contains("#FF8000"));
    }

    #[test]
    fn test_measurement_display() {
        #[allow(clippy::approx_constant)]
        let m = Measurement::new(3.14159, "m");

        // Default
        assert_eq!(format!("{}", m), "3.14159 m");

        // With precision
        assert_eq!(format!("{:.2}", m), "3.14 m");
        assert_eq!(format!("{:.0}", m), "3 m");
    }

    #[test]
    fn test_formatter_width_alignment() {
        let p = Point::new(1.0, 2.0);

        // Width (right-aligned by default)
        assert_eq!(format!("{:>15}", p), "         (1, 2)");

        // Left-aligned
        assert_eq!(format!("{:<15}", p), "(1, 2)         ");

        // Center-aligned
        assert_eq!(format!("{:^15}", p), "    (1, 2)     ");

        // Fill character
        assert_eq!(format!("{:->15}", p), "---------(1, 2)");
    }
}

// -----------------------------------
