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
use std::fmt;
// use snowflake;
use hdk::holochain_core_types::{ // HDK library: https://developer.holochain.org/api/0.0.2/hdk/
    hash::HashString,
    error::HolochainError,
    entry::Entry,
    dna::zome::entry_types::Sharing,
    entry::entry_type::EntryType,
    json::{ JsonString, RawString },
    cas::content::Address,
};
use hdk::api::AGENT_ADDRESS;
mod entries;
use crate::entries::{CTEntryType, TossSchema, TossResultSchema, SeedSchema};


// -------------------------------------- TOSS FUNCTIONS ------------------------------------------
// var me = App.Key.Hash ?? // Q: Where does this belong? And what type is it? HashString?

pub fn handle_get_my_address() -> JsonString {
  
    // AGENT_ADDRESS.clone().into();

    // TODO: Not fully implemented AGENT_ADDRESS in the current HDK release yet.
    // Temporary workaround idea: use a random hash? Or hash agent key? Where do I get it? 
    // TODO: VERY temporary - just returning a hard-coded HashString now.    
    return "prdel".into();
}


pub fn handle_set_handle(_handle: String) -> JsonString {
   
    // let handle_hash = HashString::encode_from_str(&_handle, Hash::SHA2256);

    // Q: It works with the JsonString(RawString) wrapping. How come?
    // What are the allowed forms for the argument? Can I see the memory / byte structure somewhere?    
    let raw_handle = JsonString::from(RawString::from(_handle.clone()));

    // TODO: Napsat návrh na "formatted" hdk::debug! macro?
    hdk::debug("handle_set_handle()::_handle: ");
    hdk::debug(raw_handle.clone());
   
    let handle_entry = Entry::new(EntryType::App(CTEntryType::handle.to_string()), raw_handle); // Q: my_key? &my_key? Nebo "prdel"?
    hdk::debug(handle_entry.to_string());
    
    // Q: It seems having this in genesis doesn't work - throws an exception within the holochain-nodejs. How to?
    let handle_address: JsonString = match hdk::commit_entry(&handle_entry) {

        // Ok(address) => match hdk::link_entries(&AGENT_ADDRESS, &address, "handle") {
            Ok(address) => json!({ "address": address }).into(),
            Err(hdk_err) => { hdk_err.into() }
        // },
        // Err(hdk_err) => hdk_err.into()
    };
    
    // let my_key_entry_address = match hdk::get_entry(hdk::entry_address(&my_key_entry)) {
    hdk::debug(handle_address.clone());

    return handle_address;     
}

    /*
     var handles = getLinks(App.Key.Hash, 'handle');
  if (handles.length > 0)
  {
    if (anchorExists('handle', handle) === 'false') {
      var oldKey = handles[0].Hash;
      var key = update('handle', anchor('handle', handle), oldKey);
      commit('handle_links', {
        Links: [
          {
            Base: App.Key.Hash,
            Link: oldKey,
            Tag: 'handle',
            LinkAction: HC.LinkAction.Del
          },
          { Base: App.Key.Hash, Link: key, Tag: 'handle' }
        ]
      });

      commit('directory_links', {
        Links: [
          {
            Base: App.DNA.Hash,
            Link: oldKey,
            Tag: 'handle',
            LinkAction: HC.LinkAction.Del
          },
          { Base: App.DNA.Hash, Link: key, Tag: 'handle' }
        ]
      });
      return key;
    }
    else {
      // debug('HandleInUse')
      return 'HandleInUse';
    }
  }

  if (anchorExists('handle', handle) === 'false') {
    var newHandleKey = commit('handle', anchor('handle', handle));
    commit('handle_links', {
      Links: [{ Base: App.Key.Hash, Link: newHandleKey, Tag: 'handle' }]
    });
    commit('directory_links', {
      Links: [{ Base: App.DNA.Hash, Link: newHandleKey, Tag: 'directory' }]
    });
    return newHandleKey;
  } else {
    // debug('HandleInUse')
    return 'HandleInUse';
  } */



