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

use std::io;
use rand::Rng;

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

pub fn handle_who_am_I() -> JsonString {
    
    return json!(HashString::new()).into();
    
}


pub fn handle_set_handle(handle: HashString) -> JsonString {
    return json!(HashString::new()).into();
}


// returns all the handles in the directory
pub fn handle_get_handles() -> JsonString {
    return "TEST".into();
}

// returns the handle of an agent by looking it up on the user's DHT entry, the last handle will be the current one?
pub fn handle_get_handle(handle: HashString) -> JsonString {
    return HashString::new().into();
}

pub fn handle_get_my_handle() -> JsonString { 
    return HashString::new().into();
}

// gets the AgentID (userAddress) based on handle
pub fn handle_get_agent(handle: HashString) -> JsonString {
    return Address::new().into();
}

pub fn handle_get_toss_history() -> JsonString {
        return HashString::new().into();
}

/*
/ pub fn handle_request_toss()
/ Request the toss - initiating the game by doing the first seed commit and sending the request to the agent through gossip (?)
*/
pub fn handle_request_toss(toss: toss_schema) -> JsonString {

    let seed = rand::thread_rng().gen_range(0, 10);
    println!("Generated seed: {}", seed);
    handle_commit_seed(seed);
    
    return HashString::new().into();
}

// Commit functions - should they be a part of the zome? Or private? Or both?
fn handle_confirm_seed() -> JsonString {
        return HashString::new().into();
}

fn handle_commit_toss() -> JsonString {
    return HashString::new().into();
}

fn handle_confirm_toss(toss: toss_schema) -> JsonString {
    return HashString::new().into();
}


/*
/ fn commit_seed()
/ return: ???
*/
fn handle_commit_seed(seed: i32) -> JsonString {

    // Validate if 9 <= seed >= 0 
    // Generate salt
    // Hash the salt + seed string (?)

    // Commit seed to own chain
    // Return 
    return HashString::new().into();
}

fn generate_salt() -> JsonString {
    return HashString::new().into();
}




define_zome! {
    entries: [
        
        // Entry: "handle" for __________? The player?
        entry!(
            name: "handle",
            description: "",
            sharing: Sharing::Public,
            native_type: HashString,  // Q: Or Hash? Or Json? Or JsonString?
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
             },
            validation: |_address: Address, _ctx: hdk::ValidationData| { Ok(()) }
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
            sharing: Sharing::Public,
            native_type: toss_schema, // Q: Or? Json? JsonString?
            validation_package: || { 
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_address: Address, _ctx: hdk::ValidationData| { Ok(()) }
        ),

        // Entry: 
        entry!(
            name: "toss_result",
            description: "",
            sharing: Sharing::Public,
            native_type: toss_result_schema, // Q: Or?
            validation_package: || { 
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_address: Address, _ctx: hdk::ValidationData| { Ok(()) }
        ),

        entry!(
            name: "seed",
            description: "",
            sharing: Sharing::Private,
            native_type: i32, 
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
             },
            validation: |_address: Address, _ctx: hdk::ValidationData| { Ok(()) }
        )

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
				outputs: |result: JsonString|,      // Q: Not sure about the return type. HashString? Or everything here JsonString?
				handler: handle_who_am_I            // Q: If everything is expected to be JsonString, why ask the type at all - verbose?
			}
    		set_handle: {
				inputs: |handle: HashString|,
				outputs: |result: JsonString|,
				handler: handle_set_handle
			}
            get_handles: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: handle_get_handles
			}
            get_handle: {
				inputs: |agentKey: HashString|,
				outputs: |result: JsonString|,
				handler: handle_get_handle
			}
            get_my_handle: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: handle_get_my_handle
			}
            get_agent: {
				inputs: |handle: HashString|,
				outputs: |result: JsonString|,
				handler: handle_get_agent
			}
            request_toss: {
				inputs: |request: toss_schema |,
				outputs: |result: JsonString|,
				handler: handle_request_toss
			}
            confirm_toss: {
				inputs: |toss: toss_schema|,
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

