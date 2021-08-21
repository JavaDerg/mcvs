use bytes::{Buf, BufMut};
use std::io;

pub struct BufWrapper<B: Buf>(pub B);
pub struct BufMutWrapper<B: BufMut>(pub B);

impl<B: Buf> io::Read for BufWrapper<B> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let max = buf.len().min(self.0.remaining());
        self.0.copy_to_slice(&mut buf[..max]);
        Ok(max)
    }
}

impl<B: BufMut> io::Write for BufMutWrapper<B> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.put_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<B: BufMut + Buf> io::Read for BufMutWrapper<B> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        BufWrapper(&mut self.0).read(buf)
    }
}
