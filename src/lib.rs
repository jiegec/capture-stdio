#![feature(internal_output_capture)]

use std::io::Error;

pub mod output_capture;
pub mod pipe;

pub trait Capture: Sized {
    fn capture() -> Result<Self, Error>;

    fn restore(&mut self);
}
