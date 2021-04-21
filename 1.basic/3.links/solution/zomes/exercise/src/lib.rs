use hdk::prelude::*;

//  1. Declare an entry data type called Post and register it with the entry_defs macro

entry_defs![Post::entry_def()];

#[hdk_entry(id = "post")]
pub struct Post(String);

//  2. Create an CreatePostInput structure:
//    CreatePostInput:
//      This structure can take a pair of strings for content and tag data.
//      As all create_link function calls require something to be passed into
//      the tag option, tag-less posts will need to be passed an empty string --> ''

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreatePostInput {
    content: String,
}

//  3. create_post()
//      Create an entry from input data. Then, pass the EntryHash of the data
//      to the link creation call, where the base of the link is the agent public key.
//      Then return the EntryHash of the post.

#[hdk_extern]
pub fn create_post(external_data: CreatePostInput) -> ExternResult<EntryHash> {
    let post = Post(external_data.content);

    create_entry(&post)?;
    let post_entry_hash: EntryHash = hash_entry(&post)?;

    create_link(
        agent_info()?.agent_latest_pubkey.into(),
        post_entry_hash.clone(),
        (),
    )?;

    Ok(post_entry_hash.clone())
}

//  4. get_posts_for_agent()
//      Given the agent_latest_pubkey, find all posts with the specified tag and
//      and return a vector of all the Post structures using get().
//      A private function `_return_content` handles the content gathering.

#[hdk_extern]
pub fn get_posts_for_agent(agent_pubkey: AgentPubKey) -> ExternResult<Vec<Post>> {
    let mut content: Vec<Post> = Vec::new();

    let links: Links = get_links(agent_pubkey.into(), None)?;

    for l in links.into_inner() {
        content.push(_return_content(l)?);
    }

    Ok(content)
}

fn _return_content(link: Link) -> ExternResult<Post> {
    let element: Element = get(link.target, GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Entry not found")))?;
    let entry_option: Option<Post> = element.entry().to_app_option()?;
    let entry: Post =
        entry_option.ok_or(WasmError::Guest("The targeted entry is not a post".into()))?;
    Ok(entry)
}
