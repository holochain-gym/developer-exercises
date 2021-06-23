use hdk::prelude::*;

entry_defs![Post::entry_def(), Anchor::entry_def()];

#[hdk_entry(id = "post")]
pub struct Post(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomeExternalInput {
    content: String,
}

#[hdk_extern]
pub fn create_post(input: SomeExternalInput) -> ExternResult<HeaderHash> {
    unimplemented!()
}

#[hdk_extern]
pub fn get_all_posts(_: ()) -> ExternResult<Vec<Post>> {
    unimplemented!()
}