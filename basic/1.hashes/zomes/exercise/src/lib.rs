use hdk::prelude::*;
use hc_utils::WrappedEntryHash;
use hc_utils::wrappers::HashString;
// use hc_utils::get_latest_entry;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalOutput {
    title: String,
    content: String
}

#[hdk_extern]
pub fn add_book(input: SomeExternalInput) -> ExternResult<HashString> {
    //unimplemented!();
    let book:Book = Book{
                        title: input.title,
                        content: input.content
                    };
    create_entry(&book)?;
    let x: EntryHash = hash_entry(book).unwrap();
    let y: WrappedEntryHash = WrappedEntryHash(x);
    let z: HashString = y.into();
    Ok(z)
}

#[hdk_extern]
pub fn get_book(hash: HashString) -> ExternResult<SomeExternalOutput> {
    let option:Option<WrappedEntryHash> = WrappedEntryHash::try_from(hash).ok();
    let x = option.unwrap();
    let element = get(x.0, GetOptions::default());//?.ok_or(WasmError::Zome(String::from("Could not find book")))?;
    let _y = element;
    // let bookoption: Option<Book> = element.entry().to_app_option()?;
    // let _book:Book = bookoption.unwrap();
    let response = SomeExternalOutput
    {
        title: String::from("test"),
        content: String::from("test"),
    };
    Ok(response) //.ok_or(WasmError::Zome(String::from("Could not convert book")))
}



