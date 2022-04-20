use hdk::prelude::holo_hash::HeaderHashB64;
use hdk::prelude::*;

entry_defs![Estimate::entry_def()];

#[hdk_entry(id = "book")]
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
            header:
                SignedHashed {
                    hashed:
                        HoloHashed {
                            content: header, ..
                        },
                    ..
                },
            entry,
        } => match header.app_entry_type() {
            Some(app_entry_type) => {
                let this_zome = zome_info()?;
                let matches_entry_def =
                    this_zome.matches_entry_def_id(app_entry_type, Estimate::entry_def_id());

                if matches_entry_def {
                    let estimate = Estimate::try_from(&entry)?;

                    let value: u8 = estimate.value;
                    if is_estimate_invalid(value) {
                        return Ok(ValidateCallbackResult::Invalid(
                            "No a correct value".to_string(),
                        ));
                    }
                    return Ok(ValidateCallbackResult::Valid);
                } else {
                    Ok(ValidateCallbackResult::Invalid(format!(
                        "Not a Estimate but a {:?}",
                        header.entry_type()
                    )))
                }
            }
            None => Ok(ValidateCallbackResult::Valid),
        },
        _ => Ok(ValidateCallbackResult::Valid),
    }
}

pub fn is_estimate_invalid(input: u8) -> bool {
    let allowed_estimates: Vec<u8> = vec![0, 1, 2, 3, 5, 8, 13, 20];
    let estimate: Estimate = Estimate {
        item: "workitem".into(),
        value: input,
    };
    let estimate_option: Option<Estimate> = Some(estimate);
    println!("here");
    match estimate_option {
        Some(x) => {
            println!("some");
            let e: Estimate = Some(x).unwrap();
            let value: u8 = e.value;
            for i in &allowed_estimates {
                if *i == value {
                    return false;
                }
            }
            return true;
        }
        None => {
            println!("none");
            return true;
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_invalid_true() {
        assert_eq!(super::is_estimate_invalid(6), true);
    }

    #[test]
    fn test_invalid_false() {
        assert_eq!(super::is_estimate_invalid(8), false);
    }
}
