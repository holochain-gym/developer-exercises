



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalInput {
    content: String,
}

#[hdk_extern()]
pub fn say_greeting(input: SomeExternalInput) -> ExternResult<HeaderHash> {
    unimplemented!()
}
