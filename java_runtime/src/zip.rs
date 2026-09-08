use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug)]
pub struct ZipEntryMeta {
    pub name: String,
    pub compression_method: u16,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub local_header_offset: u32,
}

impl ZipEntryMeta {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> u64 {
        self.uncompressed_size as u64
    }

    pub fn compressed_size(&self) -> u64 {
        self.compressed_size as u64
    }

    pub fn is_dir(&self) -> bool {
        self.name.ends_with('/')
    }
}

#[derive(Clone, Debug)]
pub struct ZipArchive<T = Vec<u8>> {
    data: T,
    entries: Vec<ZipEntryMeta>,
}

impl<T: AsRef<[u8]>> ZipArchive<T> {
    pub fn new(data: T) -> Result<Self, &'static str> {
        let entries = parse_central_directory(data.as_ref())?;
        Ok(Self { data, entries })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn by_index(&self, index: usize) -> Result<ZipFile<'_>, &'static str> {
        let meta = self.entries.get(index).ok_or("index out of bounds")?;
        Ok(ZipFile {
            archive: self.data.as_ref(),
            meta,
        })
    }

    pub fn by_name(&self, name: &str) -> Result<ZipFile<'_>, &'static str> {
        let meta = self.entries.iter().find(|e| e.name == name).ok_or("entry not found")?;
        Ok(ZipFile {
            archive: self.data.as_ref(),
            meta,
        })
    }

    pub fn entries(&self) -> &[ZipEntryMeta] {
        &self.entries
    }
}

pub struct ZipFile<'a> {
    archive: &'a [u8],
    meta: &'a ZipEntryMeta,
}

impl<'a> ZipFile<'a> {
    pub fn name(&self) -> Result<&str, &'static str> {
        Ok(&self.meta.name)
    }

    pub fn size(&self) -> u64 {
        self.meta.size()
    }

    pub fn compressed_size(&self) -> u64 {
        self.meta.compressed_size()
    }

    pub fn is_dir(&self) -> bool {
        self.meta.is_dir()
    }

    pub fn extract(&self) -> Result<Vec<u8>, &'static str> {
        extract_entry_data(self.archive, self.meta)
    }

    pub fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, &'static str> {
        let data = self.extract()?;
        let len = data.len();
        buf.extend_from_slice(&data);
        Ok(len)
    }
}

fn find_eocd(bytes: &[u8]) -> Result<usize, &'static str> {
    if bytes.len() < 22 {
        return Err("data too short for ZIP");
    }
    let max_search = (bytes.len() - 22).min(65535 + 22);
    let search_start = bytes.len() - 22;
    for offset in (bytes.len() - max_search..=search_start).rev() {
        if bytes[offset..offset + 4] == [0x50, 0x4b, 0x05, 0x06] {
            return Ok(offset);
        }
    }
    Err("EOCD signature not found")
}

fn parse_central_directory(bytes: &[u8]) -> Result<Vec<ZipEntryMeta>, &'static str> {
    let eocd_offset = find_eocd(bytes)?;
    let eocd = &bytes[eocd_offset..];
    let total_entries = u16::from_le_bytes([eocd[10], eocd[11]]) as usize;
    let cd_size = u32::from_le_bytes([eocd[12], eocd[13], eocd[14], eocd[15]]) as usize;
    let cd_offset = u32::from_le_bytes([eocd[16], eocd[17], eocd[18], eocd[19]]) as usize;

    if cd_offset + cd_size > bytes.len() {
        return Err("central directory out of bounds");
    }

    let mut entries = Vec::with_capacity(total_entries);
    let mut cursor = cd_offset;
    for _ in 0..total_entries {
        if cursor + 46 > bytes.len() {
            break;
        }
        if bytes[cursor..cursor + 4] != [0x50, 0x4b, 0x01, 0x02] {
            return Err("invalid central directory header signature");
        }
        let method = u16::from_le_bytes([bytes[cursor + 10], bytes[cursor + 11]]);
        let comp_size = u32::from_le_bytes([bytes[cursor + 20], bytes[cursor + 21], bytes[cursor + 22], bytes[cursor + 23]]);
        let uncomp_size = u32::from_le_bytes([bytes[cursor + 24], bytes[cursor + 25], bytes[cursor + 26], bytes[cursor + 27]]);
        let name_len = u16::from_le_bytes([bytes[cursor + 28], bytes[cursor + 29]]) as usize;
        let extra_len = u16::from_le_bytes([bytes[cursor + 30], bytes[cursor + 31]]) as usize;
        let comment_len = u16::from_le_bytes([bytes[cursor + 32], bytes[cursor + 33]]) as usize;
        let local_offset = u32::from_le_bytes([bytes[cursor + 42], bytes[cursor + 43], bytes[cursor + 44], bytes[cursor + 45]]);

        let name_start = cursor + 46;
        let name_end = name_start + name_len;
        if name_end > bytes.len() {
            return Err("entry name out of bounds");
        }

        let name = core::str::from_utf8(&bytes[name_start..name_end])
            .map(|s| s.into())
            .unwrap_or_else(|_| bytes[name_start..name_end].iter().map(|&b| b as char).collect());

        entries.push(ZipEntryMeta {
            name,
            compression_method: method,
            compressed_size: comp_size,
            uncompressed_size: uncomp_size,
            local_header_offset: local_offset,
        });

        cursor = name_end + extra_len + comment_len;
    }

    Ok(entries)
}

