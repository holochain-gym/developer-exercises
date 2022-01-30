use hdk::prelude::*;
use chrono::{DateTime, NaiveDateTime, Utc};

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
    unimplemented!()
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
    unimplemented!()
}

#[hdk_extern]
pub fn get_all_tags(_: ()) -> ExternResult<Vec<String>> {
    unimplemented!()
}

#[hdk_extern]
pub fn get_posts_by_tag(tag: String) -> ExternResult<Vec<Post>> {
    unimplemented!()
}

/** Helper functions */

fn now_date_time() -> ExternResult<DateTime<Utc>> {
    let time = sys_time()?.as_seconds_and_nanos();

    let date: DateTime<Utc> =
        DateTime::from_utc(NaiveDateTime::from_timestamp(time.0, time.1), Utc);
    Ok(date)
}
