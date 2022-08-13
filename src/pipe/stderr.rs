//! Intercept [std::io::Stderr]

use super::PipedFd;
use crate::Capture;
use os_pipe::PipeReader;
use std::io::Error;

/// Intercept stderr
pub struct PipedStderr {
    internal: PipedFd,
}

impl Capture for PipedStderr {
    fn capture() -> Result<Self, Error> {
        let internal = PipedFd::capture(2, false)?;
        Ok(Self { internal })
    }

    fn restore(&mut self) {
        self.internal.restore();
    }
}

impl PipedStderr {
    /// Get reader of pipe
    pub fn get_reader(&mut self) -> &mut PipeReader {
        &mut self.internal.reader
    }
}

#[cfg(test)]
mod tests {
    use crate::pipe::stderr::PipedStderr;
    use crate::Capture;
    use std::io::{set_output_capture, BufRead, BufReader};

    #[test]
    fn test_stderr() {
        // stderr is captured by testing
        let original = set_output_capture(None);

        let mut piped_stderr = PipedStderr::capture().unwrap();
        let string = "Write something to stderr\n";
        eprint!("{}", string);

        set_output_capture(original);

        let mut output = String::new();
        let mut buf_reader = BufReader::new(piped_stderr.get_reader());
        buf_reader.read_line(&mut output).unwrap();

        assert_eq!(output, string);
        piped_stderr.restore();
    }
}
