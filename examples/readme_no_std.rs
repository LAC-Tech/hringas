#![no_std]
use hringas::rustix::fs::{openat, Mode, OFlags, CWD};
use hringas::rustix::io;
use hringas::{prep, read_buf_stack, Fd, IoUring};

fn main() -> io::Result<()> {
    let mut ring = IoUring::new(8).expect("kernel out of date or wrong params");

    let fd: Fd =
        openat(CWD, "README.md", OFlags::RDONLY, Mode::empty())?.into();

    let buf = read_buf_stack::<1024>();

    let req = prep::read(0x42, &fd, &buf, 0);
    ring.enqueue(req).expect("submission queue is full");

    ring.submit_and_wait(1)?;
    let cqe = ring.copy_cqe().expect("completion queue is empty");

    assert_eq!(cqe.user_data(), 0x42);
    assert!(cqe.res >= 0, "read error: {}", cqe.res);

    Ok(())
}
