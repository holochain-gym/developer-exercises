
// Problem statement: https://holochain-gym.github.io/developers/basic/zome-functions/

/// HINT: If your rust-analyzer throws the error "rust-analyzer failed to discover workspace",
/// this is because the folder you have currently open in VSCode (most probably the
/// "developer-exercises" folder) does not contain a "Cargo.toml" file.
/// Open the folder "1.basic/0.zome-functions/exercise" (which *does* contain a Cargo.toml file) in a separate VSCode
/// window to get the rust-analyzer running.

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