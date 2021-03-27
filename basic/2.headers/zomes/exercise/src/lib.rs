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
    unimplemented!
}

#[hdk_extern]
pub fn get_label(hash: HeaderHashB64) -> ExternResult<KitchenJarLabel> {
    unimplemented!()
}

#[hdk_extern]
pub fn update_label(a:KitchenJarLabelUpdate) -> ExternResult<HeaderHashB64> {
    unimplemented!()
}