use bytes::Bytes;

pub trait Encryption {
    fn encrypt(&mut self, input: Bytes) -> Bytes;
    fn decrypt(&mut self, input: Bytes) -> Bytes;
}

pub struct PassTrough;

impl Encryption for PassTrough {
    fn encrypt(&mut self, input: Bytes) -> Bytes {
        input
    }

    fn decrypt(&mut self, input: Bytes) -> Bytes {
        input
    }
}
