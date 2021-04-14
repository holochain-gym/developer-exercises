use hdk::prelude::*;
use holo_hash::HeaderHashB64;

entry_defs![SnackingLog::entry_def(), WorkoutLog::entry_def()];

#[hdk_entry(id = "SnackingLog")]
pub struct SnackingLog(String);

#[hdk_entry(id = "WorkoutLog")]
pub struct WorkoutLog(String);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HeaderHashPair{
    current: HeaderHashB64,
    previous: HeaderHashB64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Answer(String);


#[hdk_extern]
pub fn register_snacking(input: SnackingLog) -> ExternResult<HeaderHashB64> {
    Ok(HeaderHashB64::from(create_entry(&input)?))
}

#[hdk_extern]
pub fn is_previous_header_hash(pair: HeaderHashPair) -> ExternResult<Answer> {
    unimplemented!()

    // This function can return any of these answers
    // Ok(Answer(String::from("previous header NOT FOUND"))),
    // Ok(Answer(String::from("is NOT previous header")))
    // Ok(Answer(String::from("IS previous header")))
}

#[hdk_extern]
pub fn happened_before(pair: HeaderHashPair) -> ExternResult<Answer> {
    unimplemented!()

    // This function always returns one of these answers
    // Ok(Answer(String::from("happened before"))),
    // Ok(Answer(String::from("did NOT happen before")))
}

#[hdk_extern]
pub fn get_header_sequence_number(a: HeaderHashB64) -> ExternResult<Answer> {
    unimplemented!()
    // Ok(Answer(format!("header sequence is {}", sequence)))
}
