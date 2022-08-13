use std::{
    io::set_output_capture,
    sync::{Arc, Mutex},
};

pub struct OutputCapture {
    local_stream: Arc<Mutex<Vec<u8>>>,
    original: Option<Arc<Mutex<Vec<u8>>>>,
    restored: bool,
}

impl OutputCapture {
    pub fn capture() -> Self {
        let local_stream = Arc::new(Mutex::new(vec![]));
        let original = set_output_capture(Some(local_stream.clone()));
        Self {
            local_stream,
            original,
            restored: false,
        }
    }

    pub fn get_output(&self) -> Arc<Mutex<Vec<u8>>> {
        self.local_stream.clone()
    }

    pub fn restore(&mut self) {
        assert!(!self.restored, "You can't restore it twice");

        set_output_capture(self.original.clone());
        self.restored = true;
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
    use crate::output_capture::OutputCapture;

    #[test]
    fn test_output_capture() {
        println!("This should not be captured");

        let mut output_capture = OutputCapture::capture();
        println!("This should be captured");
        output_capture.restore();

        println!("This should not be captured");

        let output =
            String::from_utf8(output_capture.get_output().lock().unwrap().clone()).unwrap();

        assert_eq!(output, "This should be captured\n");
    }
}
