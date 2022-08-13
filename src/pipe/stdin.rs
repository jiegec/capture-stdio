use super::PipedFd;
use crate::Capture;
use os_pipe::PipeWriter;
use std::io::Error;

pub struct PipedStdin {
    internal: PipedFd,
}

impl Capture for PipedStdin {
    fn capture() -> Result<Self, Error> {
        let internal = PipedFd::capture(0, true)?;
        Ok(Self { internal })
    }

    fn restore(&mut self) {
        self.internal.restore();
    }
}

impl PipedStdin {
    pub fn get_writer(&mut self) -> &mut PipeWriter {
        &mut self.internal.writer
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
