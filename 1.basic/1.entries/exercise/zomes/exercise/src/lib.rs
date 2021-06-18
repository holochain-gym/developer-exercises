
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalInput {
    content: String,
}

pub fn say_greeting(input: SomeExternalInput) -> ExternResult<HeaderHash> {
    unimplemented!()
}
