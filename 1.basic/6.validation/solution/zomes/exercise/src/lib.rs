use hdk::prelude::*;
use holo_hash::HeaderHashB64;

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
fn validate(validation_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let element: Element = validation_data.element;
    let estimate_option: Option<Estimate> = element.entry().to_app_option()?;
    match estimate_option {
        None => {
            return Ok(ValidateCallbackResult::Invalid("Empty estimate not allowed".to_string()))
        }
        Some(x) => {
            let e: Estimate = Some(x).unwrap();
            let value:u8 = e.value;
            if is_estimate_invalid(value){
                return Ok(ValidateCallbackResult::Invalid("No a correct value".to_string()))
            }
            return Ok(ValidateCallbackResult::Valid)
        },
    };
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
            let value:u8 = e.value;
            for i in &allowed_estimates {
                if *i == value {
                    return false
                }
            }
            return true
        },
        None => { println!("none"); return true }
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