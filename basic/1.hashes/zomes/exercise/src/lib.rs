use hdk::prelude::*;
use hc_utils::WrappedEntryHash;

entry_defs![Book::entry_def()];

#[hdk_entry(id="book")]
pub struct Book {
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalInput {
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalEntryHash{
    value: String,
}

#[hdk_extern]
pub fn add_book(input: SomeExternalInput) -> ExternResult<WrappedEntryHash> {
    //unimplemented!();
    let book:Book = Book{
                        title: input.title,
                        content: input.content
                    };
    let _a: HeaderHash = create_entry(&book)?;
    let x: EntryHash = hash_entry(&book)?;

    Ok(WrappedEntryHash(x))
}

#[hdk_extern]
pub fn get_book(hash: WrappedEntryHash) -> ExternResult<Book> {
    let element:Element = get(hash.0, GetOptions::default())?.ok_or(WasmError::Guest(String::from("Could not find book")))?;
    let bookoption: Option<Book> = element.entry().to_app_option()?;
    let book: Book = bookoption.unwrap();
    Ok(book)
}
