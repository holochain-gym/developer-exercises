use hdk::prelude::*;

entry_defs![Book::entry_def()];

#[hdk_entry(id = "book")]
pub struct Book {
    title: String,
    content: String,
}

#[hdk_extern]
pub fn add_book(external_input: Book) -> ExternResult<EntryHash> { 
    // We could return EntryHash inside the result, but when send hashes outside your zome, 
    // it is safer to use a base64 version of the hash

    let book: Book = external_input;
    let _unused_var: HeaderHash = create_entry(&book)?;
    let entry_hash: EntryHash = hash_entry(&book)?;

    Ok(entry_hash)
}

#[hdk_extern]
pub fn get_book(external_input: EntryHash) -> ExternResult<Book> {
    let entry_hash: EntryHash = external_input;
    let element: Element = get(entry_hash.into_hash(), GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Could not find book")))?;
    let book_option: Option<Book> = element.entry().to_app_option()?;
    let book: Book = book_option.expect("No book in option");

    Ok(book)
}
