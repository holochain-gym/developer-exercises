use hdk::prelude::*;

//  1. Declare an entry data type called Post and register it with the entry_defs macro

entry_defs![Post::entry_def()];

#[hdk_entry(id = "post")]
pub struct Post(String);

//  2. Create an ExternalPostData structure:
//    CreatePostInput:
//      This structure can take a string for the content of the post.
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
pub fn create_post(external_data: CreatePostInput) -> ExternResult<EntryHash> {
    unimplemented!()
}

//  4. get_posts_for_agent()
//      Given the agent_pubkey, find all posts created by the given agent
//      and return a vector of all the Post structures using get().

#[hdk_extern]
pub fn get_posts_for_agent(agent_pubkey: AgentPubKey) -> ExternResult<Vec<Post>> {
    unimplemented!()
}
