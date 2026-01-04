use std::io::{self, Read, Write};

// -------------------------
pub struct ZeroReader;

impl Read for ZeroReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        for b in buf.iter_mut() {
            *b = 0;
        }
        Ok(buf.len())
    }
}

// -------------------------
pub struct RepReader {
    pattern: Vec<u8>,
    position: usize,
}

impl RepReader {
    pub fn new(pattern: &[u8]) -> Self {
        let pattern = pattern.to_vec();
        let position = 0;
        RepReader { pattern, position }
    }
}

impl Read for RepReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.pattern.is_empty() {
            return Ok(0);
        }

        for b in buf.iter_mut() {
            *b = self.pattern[self.position];
            self.position = (self.position + 1) % self.pattern.len();
        }

        Ok(buf.len())
    }
}

// -------------------------
pub struct CntReader<R> {
    inner: R,
    bytes_read: usize,
}

impl<R> CntReader<R> {
    pub fn new(inner: R) -> Self {
        let bytes_read = 0;
        CntReader { inner, bytes_read }
    }

    pub fn bytes_read(&self) -> usize {
        self.bytes_read
    }
}

impl<R> Read for CntReader<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.bytes_read += n;
        Ok(n)
    }
}

pub struct CntWriter<W> {
    inner: W,
    bytes_written: usize,
}

impl<W> CntWriter<W> {
    pub fn new(inner: W) -> Self {
        let bytes_written = 0;
        CntWriter {
            inner,
            bytes_written,
        }
    }

    pub fn bytes_written(&self) -> usize {
        self.bytes_written
    }

    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W> Write for CntWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.bytes_written += n;
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

// -------------------------
pub struct NullWriter;

impl Write for NullWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // Accept all bytes but discard them
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// -------------------------
pub struct LimitedWriter<W> {
    inner: W,
    limit: usize,
}

impl<W> LimitedWriter<W> {
    pub fn new(inner: W, limit: usize) -> Self {
        LimitedWriter { inner, limit }
    }

    pub fn limit(&self) -> usize {
        self.limit
    }
}

impl<W> Write for LimitedWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.limit == 0 {
            Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "write limit reached",
            ))?;
        }

        let to_write = buf.len().min(self.limit);
        let n = self.inner.write(&buf[..to_write])?;
        self.limit -= n;
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

// -------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_reader() {
        let mut reader = ZeroReader;
        let mut buf = [0xFFu8; 10];

        let n = reader.read(&mut buf).unwrap();
        assert_eq!(n, 10);
        assert!(buf.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_repeat_reader() {
        let mut reader = RepReader::new(b"abc");
        let mut buf = [0u8; 9];

        let _ = reader.read(&mut buf).unwrap();
        assert_eq!(&buf, b"abcabcabc");
    }

    #[test]
    fn test_counting_reader() {
        let data = b"hello world";
        let mut reader = CntReader::new(&data[..]);

        let mut buf = [0u8; 5];
        let _ = reader.read(&mut buf).unwrap();
        assert_eq!(reader.bytes_read(), 5);

        let _ = reader.read(&mut buf).unwrap();
        assert_eq!(reader.bytes_read(), 10);
    }

    #[test]
    fn test_read_to_string() {
        // Using provided method from Read trait
        let data = b"hello";
        let mut reader = CntReader::new(&data[..]);

        let mut s = String::new();
        reader.read_to_string(&mut s).unwrap();

        assert_eq!(s, "hello");
        assert_eq!(reader.bytes_read(), 5);
    }

    #[test]
    fn test_null_writer() {
        let mut writer = NullWriter;

        let n = writer.write(b"hello world").unwrap();
        assert_eq!(n, 11);

        // Flush always succeeds
        writer.flush().unwrap();
    }

    #[test]
    fn test_counting_writer() {
        let mut buffer = Vec::new();
        let mut writer = CntWriter::new(&mut buffer);

        writer.write_all(b"hello").unwrap();
        assert_eq!(writer.bytes_written(), 5);

        writer.write_all(b" world").unwrap();
        assert_eq!(writer.bytes_written(), 11);

        // Check the actual output
        assert_eq!(&buffer, b"hello world");
    }

    #[test]
    fn test_limited_writer() {
        let mut buffer = Vec::new();
        let mut writer = LimitedWriter::new(&mut buffer, 5);

        // Can write up to limit
        let n = writer.write(b"hello world").unwrap();
        assert_eq!(n, 5);
        assert_eq!(writer.limit(), 0);

        // Further writes fail
        let err = writer.write(b"!").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::WriteZero);

        assert_eq!(&buffer, b"hello");
    }

    #[test]
    fn test_write_fmt() {
        // Using the provided write! macro
        let mut buffer = Vec::new();
        write!(&mut buffer, "value: {}", 42).unwrap();

        assert_eq!(&buffer, b"value: 42");
    }
}
