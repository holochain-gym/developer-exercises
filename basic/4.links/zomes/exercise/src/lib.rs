use hdk::link::get_links;
use hdk::prelude::*;

//  1. Declare an entry data type and register it within the macro

entry_defs![EntryData::entry_def()];

#[hdk_entry(id = "entrydata")]
pub struct EntryData(String);

//  2. Create data structures that can be used to bring in external data
//    ExternalEntryContentData is designed to contain the App entry data
//    ExternalEntryHashData is designed to bring in an EntryHash
//    ExternalLinkData is designed to be used for create_link and get_links, where
//      base, target and a tag are needed.

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExternalEntryContentData {
    content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExternalEntryHashData {
    hash: EntryHash,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExternalLinkData {
    base: EntryHash,
    target: Option<EntryHash>,
    tag: String,
}

//  3. The first two functions should be familiar to you:
//    make_entry_and_hash()
//      Create an entry from data, hash it and return.
//    get_entry_from_input()
//      Given an entry hash, return the contents of the entry

#[hdk_extern]
pub fn make_entry_and_hash(input: ExternalEntryContentData) -> ExternResult<EntryHash> {
    let data: EntryData = EntryData(input.content);
    let _: HeaderHash = create_entry(&data)?;
    Ok(hash_entry(&data)?)
}

#[hdk_extern]
pub fn get_entry_from_input(input: ExternalEntryHashData) -> ExternResult<EntryData> {
    let element: Element = get(input.hash, GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Entry not found")))?;
    let input_entry: Option<EntryData> = element.entry().to_app_option()?;
    let entry: EntryData = input_entry.unwrap();
    Ok(entry)
}

//  4. These are the new functions to examine:
//    make_links()
//      A wrapper around create_link that creates a new LinkTag from the input, then creates the link
//      and returns the header hash of the created link.
//    find_links()
//      Given an entry hash of the base + tag ('' or String), return a list of Links

#[hdk_extern]
pub fn make_links(input: ExternalLinkData) -> ExternResult<HeaderHash> {
    let new_tag: LinkTag = LinkTag::new(input.tag);
    let new_link: HeaderHash = create_link(input.base, input.target.unwrap(), new_tag)?;
    Ok(new_link)
}

#[hdk_extern]
pub fn find_links(input: ExternalLinkData) -> ExternResult<Links> {
    let links: Links = get_links(input.base, Some(LinkTag::new(input.tag)))?;
    Ok(links)
}
