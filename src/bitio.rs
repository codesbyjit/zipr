use std::io;

/// Write bits efficiently
pub struct BitWriter {
    buffer: Vec<u8>,
    current: u8,
    filled: u8,
}

impl BitWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            current: 0,
            filled: 0,
        }
    }

    pub fn write_bits(&mut self, mut value: u32, mut bits: u8) {
        while bits > 0 {
            let space = 8 - self.filled;
            let to_write = bits.min(space);
            let mask = (1 << to_write) - 1;
            let shifted = (value & mask) as u8;
            self.current |= shifted << self.filled;

            self.filled += to_write;
            bits -= to_write;
            value >>= to_write;

            if self.filled == 8 {
                self.buffer.push(self.current);
                self.current = 0;
                self.filled = 0;
            }
        }
    }

    pub fn flush(&mut self) {
        if self.filled > 0 {
            self.buffer.push(self.current);
            self.current = 0;
            self.filled = 0;
        }
    }

    pub fn into_bytes(mut self) -> Vec<u8> {
        self.flush();
        self.buffer
    }
}

/// Read bits efficiently
pub struct BitReader<'a> {
    data: &'a [u8],
    index: usize,
    current: u8,
    filled: u8,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            index: 0,
            current: 0,
            filled: 0,
        }
    }

    pub fn read_bits(&mut self, mut n: u8) -> io::Result<u32> {
        let mut value = 0;
        let mut shift = 0;

        while n > 0 {
            if self.filled == 0 {
                if self.index >= self.data.len() {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "no more bits"));
                }
                self.current = self.data[self.index];
                self.index += 1;
                self.filled = 8;
            }

            let take = n.min(self.filled);
            let bits = self.current & ((1 << take) - 1);
            value |= (bits as u32) << shift;

            self.current >>= take;
            self.filled -= take;
            n -= take;
            shift += take as u32;
        }

        Ok(value)
    }
}

// Demo usage (keeps code “used” and ensures no warnings)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_io_roundtrip() {
        let mut bw = BitWriter::new();
        bw.write_bits(5, 3); // write 101
        bw.write_bits(2, 2); // write 10
        let data = bw.into_bytes();

        let mut br = BitReader::new(&data);
        assert_eq!(br.read_bits(3).unwrap(), 5);
        assert_eq!(br.read_bits(2).unwrap(), 2);
    }
}
