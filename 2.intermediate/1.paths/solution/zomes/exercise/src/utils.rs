use hdk::hash_path::path::Component;

use super::*;
use std::convert::TryFrom;

pub fn get_posts_by_day(input: GetPostsByTimeInput) -> ExternResult<Vec<Post>> {
    let path = Path::from(format!(
        "all_posts.{}-{}-{}",
        input.year, input.month, input.day
    ));

    let children = path.children()?;

    let posts = children
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

    let links = get_links(path.path_entry_hash()?.into(), None)?;

    let posts: Vec<Post> = links
        .into_iter()
        .map(|link| get_post_by_hash(link.target.into()))
        .collect::<ExternResult<Vec<Post>>>()?;

    Ok(posts)
}

pub fn get_post_by_hash(post_hash: EntryHash) -> ExternResult<Post> {
    let element = get(post_hash, GetOptions::default())?.ok_or(err("Could not found post"))?;

    let post: Option<Post> = element.entry().to_app_option()?;

    post.ok_or(err("Could not convert post"))
}

pub fn err(reason: &str) -> WasmError {
    WasmError::Guest(String::from(reason))
}

pub fn get_last_component_string(path_tag: LinkTag) -> ExternResult<String> {
    let bytes = SerializedBytes::from(UnsafeBytes::from(path_tag.0));

    let hour_bytes = &Component::try_from(bytes)?;
    let hour_str: String = hour_bytes.try_into()?;

    Ok(hour_str)
}
