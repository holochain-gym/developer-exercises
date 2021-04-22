use hdk::prelude::*;
use holo_hash::HeaderHashB64;
use holo_hash::EntryHashB64;

entry_defs![SnackingLog::entry_def()];

#[hdk_entry(id = "SnackingLog")]
pub struct SnackingLog(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HeaderAndEntryHash{
    entry_hash: EntryHashB64,
    header_hash: HeaderHashB64,
}

#[hdk_extern]
pub fn register_snacking(input: SnackingLog) -> ExternResult<HeaderAndEntryHash> {
    let a: HeaderHash = create_entry(&input)?;
    let b: EntryHash = hash_entry(&input)?;
    let result = HeaderAndEntryHash{
        header_hash: HeaderHashB64::from(a),
        entry_hash: EntryHashB64::from(b),
    };
    Ok(result)
}

#[hdk_extern]
pub fn get_by_header_hash(external_hash_b64: HeaderHashB64) -> ExternResult<SnackingLog> {
    let element: Element = get(HeaderHash::from(external_hash_b64), GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Could not find SnackingLog for header hash")))?;
    let option: Option<SnackingLog> = element.entry().to_app_option()?;
    let snack_log: SnackingLog = option.expect("Unexpected missing option");

    Ok(snack_log)
}

#[hdk_extern]
pub fn get_by_entry_hash(hash: EntryHash) -> ExternResult<SnackingLog> {
    let element: Element = get(hash.into_hash(), GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Could not find SnackingLog for header hash")))?;
    let option: Option<SnackingLog> = element.entry().to_app_option()?;
    let snack_log: SnackingLog = option.expect("Unexpected missing option");

    Ok(snack_log)
}

#[hdk_extern]
pub fn get_header_hash_by_content(input: SnackingLog) -> ExternResult<HeaderHashB64> {
    let hash: EntryHash = hash_entry(&input)?;
    let element: Element = get(EntryHash::from(hash), GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Could not find SnackingLog based on content")))?;
    let header_hash: HeaderHash = element.header_address().clone();

    Ok(HeaderHashB64::from(header_hash))
}
