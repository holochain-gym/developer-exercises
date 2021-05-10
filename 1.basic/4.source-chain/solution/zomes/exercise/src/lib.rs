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
    let filter = ChainQueryFilter::new();
    query(filter)
}

#[hdk_extern]
pub fn query_snackings(_: ()) -> ExternResult<Vec<Element>> {
    let filter = ChainQueryFilter::new().include_entries(true).entry_type(EntryType::App(AppEntryType::new(
        entry_def_index!(SnackingLog)?,
        zome_info()?.zome_id,
        EntryVisibility::Public,
    )));

    query(filter)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeRange {
    start_time: Timestamp,
    end_time: Timestamp,
}

#[hdk_extern]
pub fn query_by_time(time_range: TimeRange) -> ExternResult<Vec<Element>> {
    let elements = query(ChainQueryFilter::new())?;

    let filtered_elements = elements
        .into_iter()
        .filter(|el| is_header_in_range(el.header(), &time_range))
        .collect();

    Ok(filtered_elements)
}

fn is_header_in_range(header: &Header, time_range: &TimeRange) -> bool {
    header.timestamp().gt(&time_range.start_time) && header.timestamp().lt(&time_range.end_time)
}
