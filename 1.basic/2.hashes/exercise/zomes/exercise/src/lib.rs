use hdk::prelude::*;

entry_defs![Book::entry_def()];

#[hdk_entry(id = "book")]
pub struct Book {
    title: String,
    content: String,
}

#[hdk_extern]
pub fn add_book(external_input: Book) -> ExternResult<EntryHash> { 
    unimplemented!()
}

#[hdk_extern]
pub fn get_book(external_input: EntryHash) -> ExternResult<Book> {
    unimplemented!()
}
