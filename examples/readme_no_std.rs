#![no_std]
use hringas::prep;
use hringas::rustix::fs::{openat, Mode, OFlags, CWD};
use hringas::{Fd, IoUring, ReadBufStack};

fn main() {
    let mut ring = IoUring::new(8).unwrap();

    let fd: Fd =
        openat(CWD, "README.md", OFlags::RDONLY, Mode::empty()).unwrap().into();

    let buf = ReadBufStack::<1024>::new();

    let req = prep::read(0x42, &fd, &buf, 0);
    ring.enqueue(req).expect("submission queue is full");

    ring.submit_and_wait(1).unwrap();
    let cqe = ring.copy_cqe().expect("completion queue is empty");

    assert_eq!(cqe.user_data.u64_(), 0x42);
    assert!(cqe.res >= 0, "read error: {}", cqe.res);
}
