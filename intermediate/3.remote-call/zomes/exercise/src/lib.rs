use hdk::prelude::*;

entry_defs![Post::entry_def()];

#[hdk_entry(id = "post", visibility = "private")]
#[derive(Clone)]
pub struct Post(String);

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    unimplemented!();
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreatePostInput {
    content: String,
    target_agent: AgentPubKey,
}

#[hdk_extern]
pub fn create_post(_create_input: CreatePostInput) -> ExternResult<CreatePostInput> {
    unimplemented!();
}

#[hdk_extern]
pub fn receive_p2p_message(_post: Post) -> ExternResult<()> {
    unimplemented!()
}

#[hdk_extern]
pub fn get_my_posts(_: ()) -> ExternResult<Vec<Post>> {
    unimplemented!()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapReceive {
    pub cap_secret: CapSecret,
    pub from_agent: AgentPubKey
}

#[hdk_extern]
pub fn receive_cap_access(_cap: CapReceive) -> ExternResult<()> {
    unimplemented!()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrantCapAccess {
    pub function: String,
    pub agent: AgentPubKey
}

#[hdk_extern]
pub fn create_cap_access(_access: GrantCapAccess) -> ExternResult<GrantCapAccess> {
    unimplemented!()
}
