use hdk::prelude::*;

entry_defs![Post::entry_def(), Anchor::entry_def()];

const POST_ANCHOR_TYPE: &str = "posts";
const POST_ANCHOR_TEXT: &str = "posts";

#[hdk_entry(id = "post")]
pub struct Post(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalInput {
    content: String,
}

#[hdk_extern]
pub fn create_post(input: SomeExternalInput) -> ExternResult<HeaderHash> {
    let anchor = anchor(POST_ANCHOR_TYPE.into(), POST_ANCHOR_TEXT.into())?;
    
    let post: Post = Post(input.content);

    let post_header = create_entry(&post)?;
    let post_entry = hash_entry(&post)?;
    create_link(anchor.into_hash(), post_entry, ())?;

    Ok(post_header)
}

#[hdk_extern]
pub fn get_all_posts(_: ()) -> ExternResult<Vec<Post>> {
    let anchor = anchor(POST_ANCHOR_TYPE.into(), POST_ANCHOR_TEXT.into())?;
    let mut content: Vec<Post> = Vec::new();

    let links: Links = get_links(anchor.into_hash(), None)?;

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
