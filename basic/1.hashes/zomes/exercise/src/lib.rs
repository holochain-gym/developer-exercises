use hdk::prelude::*;
use holo_hash::EntryHashB64;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalInput {
    title: String,
    content: String,
}

#[hdk_extern]
pub fn add_book(input: SomeExternalInput) -> ExternResult<EntryHashB64> {
    unimplemented!();
}

#[hdk_extern]
pub fn get_book(hash: EntryHashB64) -> ExternResult<Book> {
    unimplemented!()
}
