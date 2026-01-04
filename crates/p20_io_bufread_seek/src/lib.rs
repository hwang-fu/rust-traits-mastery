use std::{
    i64,
    io::{self, BufRead, Read, Seek, SeekFrom},
    str::FromStr,
};

// ------------------------------------------------
pub struct MemBuffer {
    data: Vec<u8>,
    position: usize,
}

impl MemBuffer {
    pub fn new(data: Vec<u8>) -> Self {
        MemBuffer { data, position: 0 }
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

impl FromStr for MemBuffer {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MemBuffer::new(s.as_bytes().to_vec()))
    }
}

impl Read for MemBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let rem = &self.data[self.position..];
        let to_read = buf.len().min(rem.len());

        buf[..to_read].copy_from_slice(&rem[..to_read]);
        self.position += to_read;

        Ok(to_read)
    }
}

impl BufRead for MemBuffer {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        // Returns buffered data without consuming
        Ok(&self.data[self.position..])
    }

    fn consume(&mut self, amount: usize) {
        // Marks `amount` bytes as read
        self.position = (self.position + amount).min(self.data.len());
    }
}

impl Seek for MemBuffer {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        let new_pos = match pos {
            SeekFrom::Start(n) => n as i64,
            SeekFrom::Current(n) => self.position as i64 + n,
            SeekFrom::End(n) => self.data.len() as i64 + n,
        };

        if new_pos < 0 {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "seek to negative position",
            ))?;
        }

        self.position = (new_pos as usize).min(self.data.len());
        Ok(self.position as u64)
    }
}

// ------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_basic() {
        let mut buf = MemBuffer::from_str("hello world").unwrap();
        let mut output = [0u8; 5];

        let n = buf.read(&mut output).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&output, b"hello");
        assert_eq!(buf.position(), 5);
    }

    #[test]
    fn test_fill_buf_and_consume() {
        let mut buf = MemBuffer::from_str("hello").unwrap();

        // fill_buf returns remaining data without consuming
        let data = buf.fill_buf().unwrap();
        assert_eq!(data, b"hello");
        assert_eq!(buf.position(), 0); // Position unchanged

        // consume advances position
        buf.consume(2);
        assert_eq!(buf.position(), 2);

        let data = buf.fill_buf().unwrap();
        assert_eq!(data, b"llo");
    }

    #[test]
    fn test_read_line() {
        let mut buf = MemBuffer::from_str("line1\nline2\nline3").unwrap();
        let mut line = String::new();

        buf.read_line(&mut line).unwrap();
        assert_eq!(line, "line1\n");

        line.clear();
        buf.read_line(&mut line).unwrap();
        assert_eq!(line, "line2\n");
    }

    #[test]
    fn test_lines_iterator() {
        let buf = MemBuffer::from_str("one\ntwo\nthree").unwrap();
        let lines: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();

        assert_eq!(lines, vec!["one", "two", "three"]);
    }

    #[test]
    fn test_seek_from_start() {
        let mut buf = MemBuffer::from_str("hello world").unwrap();

        buf.seek(SeekFrom::Start(6)).unwrap();
        assert_eq!(buf.position(), 6);

        let mut output = [0u8; 5];
        let _ = buf.read(&mut output).unwrap();
        assert_eq!(&output, b"world");
    }

    #[test]
    fn test_seek_from_current() {
        let mut buf = MemBuffer::from_str("hello world").unwrap();

        let _ = buf.read(&mut [0u8; 5]).unwrap(); // Read "hello"
        assert_eq!(buf.position(), 5);

        buf.seek(SeekFrom::Current(1)).unwrap(); // Skip space
        assert_eq!(buf.position(), 6);

        buf.seek(SeekFrom::Current(-3)).unwrap(); // Go back
        assert_eq!(buf.position(), 3);
    }

    #[test]
    fn test_seek_from_end() {
        let mut buf = MemBuffer::from_str("hello world").unwrap();

        buf.seek(SeekFrom::End(-5)).unwrap(); // 5 bytes from end
        assert_eq!(buf.position(), 6);

        let mut output = [0u8; 5];
        let _ = buf.read(&mut output).unwrap();
        assert_eq!(&output, b"world");
    }

    #[test]
    fn test_seek_negative_error() {
        let mut buf = MemBuffer::from_str("hello").unwrap();

        let result = buf.seek(SeekFrom::Current(-10));
        assert!(result.is_err());
    }

    #[test]
    fn test_rewind() {
        let mut buf = MemBuffer::from_str("hello").unwrap();

        let _ = buf.read(&mut [0u8; 3]).unwrap();
        assert_eq!(buf.position(), 3);

        buf.rewind().unwrap(); // Provided method
        assert_eq!(buf.position(), 0);
    }
}
