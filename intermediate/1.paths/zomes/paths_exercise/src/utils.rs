use super::*;
use hdk3::{hash_path::path::Component};

pub fn get_posts_by_day(input: GetPostsByTimeInput) -> ExternResult<Vec<Post>> {
    let path = Path::from(format!(
        "all_posts.{}-{}-{}",
        input.year, input.month, input.day
    ));

    let children = path.children()?;

    let posts = children
        .into_inner()
        .into_iter()
        .map(|hour_link| {
            let hour_str = get_last_component_string(hour_link.tag)?;

            let hour = hour_str.parse::<usize>().or(Err(err("Invalid path")))?;

            get_posts_by_hour(input.year, input.month, input.day, hour)
        })
        .collect::<ExternResult<Vec<Vec<Post>>>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(posts)
}

pub fn get_posts_by_hour(
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
) -> ExternResult<Vec<Post>> {
    let path = Path::from(format!("all_posts.{}-{}-{}.{}", year, month, day, hour));

    let links = get_links(path.hash()?, None)?;

    let posts: Vec<Post> = links
        .into_inner()
        .into_iter()
        .map(|link| get_post_by_hash(link.target))
        .collect::<ExternResult<Vec<Post>>>()?;

    Ok(posts)
}

pub fn get_post_by_hash(post_hash: EntryHash) -> ExternResult<Post> {
    let element = get(post_hash, GetOptions::default())?.ok_or(err("Could not found post"))?;

    let post: Option<Post> = element.entry().to_app_option()?;

    post.ok_or(err("Could not convert post"))
}

pub fn err(reason: &str) -> HdkError {
    HdkError::Wasm(WasmError::Zome(String::from(reason)))
}

#[derive(Serialize, Deserialize, Clone, Debug, SerializedBytes)]
struct StrComponent(String);
pub fn get_last_component_string(path_tag: LinkTag) -> ExternResult<String> {
    let bytes = SerializedBytes::from(UnsafeBytes::from(path_tag.0));
    let hour_path: Path = bytes.try_into()?;
    let hour_components: Vec<Component> = hour_path.into();

    let hour_bytes: Vec<u8> = hour_components
        .last()
        .ok_or(err("Invalid path"))?
        .clone()
        .into();
    let hour_str: StrComponent = SerializedBytes::from(UnsafeBytes::from(hour_bytes)).try_into()?;

    Ok(hour_str.0)
}
