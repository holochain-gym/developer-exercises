



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalInput {
    greeting_text: String,
}


pub fn say_greeting(input: SomeExternalInput) -> ExternResult<HeaderHash> {
    unimplemented!()
}
