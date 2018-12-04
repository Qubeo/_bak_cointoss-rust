#![feature(try_from)]
use std::convert::TryFrom;
#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate holochain_core_types_derive;
#[macro_use]
extern crate serde_json;

// see https://developer.holochain.org/api/0.0.2/hdk/ for info on using the hdk library

use hdk::holochain_core_types::{
    hash::HashString,
    error::HolochainError,
    entry::Entry,
    dna::zome::entry_types::Sharing,
    entry::entry_type::EntryType,
    json::JsonString,
    cas::content::Address
};

// Q: Include the "toss.rs" module? Or?
// mod toss;

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct toss_schema {
    pub initiator: Address,          // Q: Or should this be a JSON? Or json_serde? .into()?
    pub initiatorSeedHash: HashString, 
    pub responder: Address,         // Q: Or? Shouldn't be hdk's hash::HashString, cas::content::Address or something like that?
    pub responderSeedHash: HashString,
    pub call: bool                  // Q: What the heck is this?
    // pub required: ["initiator", "initiatorSeedHash", "responder", "responderSeedHash"]; // Q: How to initialize the field?
    
    /*
        "title": "Toss Schema",
        "type": "object",
        "properties": {
            "initiator": {
                "type": "string"
            },
            "initiatorSeedHash": {
                "type": "string"
            },
            "responder": {
                "type": "string"
            },
            "responderSeedHash": {
                "type": "string"
            },
            "call": {
                "type": "boolean"
            }
        },
        "required": [
            "initiator",
            "initiatorSeedHash",
            "responder",
            "responderSeedHash"
        ]
    */
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct toss_result_schema {
    pub toss: toss_schema, //Q: Or &toss_schema? Or string, as in the original JS version?
	pub result: String,
    pub timeStamp: String
    // pub required:  ["toss","result","timeStamp"] // Q: Validation rules?
}

// -------------------------------------- TOSS FUNCTIONS ------------------------------------------
// var me = App.Key.Hash ?? // Where does this belong? And what type is it? HashString?

pub fn handle_who_am_I() -> HashString {};

pub fn set_handle(handle: HashString) -> {};

// returns all the handles in the directory
pub fn handle_get_handles() -> {};

// returns the handle of an agent by looking it up on the user's DHT entry, the last handle will be the current one?
pub fn handle_get_handle(handle: HashString) -> HashString {};

pub fn handle_get_my_handle() -> {};

// gets the AgentID (userAddress) based on handle
pub fn handle_get_agent(handle: HashString) -> Address {};

pub fn handle_request_toss() -> {};

pub fn handle_confirm_toss() -> {};

// Commit ???
// pub fn handle_commit_seed() -> {};
// pub fn handle_confirm_seed() -> {};


define_zome! {
    entries: [
        
        // Entry: "handle" for __________? The player?
        entry!(
            name: "handle",
            description: "",
            native_type: HashString,  // Q: Or Hash? Or Json? Or JsonString?
            sharing: Sharing::Public,
            validation_package: || { },
            validation: || { Ok(()) }
        ),

        /* Entry: ???
        entry!(
            name: "handle_links",
            native_type:
        ), */

        /* Entry: ???
        entry!(
            name: "directory_links",
            native_type:
        ), */

        // Entry: 
        entry!(
            name: "toss",
            description: "",
            native_type: toss_schema; // Q: Or? Json? JsonString?
            sharing: Sharing::Public,
            validation_package: || { },
            validation: || { Ok(()) }
        ),

        // Entry: 
        entry!(
            name: "toss_result",
            description: "",
            native_type: toss_result_schema; // Q: Or?
            sharing: Sharing::Public,
            validation_package: || { },
            validation: || { Ok(()) }
        ),

        entry!(
            name: "seed",
            description: "",
            native_type: , 
            sharing: Sharing::Private,
            validation_package: || { },
            validation: || { Ok(()) }
        ),

        /* Entry: ??
        entry!(
            name: "history_link_base",
            native_type:
            sharing: Sharing::Public,
            validation_package: || { },
            validation: || {}
        ), */

        /* Entry: ??
        entry!(
            name: "history_links",
            native_type: links 
        ) */

    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
			who_am_I: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: handle_who_am_I
			}
			set_handle: {
				inputs: |handle: String|,
				outputs: |result: JsonString|,
				handler: handle_set_handle
			}
            get_handles: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: handle_get_handles
			}
            get_handle: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: handle_get_handle
			}
            get_my_handle: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: handle_get_my_handle
			}
            get_agent: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: handle_get_agent
			}
            request_toss: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: handle_request_toss
			}
            confirm_toss: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: handle_confirm_toss
			}
            get_toss_history: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: handle_get_toss_history
			}            
       }
    }
}

