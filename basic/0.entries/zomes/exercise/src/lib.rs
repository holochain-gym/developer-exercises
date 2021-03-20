use hdk::prelude::*;

entry_defs![Greeting::entry_def()];

#[hdk_entry()]
pub struct Greeting(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalInput {
    greeting_text: String,
}


pub fn say_greeting(input: SomeExternalInput) -> ExternResult<HeaderHash> {
    let greeting:Greeting = Greeting(input.greeting_text);
    create_entry(greeting)
}
