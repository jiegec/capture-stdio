#![feature(internal_output_capture)]

use std::io::Error;

pub mod output_capture;
pub mod pipe;

pub trait Capture: Sized {
    fn capture() -> Result<Self, Error>;

    fn restore(&mut self);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
