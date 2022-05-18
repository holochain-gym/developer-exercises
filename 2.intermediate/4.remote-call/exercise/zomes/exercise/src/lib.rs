use hdk::prelude::*;

entry_defs![Message::entry_def()];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrantPeerMessageCapInput {
    pub agent: AgentPubKey,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapSecretInput {
    pub cap_secret: CapSecret,
    pub from_agent: AgentPubKey,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendMessageInput {
    message: Message,
    target_agent: AgentPubKey,
}

/// A message from one agent to another
#[hdk_entry(id = "post", visibility = "private")]
#[derive(Clone)]
pub struct Message(String);

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    unimplemented!()
}

/// Directs this Agent to grant `input.agent` the capability to send messages
#[hdk_extern]
pub fn grant_peer_message_cap(input: GrantPeerMessageCapInput) -> ExternResult<GrantPeerMessageCapInput> {
    unimplemented!()
}

/// Receive the capability to message another agent.
/// 
/// Any agent can call this zome function.
#[hdk_extern]
pub fn receive_peer_message_cap_secret(input: CapSecretInput) -> ExternResult<()> {
    unimplemented!()
}

/// Sends a remote message with the given content and recipient
#[hdk_extern]
pub fn send_message(input: SendMessageInput) -> ExternResult<SendMessageInput> {
    unimplemented!()
}

/// Sends a remote message without valid CapClaim
/// 
/// This is just for testing.
#[hdk_extern]
pub fn send_unauthorized_message(input: SendMessageInput) -> ExternResult<SendMessageInput> {
    unimplemented!()
}

#[hdk_extern]
pub fn receive_peer_message(message: Message) -> ExternResult<()> {
    unimplemented!()
}

#[hdk_extern]
pub fn get_messages(_: ()) -> ExternResult<Vec<Message>> {
    unimplemented!()
}
