// Problem statement: https://holochain-gym.github.io/developers/basic/zome-functions/

pub struct SomeExternalOutput(String);

pub fn hello_world(_: ()) -> ExternResult<SomeExternalOutput> {
    let message: String = String::from("Hello world");
    let output: SomeExternalOutput = SomeExternalOutput(message);

    Ok(output)
}

pub fn get_agent_id(_: ()) -> ExternResult<AgentInfo> {
    Ok(agent_info()?)
}
