use std::os::unix::io::RawFd;

pub mod stdin;

pub fn swap_fd(fd: RawFd, target: RawFd) -> RawFd {
    unsafe {
        let orig_stdin = libc::dup(target as i32);
        libc::close(target as i32);
        libc::dup2(fd as i32, target as i32);
        orig_stdin as RawFd
    }
}
