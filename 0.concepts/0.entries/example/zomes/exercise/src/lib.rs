use hdk::prelude::*;

entry_defs![Greeting::entry_def()];

#[hdk_entry(id = "greeting")]
pub struct Greeting(String);

#[hdk_extern]
pub fn say_greeting(external_input: Greeting) -> ExternResult<HeaderHash> {
    create_entry(&external_input)
}
