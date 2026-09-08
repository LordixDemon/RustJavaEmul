use alloc::{collections::BTreeMap, string::String, vec::Vec};
use std::io::Write;

use java_runtime::RuntimeStore;

use super::RuntimeImpl;

#[async_trait::async_trait]
impl<T> RuntimeStore for RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    fn rms_open_record_store(&self, name: &str, create_if_necessary: bool) -> bool {
        let mut stores = self.rms_stores.lock();
        if stores.contains_key(name) {
            return true;
        }
        if !create_if_necessary {
            return false;
        }

        stores.insert(name.to_owned(), Vec::new());
        true
    }

    fn rms_num_records(&self, name: &str) -> i32 {
        self.rms_stores
            .lock()
            .get(name)
            .map(|records| records.iter().filter(|record| record.is_some()).count() as i32)
            .unwrap_or(0)
    }

    fn rms_next_record_id(&self, name: &str) -> i32 {
        self.rms_stores.lock().get(name).map(|records| records.len() as i32 + 1).unwrap_or(1)
    }

    fn rms_add_record(&self, name: &str, data: &[i8]) -> i32 {
        let mut stores = self.rms_stores.lock();
        let records = stores.entry(name.to_owned()).or_default();
        records.push(Some(data.to_vec()));
        records.len() as i32
    }

    fn rms_delete_record(&self, name: &str, record_id: i32) {
        if record_id <= 0 {
            return;
        }

        if let Some(records) = self.rms_stores.lock().get_mut(name) {
            if let Some(record) = records.get_mut(record_id as usize - 1) {
                *record = None;
            }
        }
    }

    fn rms_get_record(&self, name: &str, record_id: i32) -> Option<Vec<i8>> {
        if record_id <= 0 {
            return None;
        }

        self.rms_stores
            .lock()
            .get(name)
            .and_then(|records| records.get(record_id as usize - 1))
            .and_then(Clone::clone)
    }

    fn rms_set_record(&self, name: &str, record_id: i32, data: &[i8]) {
        if record_id <= 0 {
            return;
        }

        let mut stores = self.rms_stores.lock();
        let records = stores.entry(name.to_owned()).or_default();
        while records.len() < record_id as usize {
            records.push(None);
        }
        records[record_id as usize - 1] = Some(data.to_vec());
    }

    fn rms_list_record_stores(&self) -> Vec<String> {
        self.rms_stores.lock().keys().cloned().collect()
    }

    fn rms_get_size(&self, name: &str) -> i32 {
        self.rms_stores
            .lock()
            .get(name)
            .map(|records| records.iter().flatten().map(|record| record.len() as i32).sum())
            .unwrap_or(0)
    }

    fn rms_get_size_available(&self, _name: &str) -> i32 {
        64 * 1024
    }
}

pub(crate) fn initial_rms_stores() -> BTreeMap<String, Vec<Option<Vec<i8>>>> {
    let mut stores = BTreeMap::new();
    let Some(presets) = crate::config::get().rms_preset.clone() else {
        return stores;
    };

    for entry in presets.split(';').map(str::trim).filter(|entry| !entry.is_empty()) {
        let Some((name, records)) = entry.split_once('=').or_else(|| entry.split_once(':')) else {
            continue;
        };
        let name = name.trim();
        if name.is_empty() {
            continue;
        }

        let records = records
            .split(',')
            .map(str::trim)
            .filter(|record| !record.is_empty())
            .filter_map(parse_hex_record)
            .map(Some)
            .collect::<Vec<_>>();
        if !records.is_empty() {
            stores.insert(name.to_owned(), records);
        }
    }

    stores
}

fn parse_hex_record(record: &str) -> Option<Vec<i8>> {
    let hex = record
        .chars()
        .filter(|ch| !ch.is_ascii_whitespace() && *ch != '_' && *ch != '-')
        .collect::<String>();
    if hex.is_empty() || hex.len() % 2 != 0 {
        return None;
    }

    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for offset in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[offset..offset + 2], 16).ok()?;
        bytes.push(byte as i8);
    }
    Some(bytes)
}
