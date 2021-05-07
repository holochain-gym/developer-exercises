
// Problem statement: https://holochain-gym.github.io/developers/basic/zome-functions/

pub struct SomeExternalInput {
    first_name: String,
    last_name: String,
}

pub struct SomeExternalOutput(String);

pub fn hello_world(_:()) -> ExternResult<SomeExternalOutput> {
    let message: String = String::from("Hello world");
    let output: SomeExternalOutput = SomeExternalOutput(message);
    
    Ok(output)
}

pub fn say_my_name(external_input:SomeExternalInput) -> ExternResult<SomeExternalOutput> {
    let message: String = format!("Your name is {} {}", 
                                    external_input.first_name, 
                                    external_input.last_name);
    let output: SomeExternalOutput = SomeExternalOutput(message);
    
    Ok(output)
}

pub fn get_agent_id(_:()) -> ExternResult<AgentInfo> {
    Ok(agent_info()?)
}