use hdk::prelude::*;
use holo_hash::{EntryHashB64, HeaderHashB64};

entry_defs![SnackingLog::entry_def()];

#[hdk_entry(id = "SnackingLog")]
pub struct SnackingLog(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HeaderAndEntryHash {
    entry_hash: EntryHashB64,
    header_hash: HeaderHashB64,
}

#[hdk_extern]
pub fn register_snacking(input: SnackingLog) -> ExternResult<HeaderAndEntryHash> {
    unimplemented!()
}

#[hdk_extern]
pub fn get_by_header_hash(hash: HeaderHashB64) -> ExternResult<SnackingLog> {
    unimplemented!()
}

#[hdk_extern]
pub fn get_by_entry_hash(hash: EntryHashB64) -> ExternResult<SnackingLog> {
    unimplemented!()
}

#[hdk_extern]
pub fn get_header_hash_by_content(input: SnackingLog) -> ExternResult<HeaderHashB64> {
    unimplemented!()
}
