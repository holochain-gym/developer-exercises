use hdk::prelude::*;
use hc_utils::WrappedEntryHash;
use hc_utils::wrappers::HashString;
// use hc_utils::get_latest_entry;

// WARNING: this exercise does not work for the moment. Waiting to be fixed are hdk version changes

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
pub fn add_book(input: SomeExternalInput) -> ExternResult<HashString> {
    //unimplemented!();
    let book:Book = Book{
                        title: input.title,
                        content: input.content
                    };
    let _a: HeaderHash = create_entry(&book)?;
    let x: EntryHash = hash_entry(book).unwrap();
    let y: WrappedEntryHash = WrappedEntryHash(x);
    let z: HashString = y.into();
    Ok(z)
}

#[hdk_extern]
pub fn get_book(hash: HashString) -> ExternResult<Book> {
    let option:Option<WrappedEntryHash> = WrappedEntryHash::try_from(hash).ok();
    let wrapped: WrappedEntryHash = option.unwrap();
    let element:Element = get(wrapped.0, GetOptions::default())?.ok_or(WasmError::Zome(String::from("Could not find book")))?;
    let bookoption: Option<Book> = element.entry().to_app_option()?;
    let book:Book = bookoption.unwrap();
    Ok(book) //.ok_or(WasmError::Zome(String::from("Could not convert book")))
}



