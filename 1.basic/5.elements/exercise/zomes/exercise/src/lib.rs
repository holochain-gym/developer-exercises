use hdk::prelude::*;

entry_defs![SnackingLog::entry_def()];

#[hdk_entry(id = "SnackingLog")]
pub struct SnackingLog(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HeaderAndEntryHash{
    entry_hash: EntryHash,
    header_hash: HeaderHash,
}

#[hdk_extern]
pub fn register_snacking(input: SnackingLog) -> ExternResult<HeaderAndEntryHash> {
    unimplemented!()
}

#[hdk_extern]
pub fn get_by_header_hash(header_hash: HeaderHash) -> ExternResult<SnackingLog> {
    unimplemented!()
}

#[hdk_extern]
pub fn get_by_entry_hash(entry_hash: EntryHash) -> ExternResult<SnackingLog> {
    unimplemented!()
}

#[hdk_extern]
pub fn get_all_headers_from_content(input: SnackingLog) -> ExternResult<Vec<SignedHeaderHashed>> {
        unimplemented!()
}