fn extract_entry_data(bytes: &[u8], meta: &ZipEntryMeta) -> Result<Vec<u8>, &'static str> {
    let lh_offset = meta.local_header_offset as usize;
    if lh_offset + 30 > bytes.len() {
        return Err("local header out of bounds");
    }
    if bytes[lh_offset..lh_offset + 4] != [0x50, 0x4b, 0x03, 0x04] {
        return Err("invalid local file header signature");
    }
    let name_len = u16::from_le_bytes([bytes[lh_offset + 26], bytes[lh_offset + 27]]) as usize;
    let extra_len = u16::from_le_bytes([bytes[lh_offset + 28], bytes[lh_offset + 29]]) as usize;
    let data_start = lh_offset + 30 + name_len + extra_len;
    let comp_size = meta.compressed_size as usize;
    let data_end = data_start + comp_size;

    if data_end > bytes.len() {
        return Err("compressed data out of bounds");
    }

    let raw_data = &bytes[data_start..data_end];

    match meta.compression_method {
        0 => Ok(raw_data.to_vec()),
        8 => miniz_oxide::inflate::decompress_to_vec(raw_data).map_err(|_| "failed to decompress deflate stream"),
        _ => Err("unsupported compression method in zip"),
    }
}

pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ 0xedb8_8320 } else { crc >> 1 };
        }
    }
    !crc
}

pub fn create_simple_zip(files: &[(&str, &[u8])]) -> Vec<u8> {
    create_zip(files, false)
}

pub fn create_deflated_zip(files: &[(&str, &[u8])]) -> Vec<u8> {
    create_zip(files, true)
}

fn create_zip(files: &[(&str, &[u8])], deflate: bool) -> Vec<u8> {
    let mut out = Vec::new();
    let mut central_directory = Vec::new();

    for (name, content) in files {
        let local_header_offset = out.len() as u32;
        let name_bytes = name.as_bytes();
        let name_len = name_bytes.len() as u16;
        let payload = if deflate {
            miniz_oxide::deflate::compress_to_vec(content, 6)
        } else {
            content.to_vec()
        };
        let method = if deflate { 8u16 } else { 0 };
        let content_len = content.len() as u32;
        let stored_len = payload.len() as u32;
        let crc = crc32(content);

        out.extend_from_slice(&[0x50, 0x4b, 0x03, 0x04]);
        out.extend_from_slice(&20u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&method.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&crc.to_le_bytes());
        out.extend_from_slice(&stored_len.to_le_bytes());
        out.extend_from_slice(&content_len.to_le_bytes());
        out.extend_from_slice(&name_len.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(name_bytes);
        out.extend_from_slice(&payload);

        central_directory.extend_from_slice(&[0x50, 0x4b, 0x01, 0x02]);
        central_directory.extend_from_slice(&20u16.to_le_bytes());
        central_directory.extend_from_slice(&20u16.to_le_bytes());
        central_directory.extend_from_slice(&0u16.to_le_bytes());
        central_directory.extend_from_slice(&method.to_le_bytes());
        central_directory.extend_from_slice(&0u32.to_le_bytes());
        central_directory.extend_from_slice(&crc.to_le_bytes());
        central_directory.extend_from_slice(&stored_len.to_le_bytes());
        central_directory.extend_from_slice(&content_len.to_le_bytes());
        central_directory.extend_from_slice(&name_len.to_le_bytes());
        central_directory.extend_from_slice(&0u16.to_le_bytes());
        central_directory.extend_from_slice(&0u16.to_le_bytes());
        central_directory.extend_from_slice(&0u16.to_le_bytes());
        central_directory.extend_from_slice(&0u16.to_le_bytes());
        central_directory.extend_from_slice(&0u32.to_le_bytes());
        central_directory.extend_from_slice(&local_header_offset.to_le_bytes());
        central_directory.extend_from_slice(name_bytes);
    }

    let cd_offset = out.len() as u32;
    let cd_size = central_directory.len() as u32;
    let total_entries = files.len() as u16;
    out.extend_from_slice(&central_directory);

    out.extend_from_slice(&[0x50, 0x4b, 0x05, 0x06]);
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&total_entries.to_le_bytes());
    out.extend_from_slice(&total_entries.to_le_bytes());
    out.extend_from_slice(&cd_size.to_le_bytes());
    out.extend_from_slice(&cd_offset.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());

    out
}

#[cfg(test)]
mod tests {
    use super::{ZipArchive, create_deflated_zip, create_simple_zip};

    #[test]
    fn stored_and_deflated_zip_roundtrip() {
        let payload = b"hello hello hello hello rustjava zip bench";
        let stored = create_simple_zip(&[("a.txt", payload)]);
        let deflated = create_deflated_zip(&[("a.txt", payload)]);
        assert_eq!(ZipArchive::new(stored).unwrap().by_name("a.txt").unwrap().extract().unwrap(), payload);
        assert_eq!(ZipArchive::new(deflated).unwrap().by_name("a.txt").unwrap().extract().unwrap(), payload);
    }
}
