use hdk::prelude::*;

entry_defs![];

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    // grant unrestricted access to accept_cap_claim so other agents can send us claims
    let mut functions: GrantedFunctions = HashSet::new();
    functions.insert((zome_info()?.zome_name, "receive_cap_access".into()));
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;

    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn get_cap_tokens(filter_by_agent: AgentPubKey) -> ExternResult<Vec<CapClaim>> {
    let cap_claims = query(ChainQueryFilter::new().entry_type(EntryType::CapClaim).include_entries(true))?
        .into_iter()
        .map(|elem| {
            elem.entry().to_owned().into_option().unwrap().as_cap_claim().expect("Could not get option").to_owned()
        })
        .filter(|grant| grant.grantor() == &filter_by_agent)
        .collect::<Vec<CapClaim>>();
    debug!("Got claims: {:#?}", cap_claims);
    Ok(cap_claims)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapReceive {
    pub cap_secret: CapSecret,
    pub from_agent: AgentPubKey
}

#[hdk_extern]
pub fn receive_cap_access(cap: CapReceive) -> ExternResult<()> {
    create_cap_claim(CapClaimEntry::new(
        "p2p_message".into(),
        cap.from_agent,
        cap.cap_secret,
    ))?;
    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GrantCapAccess {
    pub function: String,
    pub agent: AgentPubKey
}

#[hdk_extern]
pub fn create_transferable_cap_access(access: GrantCapAccess) -> ExternResult<GrantCapAccess> {
    debug!("Sending cap to: {:#?}", access.agent);
    //Create function map of cap grant
    let mut functions: GrantedFunctions = HashSet::new();
    functions.insert((zome_info()?.zome_name, access.function.clone().into()));

    //Create the cap grant and commit for current agent
    let cap_secret = generate_cap_secret()?;
    create_cap_grant(CapGrantEntry {
        tag: "cap_grant".into(),
        access: CapAccess::from(cap_secret),
        functions,
    })?;

    //Call the zome of target agent and give them the generated cap secret
    call_remote(
        access.agent.clone(),
        zome_info()?.zome_name,
        "receive_cap_access".into(),
        None,
        CapReceive {
            cap_secret: cap_secret,
            from_agent: agent_info()?.agent_initial_pubkey
        },
    )?;
    Ok(access)
}

#[hdk_extern]
pub fn create_assigned_cap_access(access: GrantCapAccess) -> ExternResult<GrantCapAccess> {
    //Create function map of cap grant
    let mut functions: GrantedFunctions = HashSet::new();
    functions.insert((zome_info()?.zome_name, access.function.clone().into()));

    //Create the cap grant and commit for current agent
    let cap_secret = generate_cap_secret()?;
    create_cap_grant(CapGrantEntry {
        tag: "cap_grant".into(),
        access: CapAccess::from((cap_secret, access.agent.clone())),
        functions,
    })?;

    //Call the zome of target agent and give them the generated cap secret
    call_remote(
        access.agent.clone(),
        zome_info()?.zome_name,
        "receive_cap_access".into(),
        None,
        CapReceive {
            cap_secret: cap_secret,
            from_agent: agent_info()?.agent_initial_pubkey
        },
    )?;
    Ok(access)
}
