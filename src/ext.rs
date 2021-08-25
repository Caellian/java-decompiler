use std::io;

pub trait ReadByteVecExt: io::Read {
    #[inline]
    fn read_byte_vec(&mut self, byte_count: usize) -> Result<Vec<u8>, std::io::Error> {
        let mut result = Vec::with_capacity(byte_count);
        unsafe {
            result.set_len(byte_count);
        }

        self.read_exact(result.as_mut_slice())?;
        Ok(result)
    }
}

impl <R: io::Read> ReadByteVecExt for R {}
