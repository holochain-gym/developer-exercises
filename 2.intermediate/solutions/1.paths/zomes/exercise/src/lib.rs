use hdk::prelude::*;
use chrono::{DateTime, NaiveDateTime, Timelike, Datelike, Utc};

mod utils;
use utils::*;

entry_defs![Path::entry_def(), Post::entry_def()];

#[hdk_entry(id = "post")]
pub struct Post(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateTaskInput {
    content: String,
    tags: Vec<String>,
}
#[hdk_extern]
pub fn create_post(task_input: CreateTaskInput) -> ExternResult<EntryHash> {
    let post = Post(task_input.content);
    create_entry(&post)?;

    let date = now_date_time()?;

    let post_hash = hash_entry(&post)?;

    let time_path = Path::from(format!(
        "all_posts.{}-{}-{}.{}",
        date.year(),
        date.month(),
        date.day(),
        date.hour()
    ));

    time_path.ensure()?;

    create_link(time_path.hash()?, post_hash.clone(), ())?;

    for tag in task_input.tags {
        let tags_path = Path::from(format!("all_tags.{}", tag));

        tags_path.ensure()?;

        create_link(tags_path.hash()?, post_hash.clone(), ())?;
    }

    Ok(post_hash)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetPostsByTimeInput {
    year: usize,
    month: usize,
    day: usize,
    hour: Option<usize>,
}
#[hdk_extern]
pub fn get_posts_by_time(input: GetPostsByTimeInput) -> ExternResult<Vec<Post>> {
    let posts = match input.hour {
        None => get_posts_by_day(input),
        Some(h) => get_posts_by_hour(input.year, input.month, input.day, h),
    }?;

    Ok(posts)
}

#[hdk_extern]
pub fn get_all_tags(_: ()) -> ExternResult<Vec<String>> {
    let path = Path::from("all_tags");

    let links = path.children()?;

    let tags = links
        .into_inner()
        .into_iter()
        .map(|child_link| get_last_component_string(child_link.tag))
        .collect::<ExternResult<Vec<String>>>()?;

    Ok(tags)
}

#[hdk_extern]
pub fn get_posts_by_tag(tag: String) -> ExternResult<Vec<Post>> {
    let path = Path::from(format!("all_tags.{}", tag));

    let links = get_links(path.hash()?, None)?;

    let posts: Vec<Post> = links
        .into_inner()
        .into_iter()
        .map(|link| get_post_by_hash(link.target))
        .collect::<ExternResult<Vec<Post>>>()?;

    Ok(posts)
}

/** Helper functions */

fn now_date_time() -> ExternResult<DateTime<Utc>> {
    let time = sys_time()?;

    let secs = time.as_secs();

    let date: DateTime<Utc> =
        DateTime::from_utc(NaiveDateTime::from_timestamp(secs as i64, 0), Utc);
    Ok(date)
}
