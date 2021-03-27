use hdk::prelude::*;
use holo_hash::HeaderHashB64;

entry_defs![KitchenJarLabel::entry_def(), KitchenJarLabelUpdate::entry_def()];

#[hdk_entry(id = "KitchenJarLabel")]
pub struct KitchenJarLabel(String);

#[hdk_entry(id = "KitchenJarLabelUpdate")]
pub struct KitchenJarLabelUpdate{
    label: String,
    header_hash: HeaderHashB64,
}

#[hdk_extern]
pub fn add_label(input: KitchenJarLabel) -> ExternResult<HeaderHashB64> {
    let a: HeaderHash = create_entry(&input)?;
    Ok(HeaderHashB64::from(a))
}

#[hdk_extern]
pub fn get_label(hash: HeaderHashB64) -> ExternResult<KitchenJarLabel> {
    let element: Element = get(HeaderHash::from(hash), GetOptions::default())?
        .ok_or(WasmError::Guest(String::from("Could not find KitchenJarLabel")))?;
    let option: Option<KitchenJarLabel> = element.entry().to_app_option()?;
    let label: KitchenJarLabel = option.unwrap();
    Ok(label)
}

#[hdk_extern]
pub fn update_label(a:KitchenJarLabelUpdate) -> ExternResult<HeaderHashB64> {
    let header_hash: HeaderHash = HeaderHash::from(a.header_hash);
    let new_label: KitchenJarLabel = KitchenJarLabel(a.label);
    let header_hash_update: HeaderHash = update_entry(header_hash, new_label)?;
    Ok(HeaderHashB64::from(header_hash_update))   
}