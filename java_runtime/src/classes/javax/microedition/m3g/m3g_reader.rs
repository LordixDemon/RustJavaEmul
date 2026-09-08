#[allow(unused_imports)]
use super::common::*;
#[allow(unused_imports)]
use super::math::*;
#[allow(unused_imports)]
use super::prelude::*;
#[allow(unused_imports)]
use super::raw_arrays::*;
#[allow(unused_imports)]
use super::render::*;
#[allow(unused_imports)]
use super::types::*;

pub(super) struct M3gReader<'a> {
    data: &'a [u8],
    pos: usize,
    read_error: Box<dyn ClassInstance>,
}

impl<'a> M3gReader<'a> {
    pub(super) fn new(data: &'a [u8], read_error: Box<dyn ClassInstance>) -> Self {
        Self { data, pos: 0, read_error }
    }

    pub(super) fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    pub(super) fn read_slice(&mut self, len: usize) -> Result<&'a [u8]> {
        if self.pos + len > self.data.len() {
            return Err(JavaError::JavaException(self.read_error.clone()));
        }
        let start = self.pos;
        self.pos += len;
        Ok(&self.data[start..start + len])
    }

    pub(super) fn skip(&mut self, len: usize) -> Result<()> {
        self.read_slice(len).map(|_| ())
    }

    pub(super) fn read_u8(&mut self) -> Result<u8> {
        Ok(self.read_slice(1)?[0])
    }

    pub(super) fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    pub(super) fn read_byte_array(&mut self) -> Result<&'a [u8]> {
        let len = self.read_u32()? as usize;
        self.read_slice(len)
    }

    pub(super) fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_u8()? != 0)
    }

    pub(super) fn read_u16(&mut self) -> Result<u16> {
        let bytes = self.read_slice(2)?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    pub(super) fn read_i16(&mut self) -> Result<i16> {
        Ok(self.read_u16()? as i16)
    }

    pub(super) fn read_u32(&mut self) -> Result<u32> {
        let bytes = self.read_slice(4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub(super) fn read_i32(&mut self) -> Result<i32> {
        Ok(self.read_u32()? as i32)
    }

    pub(super) fn read_f32(&mut self) -> Result<f32> {
        Ok(f32::from_bits(self.read_u32()?))
    }

    pub(super) fn read_rgb(&mut self) -> Result<i32> {
        let r = self.read_u8()? as u32;
        let g = self.read_u8()? as u32;
        let b = self.read_u8()? as u32;
        Ok(((r << 16) | (g << 8) | b) as i32)
    }

    pub(super) fn read_argb(&mut self) -> Result<i32> {
        let r = self.read_u8()? as u32;
        let g = self.read_u8()? as u32;
        let b = self.read_u8()? as u32;
        let a = self.read_u8()? as u32;
        Ok(((a << 24) | (r << 16) | (g << 8) | b) as i32)
    }

    pub(super) fn read_string(&mut self) -> Result<RustString> {
        let start = self.pos;
        while self.pos < self.data.len() && self.data[self.pos] != 0 {
            self.pos += 1;
        }
        let bytes = &self.data[start..self.pos];
        if self.pos < self.data.len() {
            self.pos += 1;
        }
        Ok(core::str::from_utf8(bytes).unwrap_or("").to_string())
    }
}
