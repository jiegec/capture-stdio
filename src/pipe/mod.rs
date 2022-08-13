//! Intercept stdio via `pipe`-then-`dup` method
//!
use os_pipe::{PipeReader, PipeWriter};
use std::{
    io::Error,
    os::{unix::io::RawFd, unix::prelude::AsRawFd},
};

pub mod stderr;
pub mod stdin;
pub mod stdout;

pub use stderr::PipedStderr;
pub use stdin::PipedStdin;
pub use stdout::PipedStdout;

struct PipedFd {
    reader: PipeReader,
    writer: PipeWriter,
    original: RawFd,
    target: RawFd,
    restored: bool,
}

impl PipedFd {
    fn capture(target: RawFd, is_stdin: bool) -> Result<Self, Error> {
        let (reader, writer) = os_pipe::pipe()?;
        let original = if is_stdin {
            swap_fd(reader.as_raw_fd(), target)
        } else {
            swap_fd(writer.as_raw_fd(), target)
        };

        Ok(Self {
            reader,
            writer,
            original,
            target,
            restored: false,
        })
    }

    fn restore(&mut self) {
        assert!(!self.restored, "You can't restore it twice");

        let fd = swap_fd(self.original, self.target);
        unsafe {
            libc::close(fd);
        }

        self.restored = true;
    }
}

impl Drop for PipedFd {
    fn drop(&mut self) {
        if !self.restored {
            self.restore();
        }
    }
}

fn swap_fd(fd: RawFd, target: RawFd) -> RawFd {
    unsafe {
        let orig_stdin = libc::dup(target as i32);
        libc::close(target as i32);
        libc::dup2(fd as i32, target as i32);
        orig_stdin as RawFd
    }
}
