use hdk::info::agent_info;
use hdk::link::get_links;
use hdk::prelude::*;

//  1. Declare an entry data type called Post and register it with the entry_defs macro

entry_defs![Post::entry_def()];

#[hdk_entry(id = "post")]
pub struct Post(String);

//  2. Create an ExternalPostData structure:
//    ExternalPostData:
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
pub fn create_post(external_data: ExternalPostData) -> ExternResult<EntryHash> {
    unimplemented!()
}

//  4. get_posts_for_agent()
//      Given the agent_latest_pubkey, find all posts with the specified tag and
//      and return a vector of all the Post structures using get().
//      A private function `_return_content` handles the content gathering.

#[hdk_extern]
pub fn get_posts_for_agent(link_query: String) -> ExternResult<Vec<Post>> {
    unimplemented!()
}
