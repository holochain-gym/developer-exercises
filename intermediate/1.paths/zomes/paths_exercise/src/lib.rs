use hdk3::prelude::*;
use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};

entry_defs![Path::entry_def(), Post::entry_def()];

#[hdk_entry(id = "post")]
#[derive(Clone, Debug)]
pub struct Post(String);

#[derive(Serialize, Deserialize, Clone, Debug, SerializedBytes)]
pub struct GetPostsOutput(Vec<Post>);

#[derive(Serialize, Deserialize, Clone, Debug, SerializedBytes)]
pub struct CreateTaskInput {
    content: String,
    tags: Vec<String>,
}
#[hdk_extern]
pub fn create_post(task_input: CreateTaskInput) -> ExternResult<EntryHash> {
    unimplemented!()
}

#[derive(Serialize, Deserialize, Clone, Debug, SerializedBytes)]
pub struct GetPostsByTimeInput {
    year: usize,
    month: usize,
    day: usize,
    hour: Option<usize>,
}
#[hdk_extern]
pub fn get_post_by_time(input: GetPostsByTimeInput) -> ExternResult<GetPostsOutput> {
    unimplemented!()
}

#[derive(Serialize, Deserialize, Clone, Debug, SerializedBytes)]
pub struct GetTagsOutput(Vec<String>);
#[hdk_extern]
pub fn get_all_tags(_: ()) -> ExternResult<GetTagsOutput> {
    unimplemented!()
}

#[derive(Serialize, Deserialize, Clone, Debug, SerializedBytes)]
pub struct GetPostsByTagInput(String);
#[hdk_extern]
pub fn get_posts_by_tag(input: GetPostsByTagInput) -> ExternResult<GetPostsOutput> {
    unimplemented!()
}

/** Helper functions */

fn now_date_time() -> ExternResult<DateTime<Utc>> {
    let time = sys_time()?;

    let secs = time.as_secs();

    let date: DateTime<Utc> =
        DateTime::from_utc(NaiveDateTime::from_timestamp(secs as i64, 0), Utc);
    Ok(date)
}
