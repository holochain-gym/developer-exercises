use hdk::link::get_links;
use hdk::info::agent_info;
use hdk::prelude::*;

//  1. Declare an entry data type called Post and register it within the macro

entry_defs![Post::entry_def()];

#[hdk_entry(id = "post")]
pub struct Post(String);

//  2. Create an External Post Data structure:
//    This structure can take a pair of strings for content and tag data.
//    As all create_link function calls require a something to be passed into 
//    the tag option, tag-less posts will need to be passed an empty string --> ''

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExternalPostData {
    content: String,
    tag: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostQuery {
    agent_id: AgentPubKey,
    tag: String
}

//  3. create_post()
//      Create an entry from input data, pass the new entry HeaderHash to the 
//      link creation call with the base of the link as the agent public key,
//      and then return the EntryHash of the post

#[hdk_extern]
pub fn create_post(external_data: ExternalPostData) -> ExternResult<EntryHash> {
    let post: Post = Post(external_data.content);
    let _post_header_hash: HeaderHash = create_entry(&post)?;
    let post_entry_hash: EntryHash = hash_entry(&post)?;

    let _new_link: HeaderHash = create_link(
        HoloHash::from(agent_info()?.agent_latest_pubkey),
        post_entry_hash.clone(),
        LinkTag::new(external_data.tag)
    )?;

    Ok(post_entry_hash.clone())
}

//  4. get_posts_for_agent()
//      Given the AgentPubKey, find all posts with the specified tag and
//      and return a vector of all the Post structures. Here, we are going to
//      re-use the structure ExternalPostData

#[hdk_extern]
pub fn get_posts_for_agent(post_query: PostQuery) -> ExternResult<Vec<Post>> {
    let mut content: Vec<Post> = Vec::new();

    let links = get_links(
        HoloHash::from(post_query.agent_id),
        Some(LinkTag::new(post_query.tag))
    ).unwrap();

    for l in links.into_inner() {
        content.push(_return_content(l).unwrap())
    }

    Ok(content)
}

fn _return_content(link: Link) -> ExternResult<Post> {
    let element: Element = get(link.target, GetOptions::default()).unwrap()
        .ok_or(WasmError::Guest(String::from("Entry not found"))).unwrap();
    let entry_option: Option<Post> = element.entry().to_app_option()?;
    let entry: Post = entry_option.unwrap();
    Ok(entry)
}
