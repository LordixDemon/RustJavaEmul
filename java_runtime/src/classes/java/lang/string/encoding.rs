#[allow(unused_imports)]
use super::*;
use alloc::{
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};
#[allow(unused_imports)]
use core::cmp::Ordering;

impl String {
    pub(super) fn decode_str(charset: &str, bytes: &[u8]) -> RustString {
        match charset.to_ascii_uppercase().replace('_', "-").as_str() {
            "UTF-8" | "UTF8" => RustString::from_utf8_lossy(bytes).into_owned(),
            "EUC-KR" | "EUCKR" | "KS-C-5601-1987" | "MS949" | "CP949" => encoding_rs::EUC_KR.decode(bytes).0.to_string(),
            "ISO-8859-1" | "LATIN1" | "US-ASCII" | "ASCII" => bytes.iter().map(|&b| b as char).collect(),
            "UTF-16BE" | "UNICODEBIG" | "UNICODEBIGUNMARKED" => decode_utf16(bytes, true),
            "UTF-16LE" | "UNICODELITTLE" | "UNICODELITTLEUNMARKED" => decode_utf16(bytes, false),
            "UTF-16" | "UNICODE" => {
                if bytes.starts_with(&[0xfe, 0xff]) {
                    decode_utf16(&bytes[2..], true)
                } else if bytes.starts_with(&[0xff, 0xfe]) {
                    decode_utf16(&bytes[2..], false)
                } else {
                    decode_utf16(bytes, true)
                }
            }
            _ => bytes.iter().map(|&b| b as char).collect(),
        }
    }

    pub(super) fn encode_str(charset: &str, string: &str) -> Vec<u8> {
        match charset.to_ascii_uppercase().replace('_', "-").as_str() {
            "UTF-8" | "UTF8" => string.as_bytes().to_vec(),
            "EUC-KR" | "EUCKR" | "KS-C-5601-1987" | "MS949" | "CP949" => encoding_rs::EUC_KR.encode(string).0.to_vec(),
            "ISO-8859-1" | "LATIN1" => string.chars().map(|c| if (c as u32) <= 0xff { c as u8 } else { b'?' }).collect(),
            "US-ASCII" | "ASCII" => string.chars().map(|c| if c.is_ascii() { c as u8 } else { b'?' }).collect(),
            "UTF-16BE" | "UNICODEBIG" | "UNICODEBIGUNMARKED" => encode_utf16(string, true),
            "UTF-16LE" | "UNICODELITTLE" | "UNICODELITTLEUNMARKED" => encode_utf16(string, false),
            "UTF-16" | "UNICODE" => {
                let mut out = vec![0xfe, 0xff];
                out.extend(encode_utf16(string, true));
                out
            }
            _ => string.chars().map(|c| if (c as u32) <= 0xff { c as u8 } else { b'?' }).collect(),
        }
    }
}

fn decode_utf16(bytes: &[u8], big_endian: bool) -> RustString {
    let mut units = Vec::with_capacity(bytes.len() / 2);
    let mut chunks = bytes.chunks_exact(2);
    for chunk in chunks.by_ref() {
        units.push(if big_endian {
            u16::from_be_bytes([chunk[0], chunk[1]])
        } else {
            u16::from_le_bytes([chunk[0], chunk[1]])
        });
    }
    RustString::from_utf16_lossy(&units)
}

fn encode_utf16(string: &str, big_endian: bool) -> Vec<u8> {
    let mut out = Vec::with_capacity(string.len() * 2);
    for unit in string.encode_utf16() {
        let bytes = if big_endian { unit.to_be_bytes() } else { unit.to_le_bytes() };
        out.extend_from_slice(&bytes);
    }
    out
}
