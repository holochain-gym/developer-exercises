use std::unimplemented;

use hdk::prelude::*;

entry_defs![];

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    unimplemented!()
}

#[hdk_extern]
pub fn get_cap_tokens(_filter_by_agent: AgentPubKey) -> ExternResult<Vec<CapClaim>> {
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
pub fn create_transferable_cap_access(_access: GrantCapAccess) -> ExternResult<GrantCapAccess> {
    unimplemented!()
}

#[hdk_extern]
pub fn create_assigned_cap_access(_access: GrantCapAccess) -> ExternResult<GrantCapAccess> {
    unimplemented!()
}