// returns all the handles in the directory
pub fn handle_get_handles() -> JsonString {

    // TODO: Just finding out what this does. Copied from the HoloChat to test it.
    // It doesn't work here. How come it works in the HoloChat?? (Or does it?)
    return "prdelgethandle".into();
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

/*
/ pub fn handle_request_toss()
/ Request the toss - initiating the game by doing the first seed commit and sending the request to the agent through gossip (?)
*/

// TODO: This is wrong, I believe. The argument should be the agent (address? key? handle?), toss should be the output.
pub fn handle_request_toss(_agent_key: Address) -> JsonString {
    
    // TODO: Just a rough random salt and seed. Change to sth more secure.
    let seed = SeedSchema {
        salt: rand::thread_rng().gen_range(0, 10).to_string(),
        seed_value: rand::thread_rng().gen_range(0, 10)
    };


    hdk::debug("Generated seed: ");
    hdk::debug(seed.seed_value.to_string());

    let seed_entry = handle_commit_seed(seed);

    // Q: Can I call gossip functions from here? If yes, how? Or should I do it from the outside of the container?
    // TODO: Reconsider the design when I get the info. For now, passing the commited seed address to the JS.
    
    /*
    let toss = TossSchema {
        initiator: &AGENT_ADDRESS,
        initiator_seed_hash: seed_entry,
        responder: _agent_key,
        responder_seed_hash: 
    };
    */

    return seed_entry.into();
}

pub fn handle_receive_request(_agent_key: Address, seed_hash: HashString) -> JsonString {

    hdk::debug("handle_receive_request() agent_key:");
    hdk::debug(_agent_key);

    return seed_hash.into();
}

pub fn handle_get_toss_history() -> JsonString {
        
        let prdel = "prdel".to_string();
        let prdel_hash = HashString::encode_from_str(&prdel.clone(), Hash::SHA2256);

        let toss = TossSchema {
            initiator:  prdel_hash.clone(),
            initiator_seed_hash: prdel_hash.clone(),
            responder: prdel_hash.clone(),
            responder_seed_hash: prdel_hash.clone(),
            call: true
        };
        
        return json!(prdel_hash).into();
}

fn handle_confirm_toss(_toss: TossSchema) -> JsonString {
// fn handle_confirm_toss(_toss: JsonString) -> JsonString {
  
    hdk::debug("_toss: ");
    hdk::debug(_toss);
    
  /*  let toss_entry = Entry::new(EntryType::App("toss".into()), _toss); // Q: my_key? &my_key? Nebo "prdel"?
    

    // Q: It seems having this in genesis doesn't work - throws an exception within the holochain-nodejs.
    // TODO: Ask in Mattermost.
    let toss_address: JsonString = match hdk::commit_entry(&toss_entry) {
         Ok(address) => match hdk::link_entries(&AGENT_ADDRESS, &address, "toss") {
            Ok(address) => json!({ "address": address }).into(),
            Err(hdk_err) => { hdk_err.into() }
         },
         Err(hdk_err) => hdk_err.into()
    };*/
    
    return json!("prdel").into(); //toss_address.into();
}


// Commit functions - should they be a part of the zome? Or private? Or both?

/*
/ fn commit_seed()
/ return: ???
*/
fn handle_commit_seed(_seed: SeedSchema) -> JsonString {

    // Validate if 9 <= seed >= 0 
    // Generate salt
    // Hash the salt + seed string (?)
    // Commit seed to own chain
    // Return 

    //let entry_arg = JsonString::from(RawString::from(_seed));
    //hdk::debug("Raw seed: ");
    //hdk::debug(entry_arg.clone());

    let seed_entry = Entry::new(EntryType::App(CTEntryType::seed.to_string()), _seed);
    hdk::debug(seed_entry.to_string());
    
    let seed_address: JsonString = match hdk::commit_entry(&seed_entry) {

        // Ok(address) => match hdk::link_entries(&AGENT_ADDRESS, &address, "seeds") {
            Ok(address) => json!({ "address": address }).into(),
            Err(hdk_err) => { hdk_err.into() }
        // },
        // Err(hdk_err) => hdk_err.into()
    };

    return seed_address.into();
}

fn confirm_seed() -> JsonString {
    return HashString::new().into();
}

fn commit_toss(_toss: TossSchema) -> JsonString {   
    return HashString::new().into();
}

fn generate_salt() -> JsonString {
    return HashString::new().into();
}


define_zome! {
    entries: [

        entries::handle_definition(),
        entries::toss_definition(),
        entries::toss_result_definition(),
        entries::seed_definition()

        // TODO: Q: It seems I can define multiple entries of the same type / content. Isn't this a bug?

       /* Q: Link entries. What to do with those?
        entry!(
            name: "handle_links",
            native_type:
        ),
        entry!(
            name: "directory_links",
            native_type:
        ), 
        entry!(
            name: "history_link_base",
            native_type:
            sharing: Sharing::Public,
            validation_package: || { },
            validation: || {}
        ),
        entry!(
            name: "history_links",
            native_type: links 
        ) */
    ]

    genesis: || {
        
         // TODO workaround around not-yet-implemented hdk::api::AGENT_ADDRESS
         // Commit a tomporarily created agent hash to my chain and return the entry address?
            {
                //handle_set_handle(&AGENT_ADDRESS);
                Ok(())
            }
         }

    functions: {
        main (Public) {
			get_my_address: {
				inputs: | |,
				outputs: |result: JsonString|,      // Q: Not sure about the return type. HashString? Or everything here JsonString?
				handler: handle_get_my_address            // Q: If everything is expected to be JsonString, why ask the type at all - verbose?
			}
    		set_handle: {
				inputs: |handle: String|,
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
				inputs: |agent_key: Address|,
				outputs: |result: JsonString|,
				handler: handle_request_toss
			}
            receive_request: {
                inputs: |agent_key: Address, seed_hash: HashString|,    // TODO: He should probably read it automatically from the message sender. How? Gossip?
                outputs: |result: JsonString|,
                handler: handle_receive_request
            }
            confirm_toss: {
				inputs: |toss: entries::TossSchema|,
				outputs: |result: JsonString|,
				handler: handle_confirm_toss
			}
            get_toss_history: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: handle_get_toss_history
			}
            commit_seed: {
                inputs: |seed: entries::SeedSchema|,
                outputs: |result: JsonString|,
                handler: handle_commit_seed
            }            
       }
    }
}

