use hdk::prelude::*;
use holo_hash::EntryHashB64;

entry_defs![Password::entry_def()];

#[hdk_entry(id = "password")]
pub struct Password {
    username: String,
    code: String,
}

#[hdk_extern]
pub fn add_password(external_input: Password) -> ExternResult<EntryHashB64> { 
    // We could return EntryHash inside the result, but when send hashes outside your zome, 
    // it is safer to use a base64 version of the hash

    let password: Password = external_input;
    let _unused_var: HeaderHash = create_entry(&password)?;
    let entry_hash: EntryHash = hash_entry(&password)?;

    Ok(EntryHashB64::from(entry_hash))
}

#[hdk_extern]
pub fn get_password(external_input: EntryHash) -> ExternResult<Password> {
    let entry_hash: EntryHash = external_input;
    let element: Element = get(entry_hash.into_hash(), GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Could not find password")))?;
    let password_option: Option<Password> = element.entry().to_app_option()?;
    let password: Password = password_option.expect("No password in option");

    Ok(password)
}

#[hdk_extern]
pub fn get_password_by_guessing(external_input: Password) -> ExternResult<Password> { 
    let password: Password = external_input;
    let entry_hash: EntryHash = hash_entry(&password)?;
    let element: Element = get(entry_hash, GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Could not find password")))?;
    let pw_option: Option<Password> = element.entry().to_app_option()?;
    let pw: Password = pw_option.expect("No book in option");

    Ok(pw)
}