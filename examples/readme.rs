use hringas::{prep, Fd, IoUring, ReadBufHeap};
use std::{fs, io};

fn main() -> io::Result<()> {
    let mut ring = IoUring::new(8).expect("kernel out of date or wrong params");

    let fd: Fd = fs::File::open("README.md")?.into();
    let buf = ReadBufHeap::<1024>::new();

    let req = prep::read(0x42, &fd, &buf, 0);
    ring.enqueue(req).expect("submission queue is full");

    ring.submit_and_wait(1)?;

    let cqe = ring.copy_cqe().expect("completion queue is empty");

    assert_eq!(cqe.user_data(), 0x42);
    assert!(cqe.res >= 0, "read error: {}", cqe.res);

    Ok(())
}
