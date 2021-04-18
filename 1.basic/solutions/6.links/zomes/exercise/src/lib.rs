use hdk::prelude::*;

entry_defs![Post::entry_def(), Author::entry_def()];

#[hdk_entry(id = "post")]
pub struct Post(String);

#[hdk_entry(id = "author")]
pub struct Author(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LinkInput {
    post_entry_hash: EntryHash,
    author_entry_hash: EntryHash,
}

#[hdk_extern]
pub fn create_author(external_data: Author) -> ExternResult<EntryHash> {
    // unimplemented!()
    // TODO Create an author entry from input data and return the EntryHash of the author.
    let author = external_data;

    create_entry(&author)?;
    let author_entry_hash: EntryHash = hash_entry(&author)?;

    Ok(author_entry_hash)  // is the same as:   EntryHashB64::from(author_entry_hash)
}

#[hdk_extern]
pub fn create_post(external_data: Post) -> ExternResult<EntryHash> {
    // unimplemented!()
    // TODO Create a post entry from input data and return the EntryHash of the post.
    let post = external_data;

    create_entry(&post)?;
    let post_entry_hash: EntryHash = hash_entry(&post)?;

    Ok(post_entry_hash)
}

#[hdk_extern]
pub fn link_author_to_post(external_data: LinkInput) -> ExternResult<HeaderHash> {

    let author_hash:EntryHash = external_data.author_entry_hash;
    let post_hash:EntryHash = external_data.post_entry_hash;
    
    let link_tag: LinkTag = LinkTag::new(String::from("is_author"));

    let link_hash: HeaderHash = create_link(
        author_hash,
        post_hash,
        link_tag,
    )?;

    Ok(link_hash)
}

#[hdk_extern]
pub fn get_link_header(external_data:HeaderHash) -> ExternResult<Header>{
    let link_header_hash: HeaderHash = external_data;
    // used by test to validate if link was created correctly
    let element: Element = get(link_header_hash, GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Entry not found")))?;
    Ok(element.header().clone())
}

//  4. get_posts_for_agent()
//      Given the agent_latest_pubkey, find all posts with the specified tag and
//      and return a vector of all the Post structures using get().
//      A private function `_return_content` handles the content gathering.

#[hdk_extern]
pub fn get_posts_for_author(author_entry_hash: EntryHash) -> ExternResult<Vec<Post>> {
    // unimplemented!()
    // get all the links for the author
    // return all posts as a collection
    
    let links: Links = get_links(author_entry_hash, None)?;
    
    let mut content: Vec<Post> = Vec::new();
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



// EXTRA
// create comment and link immediately to post

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostCommentInput {
    post_entry_hash: EntryHash,
    comment: String,
}

#[hdk_entry(id = "comment")]
pub struct Comment(String);

// converts string to comment; implementing this automaticly provide .into()
impl From<String> for Comment {
    fn from(s:String)->Self{
        Comment{0:s}
    }
}

#[hdk_extern]
pub fn comment_on_post(external_data: PostCommentInput) -> ExternResult<HeaderHash> {
    let post_entry_hash:EntryHash = external_data.post_entry_hash;
    let comment:Comment = external_data.comment.into();

    create_entry(&comment)?;
    let comment_entry_hash: EntryHash = hash_entry(&comment)?;

    let link_tag: LinkTag = LinkTag::new(String::from("has_comment"));
    let link_hash: HeaderHash = create_link(
        post_entry_hash,
        comment_entry_hash,
        link_tag,
    )?;

    Ok(link_hash)
}