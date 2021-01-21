use hdk3::prelude::*;
mod tests;

// Gym exercise: http://holochain-gym.github.io/developers/beginner/elements/

// Returns the header hash from the element
pub fn get_header_hash(element: Element) -> HeaderHash {
    element.header_address().clone()
}

// Returns the header timestamp from the element
pub fn get_header_timestamp(element: Element) -> timestamp::Timestamp {
    element.header().timestamp()
}

// Returns whether the element contains some entry or not
pub fn contains_entry(element: Element) -> bool {
    element.header().entry_data().is_some()
}

// Returns whether the header has been produced by the subconscious of holochain or if it was some hdk call
pub fn is_header_subconscious(header: Header) -> bool {
    match header {
        Header::Create(_)
        | Header::CreateLink(_)
        | Header::Delete(_)
        | Header::Update(_)
        | Header::DeleteLink(_) => false,
        _ => true,
    }
}