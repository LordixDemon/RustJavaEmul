pub(super) type ParseResult<T> = core::result::Result<T, &'static str>;

pub(super) struct ByteLoader<'a> {
    data: &'a [u8],
    pos: usize,
    cached: u32,
    cache: u32,
}

impl<'a> ByteLoader<'a> {
    pub(super) fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
            cached: 0,
            cache: 0,
        }
    }

    pub(super) fn skip(&mut self, count: usize) -> ParseResult<()> {
        if self.pos + count > self.data.len() {
            return Err("truncated data");
        }
        self.pos += count;
        Ok(())
    }

    pub(super) fn read_u8(&mut self) -> ParseResult<u8> {
        let Some(value) = self.data.get(self.pos).copied() else {
            return Err("truncated u8");
        };
        self.pos += 1;
        Ok(value)
    }

    pub(super) fn read_i8(&mut self) -> ParseResult<i8> {
        Ok(self.read_u8()? as i8)
    }

    pub(super) fn read_u16(&mut self) -> ParseResult<u16> {
        let lo = self.read_u8()? as u16;
        let hi = self.read_u8()? as u16;
        Ok(lo | (hi << 8))
    }

    pub(super) fn read_i16(&mut self) -> ParseResult<i16> {
        Ok(self.read_u16()? as i16)
    }

    pub(super) fn read_i32(&mut self) -> ParseResult<i32> {
        let lo = self.read_u16()? as u32;
        let hi = self.read_u16()? as u32;
        Ok((lo | (hi << 16)) as i32)
    }

    pub(super) fn read_ubits(&mut self, size: u32) -> ParseResult<i32> {
        if size > 25 {
            return Err("invalid bit width");
        }
        while size > self.cached {
            self.cache |= (self.read_u8()? as u32) << self.cached;
            self.cached += 8;
        }
        let mask = if size == 32 { u32::MAX } else { (1u32 << size) - 1 };
        let result = self.cache & mask;
        self.cached -= size;
        self.cache >>= size;
        Ok(result as i32)
    }

    pub(super) fn read_bits(&mut self, size: u32) -> ParseResult<i32> {
        let value = self.read_ubits(size)?;
        let shift = 32 - size;
        Ok((value << shift) >> shift)
    }

    pub(super) fn clear_cache(&mut self) {
        self.cached = 0;
        self.cache = 0;
    }
}
