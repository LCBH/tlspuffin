use std::io::{Read, Write};
use std::io;

#[derive(Debug)]
pub struct MemoryStream {
    incoming: io::Cursor<Vec<u8>>,
    outgoing: Vec<u8>,
}

impl MemoryStream {
    pub fn new() -> Self {
        Self {
            incoming: io::Cursor::new(Vec::new()),
            outgoing: Vec::new(),
        }
    }

    pub fn extend_incoming(&mut self, data: &[u8]) {
        self.incoming.get_mut().extend_from_slice(data);
    }

    pub fn take_outgoing(&mut self) -> Outgoing<'_> {
        Outgoing(&mut self.outgoing)
    }
}

impl Read for MemoryStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.incoming.read(buf)?;
        if self.incoming.position() == self.incoming.get_ref().len() as u64 {
            self.incoming.set_position(0);
            self.incoming.get_mut().clear();
        }
        if n == 0 {
            return Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "no data available",
            ));
        }
        Ok(n)
    }
}

impl Write for MemoryStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.outgoing.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct Outgoing<'a>(&'a mut Vec<u8>);

impl<'a> Drop for Outgoing<'a> {
    fn drop(&mut self) {
        self.0.clear();
    }
}

impl<'a> ::std::ops::Deref for Outgoing<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl<'a> AsRef<[u8]> for Outgoing<'a> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}