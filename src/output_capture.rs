use std::{
    io::{set_output_capture, Error},
    sync::{Arc, Mutex},
};

use crate::Capture;

pub struct OutputCapture {
    local_stream: Arc<Mutex<Vec<u8>>>,
    original: Option<Arc<Mutex<Vec<u8>>>>,
    restored: bool,
}

impl Capture for OutputCapture {
    fn capture() -> Result<Self, Error> {
        let local_stream = Arc::new(Mutex::new(vec![]));
        let original = set_output_capture(Some(local_stream.clone()));
        Ok(Self {
            local_stream,
            original,
            restored: false,
        })
    }

    fn restore(&mut self) {
        assert!(!self.restored, "You can't restore it twice");

        set_output_capture(self.original.clone());
        self.restored = true;
    }
}

impl OutputCapture {
    pub fn get_output(&self) -> Arc<Mutex<Vec<u8>>> {
        self.local_stream.clone()
    }
}

impl Drop for OutputCapture {
    fn drop(&mut self) {
        if !self.restored {
            self.restore();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{output_capture::OutputCapture, Capture};

    #[test]
    fn test_output_capture() {
        println!("This should not be captured");

        let mut output_capture = OutputCapture::capture().unwrap();
        println!("This should be captured");
        output_capture.restore();

        println!("This should not be captured");

        let output =
            String::from_utf8(output_capture.get_output().lock().unwrap().clone()).unwrap();

        assert_eq!(output, "This should be captured\n");
    }
}
