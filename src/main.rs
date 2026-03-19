use bytes::{Buf, BytesMut};
fn main() {
    println!("Hello, world!");

    let mut buf: BytesMut = BytesMut::from("0123456789");
    buf.advance(4); // 456789
    let s = String::from_utf8_lossy(&buf[0..=4]); // 截取 "123"
    let size: Result<usize, _> = s.parse();
    println!("size: {:?}", size);
}
