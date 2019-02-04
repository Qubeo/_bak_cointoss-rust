/*use hdk::{
    self,
    entry_definition::{
        ValidatingEntryType,
        ValidatingLinkDefinition,
    },
    error::{ ZomeApiError, ZomeApiResult },
    holochain_core_types::{
        cas::content::Address,
        dna::entry_types::Sharing,
        entry::{ entry_type::EntryType, Entry },
        error::HolochainError,
        json::JsonString,
        hash::HashString
    },
    AGENT_ADDRESS,
}; */

use hdk::{
    self,
    entry_definition::{
        ValidatingEntryType,
        ValidatingLinkDefinition,
    },
    error::ZomeApiResult,
    holochain_core_types::{
        cas::content::Address, entry::Entry, dna::entry_types::Sharing, error::HolochainError, json::JsonString, hash::HashString
    },
    holochain_wasm_utils::api_serialization::{
        get_entry::GetEntryOptions, get_links::GetLinksResult,
    },
    AGENT_ADDRESS,
};

use std::convert::TryFrom;
use std::fmt;

#[derive(Debug)]
pub enum CTEntryType {
    handle,
    seed,
    toss,
    toss_result
}

// Learning: Playing with an alternative to "entry_type".into() for creating entries
// Q: Wouldn't CTEntryName or CTEntry be more appropriate?
// Q: How to integrate this better with the entry! macro?
// Q: How to automatically convert the value into string, without needing to use value.to_string()?
impl fmt::Display for CTEntryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct TossSchema {
    pub initiator: Address,
    pub initiator_seed_hash: HashString,
    pub responder: Address,
    pub responder_seed_hash: HashString,
    pub call: bool
    // pub required: ["initiator", "initiator_seed_hash", "responder", "responder_seed_hash"]; // Q: How to initialize the field?
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct HandleSchema {
    pub handle: String
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct TossResultSchema {
    pub toss: TossSchema,       
	pub result: String,         // Q: What format?
    pub time_stamp: String
    // pub required:  ["toss","result","time_stamp"] // Q: Validation rules?
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct SeedSchema {
    pub salt: String,    
    pub seed_value: u8
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct AddrSchema {
    pub key: String
}



//-----------------------------------------------------------------------------
//                            Entry definitions
//-----------------------------------------------------------------------------

pub fn handle_definition() -> ValidatingEntryType {
        
    // Entry: "handle" for __________? The player? 
    entry!(
        name: "handle",
        description: "",
        sharing: Sharing::Public,
        native_type: HandleSchema,                                // Q: Why does String, or even JsonString not work any more?
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |handle: HandleSchema, _validation_data: hdk::ValidationData| { Ok(()) }
    )
} 

pub fn seed_definition() -> ValidatingEntryType {
    entry!(
        name: "seed",
        description: "",
        sharing: Sharing::Private,
        native_type: SeedSchema, 
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |seed: SeedSchema, _validation_data: hdk::ValidationData| { Ok(()) },
        links: [
            from!(
                "%agent_id",
                tag: "agent",
                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },
                validation: |source: Address, _target: Address, _validation_data: hdk::ValidationData| {
                    Ok(())
                })
            ]
    )
}

pub fn toss_definition() -> ValidatingEntryType {
    entry!(
        name: "toss",    // Learning: Experimenting with "enum" instead of hardcoded string
        description: "",
        sharing: Sharing::Public,
        native_type: TossSchema, // Q: Or? Json? JsonString?
        validation_package: || { 
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |toss: TossSchema, _validation_data: hdk::ValidationData| { Ok(()) }
    )
}

pub fn toss_result_definition() -> ValidatingEntryType {    
    entry!(
        name: "toss_result",
        description: "",
        sharing: Sharing::Public,
        native_type: TossResultSchema, // Q: Or?
        validation_package: || { 
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |toss_result: TossResultSchema, _validation_data: hdk::ValidationData| { Ok(()) }
    )
}