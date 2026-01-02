use std::{fmt, path::Path, str::FromStr};

// --------------------------------------

/// Counts words in any string-like type.
/// Accepts: &str, String, &String, Box<str>, etc.
pub fn count_words<S: AsRef<str>>(text: S) -> usize {
    text.as_ref().split_whitespace().count()
}

pub fn has_extension<P, S>(path: P, ext: S) -> bool
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    path.as_ref()
        .extension()
        .map(|extension| extension == ext.as_ref())
        .unwrap_or(false)
}

// --------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct ByteBuffer {
    data: Vec<u8>,
}

impl ByteBuffer {
    pub fn new(data: Vec<u8>) -> Self {
        ByteBuffer { data }
    }

    pub fn new_from_str(s: &str) -> Self {
        let data = s.as_bytes().to_vec();
        ByteBuffer { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// AsRef<Vec<u8>> - view as the inner Vec
impl AsRef<Vec<u8>> for ByteBuffer {
    fn as_ref(&self) -> &Vec<u8> {
        &self.data
    }
}

impl AsRef<[u8]> for ByteBuffer {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

// AsMut<[u8]> - mutable access to the byte slice
impl AsMut<[u8]> for ByteBuffer {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

pub fn checksum<T>(data: T) -> u32
where
    T: AsRef<[u8]>,
{
    data.as_ref().iter().map(|b| (*b) as u32).sum()
}

pub fn zero_out<T>(mut data: T)
where
    T: AsMut<[u8]>,
{
    for b in data.as_mut() {
        *b = 0
    }
}

// --------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseColorError {
    pub message: String,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    fn parse_hex(s: &str) -> Result<Self, ParseColorError> {
        if s.len() != 7 {
            return Err(ParseColorError {
                message: format!("Hex color must be 7 chars (#RRGGBB): {}", s),
            });
        }

        let r = u8::from_str_radix(&s[1..3], 16);
        let g = u8::from_str_radix(&s[3..5], 16);
        let b = u8::from_str_radix(&s[5..7], 16);

        match (r, g, b) {
            (Ok(r), Ok(g), Ok(b)) => Ok(Color { r, g, b }),
            _ => Err(ParseColorError {
                message: format!("Invalid hex color: {}", s),
            }),
        }
    }

    fn parse_csv(s: &str) -> Result<Self, ParseColorError> {
        let parts: Vec<&str> = s.split(',').collect();

        if parts.len() != 3 {
            return Err(ParseColorError {
                message: format!("Expected r,g,b format: {}", s),
            });
        }

        let r = parts[0].trim().parse::<u8>();
        let g = parts[1].trim().parse::<u8>();
        let b = parts[2].trim().parse::<u8>();

        match (r, g, b) {
            (Ok(r), Ok(g), Ok(b)) => Ok(Color { r, g, b }),
            _ => Err(ParseColorError {
                message: format!("Invalid RGB values: {}", s),
            }),
        }
    }
}

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.starts_with('#') {
            // Hex format: #RRGGBB
            Self::parse_hex(s)
        } else {
            // Comma format: r,g,b
            Self::parse_csv(s)
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

// --------------------------------------

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_count_words_str() {
        assert_eq!(count_words("hello world"), 2);
        assert_eq!(count_words("one two three four"), 4);
        assert_eq!(count_words(""), 0);
    }

    #[test]
    fn test_count_words_string() {
        let s = String::from("rust is awesome");
        assert_eq!(count_words(s), 3);

        let s2 = String::from("one two");
        assert_eq!(count_words(&s2), 2);
    }

    #[test]
    fn test_has_extension_str() {
        assert!(has_extension("a.txt", "txt"));
        assert!(has_extension("b.bin", String::from("bin")));
        assert!(!has_extension("keep", "txt"));
    }

    #[test]
    fn test_has_extension_pathbuf() {
        let path = PathBuf::from("/home/user/code.rs");
        assert!(has_extension(&path, "rs"));
        assert!(!has_extension(&path, "txt"));

        let path = PathBuf::from("/home/etc/serv.conf");
        assert!(has_extension(path, "conf"));
    }

    #[test]
    fn test_has_extension_mixed_types() {
        let path = PathBuf::from("document.pdf");
        let ext = String::from("pdf");

        assert!(has_extension(&path, &ext));
        assert!(has_extension("document.pdf", "pdf"));
        assert!(has_extension(path, ext));
    }

    #[test]
    fn test_byte_buffer_as_ref() {
        let buf = ByteBuffer::new_from_str("hello");

        let sum = checksum(&buf);
        assert_eq!(
            sum,
            b'h' as u32 + b'e' as u32 + b'l' as u32 + b'l' as u32 + b'o' as u32
        );
    }

    #[test]
    fn test_checksum_multiple_types() {
        let buf = ByteBuffer::new(vec![1, 2, 3]);
        assert_eq!(checksum(&buf), 6);

        let vec = vec![10_u8, 20_u8, 30_u8];
        assert_eq!(checksum(&vec), 60);

        let slice: &[u8] = &[5, 5, 5];
        assert_eq!(checksum(slice), 15);
    }

    #[test]
    fn test_as_mut_zero_out() {
        let mut buf = ByteBuffer::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(checksum(&buf), 15);

        zero_out(&mut buf);
        assert_eq!(checksum(&buf), 0);

        assert_eq!(buf.as_ref() as &[u8], &[0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_as_mut_vec() {
        let mut vec: Vec<u8> = vec![100, 200];
        zero_out(&mut vec);
        assert_eq!(vec, vec![0, 0]);
    }

    #[test]
    fn test_color_from_str_csv() {
        // Parse comma-separated format
        let color: Color = "255,128,0".parse().unwrap();
        assert_eq!(color, Color::new(255, 128, 0));

        // With spaces
        let color: Color = " 0, 255, 128 ".parse().unwrap();
        assert_eq!(color, Color::new(0, 255, 128));
    }

    #[test]
    fn test_color_from_str_hex() {
        // Parse hex format
        let color: Color = "#FF8000".parse().unwrap();
        assert_eq!(color, Color::new(255, 128, 0));

        let color: Color = "#000000".parse().unwrap();
        assert_eq!(color, Color::new(0, 0, 0));

        let color: Color = "#FFFFFF".parse().unwrap();
        assert_eq!(color, Color::new(255, 255, 255));
    }

    #[test]
    fn test_color_from_str_errors() {
        // Invalid format
        let result: Result<Color, _> = "invalid".parse();
        assert!(result.is_err());

        // Wrong number of values
        let result: Result<Color, _> = "1,2".parse();
        assert!(result.is_err());

        // Invalid hex
        let result: Result<Color, _> = "#GGGGGG".parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_color_to_string() {
        // Display (and thus ToString) outputs hex format
        let color = Color::new(255, 128, 0);
        assert_eq!(color.to_string(), "#FF8000");

        let color = Color::new(0, 0, 0);
        assert_eq!(color.to_string(), "#000000");
    }

    #[test]
    fn test_color_roundtrip() {
        // Parse -> Display -> Parse should give same result
        let original = Color::new(100, 150, 200);
        let as_string = original.to_string();
        let parsed: Color = as_string.parse().unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_parse_turbofish() {
        // Using turbofish syntax
        let color = "128,64,32".parse::<Color>().unwrap();
        assert_eq!(color.r, 128);

        // Using Color::from_str directly
        let color = Color::from_str("#AABBCC").unwrap();
        assert_eq!(color, Color::new(170, 187, 204));
    }
}
