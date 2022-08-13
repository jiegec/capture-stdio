use crate::Capture;
use os_pipe::{PipeReader, PipeWriter};
use std::{
    io::Error,
    os::{unix::io::RawFd, unix::prelude::AsRawFd},
};

use super::swap_fd;

pub struct PipedStdin {
    _reader: PipeReader,
    writer: PipeWriter,
    original: RawFd,
    restored: bool,
}

impl Capture for PipedStdin {
    fn capture() -> Result<Self, Error> {
        let (reader, writer) = os_pipe::pipe()?;
        let original = swap_fd(reader.as_raw_fd(), 0 as RawFd);
        Ok(Self {
            _reader: reader,
            writer,
            original,
            restored: false,
        })
    }

    fn restore(&mut self) {
        assert!(!self.restored, "You can't restore it twice");

        swap_fd(self.original, 0 as RawFd);
        self.restored = true;
    }
}

impl PipedStdin {
    pub fn get_writer(&mut self) -> &mut PipeWriter {
        &mut self.writer
    }
}

impl Drop for PipedStdin {
    fn drop(&mut self) {
        if !self.restored {
            self.restore();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pipe::stdin::PipedStdin;
    use crate::Capture;
    use std::io::Write;

    #[test]
    fn test_stdin() {
        let mut piped_stdin = PipedStdin::capture().unwrap();
        let string = "Write something to stdin\n";
        write!(piped_stdin.get_writer(), "{}", string).unwrap();

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        assert_eq!(string, line);
    }
}
