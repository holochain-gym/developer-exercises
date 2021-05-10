use hdk::prelude::*;

entry_defs![SnackingLog::entry_def()];

#[hdk_entry(id = "SnackingLog")]
pub struct SnackingLog(String);

#[hdk_extern]
pub fn register_snacking(input: SnackingLog) -> ExternResult<HeaderHash> {
    create_entry(&input)
}

#[hdk_extern]
pub fn query_all_elements(_: ()) -> ExternResult<Vec<Element>> {
    unimplemented!()
}

#[hdk_extern]
pub fn query_snackings(_: ()) -> ExternResult<Vec<Element>> {
    unimplemented!()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeRange {
    start_time: Timestamp,
    end_time: Timestamp,
}

#[hdk_extern]
pub fn query_by_time(time_range: TimeRange) -> ExternResult<Vec<Element>> {
    unimplemented!()
}
