use hdk::prelude::*;

entry_defs![Message::entry_def()];

/// The tag of CapGrants to send messages.
/// 
/// This would allow us to distinguish between grants for this zome and
/// other zomes.
const SEND_PEER_MESSAGE_CAP_GRANT_TAG: &str = "send_peer_message";

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
    // Allow any agent to send us capabilities to send them messages
    create_unrestricted_grant("receive_peer_message_cap_secret")?;
    Ok(InitCallbackResult::Pass)
}

/// Directs this Agent to grant `input.agent` the capability to send messages
#[hdk_extern]
pub fn grant_peer_message_cap(input: GrantPeerMessageCapInput) -> ExternResult<GrantPeerMessageCapInput> {
    let mut functions: GrantedFunctions = BTreeSet::new();
    functions.insert((zome_info()?.name, "receive_peer_message".into()));

    // Record that the agent can call these functions
    let cap_secret = generate_cap_secret()?;
    create_cap_grant(CapGrantEntry {
        tag: SEND_PEER_MESSAGE_CAP_GRANT_TAG.into(),
        access: CapAccess::from((cap_secret, input.agent.clone())),
        functions,
    })?;

    // Call the zome of target agent and give them the generated cap secret
    call_remote(
        input.agent.clone(),
        zome_info()?.name,
        "receive_peer_message_cap_secret".into(),
        None,
        CapSecretInput {
            cap_secret: cap_secret,
            from_agent: agent_info()?.agent_initial_pubkey,
        },
    )?;
    Ok(input)
}

/// Receive the capability to message another agent.
/// 
/// Any agent can call this zome function.
#[hdk_extern]
pub fn receive_peer_message_cap_secret(input: CapSecretInput) -> ExternResult<()> {
    create_cap_claim(CapClaimEntry {
        tag: SEND_PEER_MESSAGE_CAP_GRANT_TAG.into(),
        grantor: input.from_agent,
        secret: input.cap_secret,
    })?;
    Ok(())
}

/// Sends a remote message with the given content and recipient
#[hdk_extern]
pub fn send_message(input: SendMessageInput) -> ExternResult<SendMessageInput> {
    //Get the cap claim from local private chain
    let cap_claim = get_cap_claim(&input.target_agent, SEND_PEER_MESSAGE_CAP_GRANT_TAG)?;

    // Call remote function of target agent using secret found in cap claim
    match zome_call_error_message(call_remote(
        input.target_agent.clone(),
        zome_info()?.name,
        "receive_peer_message".into(),
        Some(cap_claim.secret().to_owned()),
        input.message.clone(),
    )?) {
        Ok(()) => Ok(input),
        Err(message) => Err(WasmError::Guest(message))
    }
}

/// Sends a remote message without valid CapClaim
/// 
/// This is just for testing.
#[hdk_extern]
pub fn send_unauthorized_message(input: SendMessageInput) -> ExternResult<SendMessageInput> {
    // Make up a cap secret
    let cap_secret = generate_cap_secret()?;

    match zome_call_error_message(call_remote(
        input.target_agent.clone(),
        zome_info()?.name,
        "receive_peer_message".into(),
        Some(cap_secret),
        input.message.clone(),
    )?) {
        Ok(()) => Ok(input),
        Err(message) => Err(WasmError::Guest(message))
    }
}

#[hdk_extern]
pub fn receive_peer_message(message: Message) -> ExternResult<()> {
    create_entry(message)?;
    Ok(())
}

#[hdk_extern]
pub fn get_messages(_: ()) -> ExternResult<Vec<Message>> {
    get_all_entries::<Message>()
}

fn get_all_entries<T: EntryDefRegistration + TryFrom<SerializedBytes, Error = SerializedBytesError>>() -> ExternResult<Vec<T>> {
    let entry_def = T::entry_def();
    let entries = query(
        ChainQueryFilter::new()
            .entry_type(EntryType::App(AppEntryType::new(
                entry_def_index!(T)?,
                zome_info()?.id,
                entry_def.visibility,
            )))
            .include_entries(true),
        )?
        .into_iter()
        .map(|elem| elem.entry().to_app_option::<T>().unwrap().unwrap())
        .collect::<Vec<T>>();
    Ok(entries)
}

fn create_unrestricted_grant(fn_name: &str) -> ExternResult<HeaderHash> {
    let mut functions: GrantedFunctions = BTreeSet::new();
    functions.insert((zome_info()?.name, fn_name.into()));
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        access: CapAccess::Unrestricted,
        functions,
    })
}

/// Returns the CapClaim with the grantor and tag
fn get_cap_claim(grantor_hash: &AgentPubKey, tag: &str) -> ExternResult<CapClaim> {
    query(
        ChainQueryFilter::new()
            .entry_type(EntryType::CapClaim)
            .include_entries(true),
    )?
        .into_iter()
        .map(|elem| {
            elem.entry()
                .to_owned()
                .into_option()
                .unwrap()
                .as_cap_claim()
                .expect("Could not get option")
                .to_owned()
        })
        .filter(|grant|
            // We only want a grant for messaging the agent
            grant.grantor() == grantor_hash && 
            grant.tag == tag)
        .collect::<Vec<CapClaim>>()
        .pop()
        .ok_or(WasmError::Host(
            "No CapClaim for messaging the agent was found".to_string(),
        ))
}

/// Returns an error string for the ZomeCallResponse, if any
fn zome_call_error_message(response: ZomeCallResponse) -> Result<(), String> {
    let result = match response {
        ZomeCallResponse::Ok(_) => Ok(()),
        r @ ZomeCallResponse::Unauthorized(_, _, _, _) => 
            Err(format!("Unauthorized remote call {r:?}")),
        ZomeCallResponse::NetworkError(m) => 
            Err(format!("Network error during remote call: {m:?}")),
        ZomeCallResponse::CountersigningSession(m) => 
            Err(format!("Counter signing session failed to start: {m:?}")),
    };
    match result {
        Ok(()) => Ok(()),
        Err(m) => Err(m),
    }
}