use hdk::prelude::*;
use holo_hash::{HeaderHashB64, EntryHashB64};

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
    let result = HeaderAndEntryHash {
        header_hash: HeaderHashB64::from(a),
        entry_hash: EntryHashB64::from(b),
    };
    Ok(result)
}

#[hdk_extern]
pub fn get_by_header_hash(header_hash: HeaderHashB64) -> ExternResult<SnackingLog> {
    let element: Element = get(HeaderHash::from(header_hash), GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Could not find SnackingLog for header hash")))?;
    let option: Option<SnackingLog> = element.entry().to_app_option()?;
    let snack_log: SnackingLog = option
        .ok_or(WasmError::Guest(String::from("No book inside option")))?;

    Ok(snack_log)
}

#[hdk_extern]
pub fn get_by_entry_hash(entry_hash: EntryHashB64) -> ExternResult<SnackingLog> {
    let element: Element = get(EntryHash::from(entry_hash), GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Could not find SnackingLog for header hash")))?;
    let option: Option<SnackingLog> = element.entry().to_app_option()?;
    let snack_log: SnackingLog = option
        .ok_or(WasmError::Guest(String::from("No book inside option")))?;

    Ok(snack_log)
}

#[hdk_extern]
pub fn get_all_headers_from_content(input: SnackingLog) -> ExternResult<Vec<SignedHeaderHashed>> {
    let hash: EntryHash = hash_entry(&input)?;
    let details = get_details(hash, GetOptions::default())?;
        
    match details {
        Some(Details::Entry(entry_details)) => {
            Ok(entry_details.headers)
        },
        None => Ok(vec![]),
        _ => Err(WasmError::Guest(String::from("Could not find SnackingLog based on content")))
    }
}
