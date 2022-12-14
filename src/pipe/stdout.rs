//! Intercept [std::io::Stdout]

use super::PipedFd;
use crate::Capture;
use os_pipe::PipeReader;
use std::io::Error;

/// Intercept stdout
pub struct PipedStdout {
    internal: PipedFd,
}

impl Capture for PipedStdout {
    fn capture() -> Result<Self, Error> {
        let internal = PipedFd::capture(1, false)?;
        Ok(Self { internal })
    }

    fn restore(&mut self) {
        self.internal.restore();
    }
}

impl PipedStdout {
    /// Get reader of pipe
    pub fn get_reader(&mut self) -> &mut PipeReader {
        &mut self.internal.reader
    }
}

#[cfg(test)]
mod tests {
    use crate::pipe::stdout::PipedStdout;
    use crate::Capture;
    use std::io::{set_output_capture, BufRead, BufReader};

    #[test]
    fn test_stdout() {
        // stdout is captured by testing
        let original = set_output_capture(None);

        let mut piped_stdout = PipedStdout::capture().unwrap();
        let string = "Write something to stdout\n";
        print!("{}", string);

        set_output_capture(original);

        let mut output = String::new();
        let mut buf_reader = BufReader::new(piped_stdout.get_reader());
        buf_reader.read_line(&mut output).unwrap();

        assert_eq!(output, string);
        piped_stdout.restore();
    }
}
