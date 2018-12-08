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
#[macro_use]
extern crate log;

extern crate multihash;

use std::io;
use rand::Rng;
use multihash::{encode, decode, Hash};

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
pub struct TossSchema {
    pub initiator: Address,          // Q: Or should this be a JSON? Or json_serde? .into()?
    pub initiator_seed_hash: HashString, 
    pub responder: Address,         // Q: Or? Shouldn't be hdk's hash::HashString, cas::content::Address or something like that?
    pub responder_seed_hash: HashString,
    pub call: bool                  // Q: What the heck is this?
    // pub required: ["initiator", "initiator_seed_hash", "responder", "responder_seed_hash"]; // Q: How to initialize the field?
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct TossResultSchema {
    pub toss: TossSchema, //Q: Or &TossSchema? Or string, as in the original JS version?
	pub result: String,    // Q: What format?
    pub time_stamp: String
    // pub required:  ["toss","result","time_stamp"] // Q: Validation rules?
}

// -------------------------------------- TOSS FUNCTIONS ------------------------------------------
// var me = App.Key.Hash ?? // Where does this belong? And what type is it? HashString?

pub fn handle_who_am_I() -> JsonString {
    
    // TODO: Not fully implemented AGENT_ADDRESS in the current HDK release yet.
    // Temporary workaround idea: use a random hash? Or hash agent key? Where do I get it?
 
    // TODO: VERY temporary - just returning a hard-coded HashString now.
    return set_my_key().into();
}


pub fn handle_set_handle(_handle: HashString) -> JsonString {
    return json!(HashString::new()).into();
}


// returns all the handles in the directory
pub fn handle_get_handles() -> JsonString {
    return "TEST".into();
}

// returns the handle of an agent by looking it up on the user's DHT entry, the last handle will be the current one?
pub fn handle_get_handle(_handle: HashString) -> JsonString {
    return HashString::new().into();
}

pub fn handle_get_my_handle() -> JsonString { 
    return HashString::new().into();
}

// gets the AgentID (userAddress) based on handle
pub fn handle_get_agent(_handle: HashString) -> JsonString {
    return Address::new().into();
}

pub fn handle_get_toss_history() -> JsonString {
        return HashString::new().into();
}

/*
/ pub fn handle_request_toss()
/ Request the toss - initiating the game by doing the first seed commit and sending the request to the agent through gossip (?)
*/
pub fn handle_request_toss(_toss: TossSchema) -> JsonString {

    let seed = rand::thread_rng().gen_range(0, 10);
    println!("Generated seed: {}", seed);
    commit_seed(seed);
    
    return HashString::new().into();
}

fn handle_confirm_toss(_toss: TossSchema) -> JsonString {
    return HashString::new().into();
}

fn set_my_key() -> HashString {

    //let mut hasher = DefaultHasher::new();

    let my_key = HashString::encode_from_str("prdel", Hash::SHA2256);
    hdk::debug("my_key: ");
    hdk::debug(&my_key);

    return my_key; 
}

// Commit functions - should they be a part of the zome? Or private? Or both?

/*
/ fn commit_seed()
/ return: ???
*/
fn commit_seed(_seed: i32) -> JsonString {

    // Validate if 9 <= seed >= 0 
    // Generate salt
    // Hash the salt + seed string (?)

    // Commit seed to own chain
    // Return 
    return HashString::new().into();
}

fn confirm_seed() -> JsonString {
        return HashString::new().into();
}

fn commit_toss() -> JsonString {
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
            native_type: TossSchema, // Q: Or? Json? JsonString?
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
            native_type: TossResultSchema, // Q: Or?
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

    genesis: || {
        
         // TODO workaround around not-yet-implemented hdk::api::AGENT_ADDRESS
         // Commit a tomporarily created agent hash to my chain and return the entry address?
            {
                // set_my_key();        
                Ok(())
            }
         }

    functions: {
        main (Public) {
			who_am_I: {
				inputs: | |,
				outputs: |result: JsonString|,      // Q: Not sure about the return type. HashString? Or everything here JsonString?
				handler: handle_who_am_I            // Q: If everything is expected to be JsonString, why ask the type at all - verbose?
			}
    		set_handle: {
				inputs: |handle: HashString|,
				outputs: |result: JsonString|,      // Q: How does this syntax work? Closure arguments without follow up function body? :o
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
				inputs: |request: TossSchema |,
				outputs: |result: JsonString|,
				handler: handle_request_toss
			}
            confirm_toss: {
				inputs: |toss: TossSchema|,
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

