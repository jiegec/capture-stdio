//! This crate provide some helper functions to capture the stdin/out/err of the current process to help mocking.
//!
//! In testing, `cargo test` capture the stdout/err via `std::io::set_output_capture`. We can use the same mechanism to intercept [print!] and [eprint!] output. The crate wraps the call in [OutputCapture] so that you can intercept the output easily.
//!
//! The crate also implements a `pipe`-then-`dup` method to intercept stdio. In detail, it creates a pipe and replaces the fd of stdio. You can use [PipedStdin] to intercept stdin, use [PipedStdout] for stdout and [PipedStderr] for stderr.
#![feature(internal_output_capture)]

use std::io::Error;

pub mod output_capture;
pub mod pipe;

pub use output_capture::*;
pub use pipe::*;

/// Common trait to capture stdio and then restore
///
/// You should use [Capture::capture] to begin intercept and restore it via [Capture::restore] or [Drop].
pub trait Capture: Sized {
    /// Capture stdio
    fn capture() -> Result<Self, Error>;

    /// Restore stdio
    fn restore(&mut self);
}
