use hdk::prelude::*;

entry_defs![Greeting::entry_def()];

#[hdk_entry()]
pub struct Greeting(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalInput {
    content: String,
}


pub fn say_greeting(input: SomeExternalInput) -> ExternResult<HeaderHash> {
    let greeting:Greeting = Greeting(input.content);
    create_entry(greeting)
}
