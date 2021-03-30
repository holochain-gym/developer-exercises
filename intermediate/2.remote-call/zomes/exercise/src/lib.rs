use hdk::prelude::*;

entry_defs![Post::entry_def()];

#[hdk_entry(id = "post", visibility = "private")]
#[derive(Clone)]
pub struct Post(String);

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreatePostInput {
    content: String,
    target_agent: AgentPubKey,
}

#[hdk_extern]
pub fn create_post(create_input: CreatePostInput) -> ExternResult<CreatePostInput> {
    //Get the cap claim from local private chain
    let cap_secret = query(ChainQueryFilter::new().entry_type(EntryType::CapClaim).include_entries(true))?
        .into_iter()
        .map(|elem| {
            elem.entry().to_owned().into_option().unwrap().as_cap_claim().expect("Could not get option").to_owned()
        })
        .filter(|grant| grant.grantor() == &create_input.target_agent)
        .collect::<Vec<CapClaim>>()
        .pop()
        .ok_or(WasmError::Host(
            "No cap claim was found for desired target agent".to_string(),
        ))?;

    //Call remote function of target agent using secret found in cap claim
    call_remote(
        create_input.target_agent.clone(),
        zome_info()?.zome_name,
        "receive_p2p_message".into(),
        Some(cap_secret.secret().to_owned()),
        Post(create_input.content.clone()),
    )?;
    Ok(create_input)
}

#[hdk_extern]
pub fn receive_p2p_message(post: Post) -> ExternResult<()> {
    create_entry(post.clone())?;
    Ok(())
}

#[hdk_extern]
pub fn get_my_posts(_: ()) -> ExternResult<Vec<Post>> {
    let entry_def = Post::entry_def();
    let posts = query(
        ChainQueryFilter::new().entry_type(EntryType::App(AppEntryType::new(
            0.into(),
            zome_info()?.zome_id,
            entry_def.visibility,
        ))).include_entries(true),
    )?
    .into_iter()
    .map(|elem| elem.entry().to_app_option::<Post>().unwrap().unwrap())
    .collect::<Vec<Post>>();
    Ok(posts)
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
pub fn create_cap_access(access: GrantCapAccess) -> ExternResult<GrantCapAccess> {
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
