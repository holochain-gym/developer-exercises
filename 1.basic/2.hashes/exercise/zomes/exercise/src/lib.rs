use hdk::prelude::*;

entry_defs![];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalInput {
    title: String,
    content: String,
}

#[hdk_extern]
pub fn add_book(external_input: SomeExternalInput) -> ExternResult<EntryHash> { 
    unimplemented!()
}

#[hdk_extern]
pub fn get_book(external_input: EntryHash) -> ExternResult<Book> {
    unimplemented!()
}
