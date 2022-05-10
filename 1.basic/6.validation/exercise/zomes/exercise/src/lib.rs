use hdk::prelude::holo_hash::HeaderHashB64;
use hdk::prelude::*;

entry_defs![Estimate::entry_def()];

#[hdk_entry(id = "estimate")]
pub struct Estimate {
    item: String,
    value: u8,
}

#[hdk_extern]
fn add_estimate(external_estimate: Estimate) -> ExternResult<HeaderHashB64> {
    let header_hash: HeaderHash = create_entry(&external_estimate)?;
    Ok(HeaderHashB64::from(header_hash))
}

#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op {
        Op::StoreEntry {
            header: SignedHashed {
                hashed: HoloHashed {
                    content: header, ..
                }, ..
            },
            entry,
        } => validate_entry_creation(header, entry),
        Op::StoreElement {
            element: Element {
                entry: element_entry,
                ..
            }
        } => validate_element_entry(element_entry),
        Op::RegisterAgentActivity {..} => Ok(ValidateCallbackResult::Valid),
        _ => Ok(ValidateCallbackResult::Invalid(format!(
            "Unsupported op: {:?}",
            op
        ))),
    }
}

fn validate_element_entry(element_entry: ElementEntry) -> ExternResult<ValidateCallbackResult> {
    match element_entry {
        ElementEntry::Present(entry) => validate_entry(entry),
        _ => Ok(ValidateCallbackResult::Valid)
    }
}

fn validate_entry_creation(header: EntryCreationHeader, entry: Entry) -> ExternResult<ValidateCallbackResult> {
    let app_entry_type = match header.app_entry_type() {
        Some(app_entry_type) => app_entry_type,
        None => return Ok(ValidateCallbackResult::Invalid("Missing app_entry_type".to_string())),
    };
    let this_zome = zome_info()?;
    if !this_zome.matches_entry_def_id(app_entry_type, Estimate::entry_def_id()) {
        return Ok(ValidateCallbackResult::Invalid(format!(
            "Unsupported entry type for creation: {:?}",
            header.entry_type()
        )))
    }
    validate_entry(entry)
}

fn validate_entry(entry: Entry) -> ExternResult<ValidateCallbackResult>  {
    let estimate = Estimate::try_from(&entry)?;
    match validate_estimate(estimate) {
        Err(message) => return Ok(ValidateCallbackResult::Invalid(
            format!("Estimate was invalid: {}", message),
        )),
        _ => (),
    }

    Ok(ValidateCallbackResult::Valid)
}

/// Return true iff the estimate has a valid value
pub fn validate_estimate(estimate: Estimate) -> Result<(), String> {
    let valid_estimate_values = vec![0, 1, 2, 3, 5, 8, 13, 20];
    unimplemented!()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_invalid_true() {
        let estimate = crate::Estimate { item: "task".to_string(), value: 6 };
        assert_eq!(crate::validate_estimate(estimate), Ok(()));
    }

    #[test]
    fn test_invalid_false() {
        let estimate = crate::Estimate { item: "task".to_string(), value: 8 };
        assert_eq!(match crate::validate_estimate(estimate) {
            Err(_) => false,
            _ => true,
        }, false);
    }
}
