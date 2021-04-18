#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalInput {
    title: String,
    content: String,
}

pub fn add_book(input: SomeExternalInput) -> ExternResult<EntryHash> {
    unimplemented!();
}

pub fn get_book(hash: String) -> ExternResult<Book> {
    unimplemented!()
}
