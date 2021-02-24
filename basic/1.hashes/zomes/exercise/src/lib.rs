use hdk3::prelude::*;

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
pub fn add_book(input: SomeExternalInput) -> ExternResult<EntryHash> {
    //unimplemented!();
    let book:Book = Book{
                        title: input.title,
                        content: input.content
                    };
    create_entry(&book)?;
    hash_entry(book)
}


#[hdk_extern]
pub fn get_book(entry_hash: SomeExternalEntryHash) -> ExternResult<SomeExternalOutput> {
    let e:EntryHash = EntryHash::from_raw_36(entry_hash.value.as_bytes().to_vec());
    let element = get(e, GetOptions::default())?.ok_or(WasmError::Zome(String::from("Could not find book")))?;
    let bookoption: Option<Book> = element.entry().to_app_option()?;
    let book:Book = bookoption.unwrap();
    let response = SomeExternalOutput{
        title: book.title,
        content: book.content,
    };
    Ok(response) //.ok_or(WasmError::Zome(String::from("Could not convert book")))
}



