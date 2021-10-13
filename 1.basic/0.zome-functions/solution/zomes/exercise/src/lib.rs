// Problem statement: https://holochain-gym.github.io/developers/basic/zome-functions/
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalOutput(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Greeting {
    greeting_text: String,
}

#[hdk_extern]
pub fn hello_world(_: ()) -> ExternResult<SomeExternalOutput> {
    let message: String = String::from("Hello world");
    let output: SomeExternalOutput = SomeExternalOutput(message);

    Ok(output)
}

#[hdk_extern]
pub fn get_agent_id(_: ()) -> ExternResult<AgentInfo> {
    Ok(agent_info()?)
}
