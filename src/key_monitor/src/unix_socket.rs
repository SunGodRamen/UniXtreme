use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::error::Error;

pub struct UnixSocket {
    path: String,
    listener: UnixListener,
}

impl UnixSocket {
    pub fn bind<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let path = path.as_ref().to_str().ok_or("Invalid path")?.to_owned();
        if let Err(_) = std::fs::remove_file(&path) {}
        let listener = UnixListener::bind(&path)?;
        Ok(UnixSocket { path, listener })
    }

    pub fn accept(&self) -> Result<UnixStream, Box<dyn Error>> {
        let (stream, _addr) = self.listener.accept()?;
        Ok(stream)
    }

    pub fn connect(path: &str) -> Result<UnixStream, Box<dyn Error>> {
        let stream = UnixStream::connect(path)?;
        Ok(stream)
    }

    pub fn receive_message(&self) -> Result<String, Box<dyn Error>> {
        let mut stream = self.accept()?;
        let mut buf = [0u8; 1024];
        let size = stream.read(&mut buf)?;
        let message = std::str::from_utf8(&buf[..size])?.to_owned();
        Ok(message)
    }

    pub fn send_message(&self, message: &str) -> Result<(), Box<dyn Error>> {
        let mut stream = Self::connect(&self.path)?;
        stream.write_all(message.as_bytes())?;
        Ok(())
    }
}

