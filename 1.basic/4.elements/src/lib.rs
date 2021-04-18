use hdk3::prelude::*;
mod tests;

// Gym Exercise: http://holochain-gym.github.io/developers/beginner/elements/

// Returns the header hash from the element
pub fn get_header_hash(_element: Element) -> HeaderHash {
    unimplemented!()
}

// Returns the header timestamp from the element
pub fn get_header_timestamp(_element: Element) -> timestamp::Timestamp {
    unimplemented!()
}

// Returns whether the element contains some entry or not
pub fn contains_entry(_element: Element) -> bool {
    unimplemented!()
}

// Returns whether the header has been produced by the subconscious of holochain or if it was some hdk call
pub fn is_header_subconscious(_header: Header) -> bool {
    unimplemented!()
}
