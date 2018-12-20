const test = require('tape');
const tapSpec = require('tap-spec');
const shajs = require('sha.js');
const Container = require('@holochain/holochain-nodejs');
const bs58 = require('bs58');
//const hash = require('hash.js');


test.createStream()
  .pipe(tapSpec())
  .pipe(process.stdout);

// instantiate an app from the DNA JSON bundle
const app_a = Container.instanceFromNameAndDna("jakub", "dist/bundle.json");
const app_b = Container.instanceFromNameAndDna("bob", "dist/bundle.json");

// activate the new instance
app_a.start();
app_b.start();

const handle_player_a = "prdelA";
const handle_player_b = "prdelB";

var handle_address_a;
var handle_address_b;

var g_seed_hash_a;

// Q: How to get the agent address?

test('Get my address', (t) => {

  const result_a = app_a.call("cointoss", "main", "get_my_address", {});  
  const result_b = app_b.call("cointoss", "main", "get_my_address", {});  
  console.log("Address A, Address B: ");
  console.log(result_a, result_b);

  //t.deepEqual(result_a, "QmeQPvoUwXXskAJtyBUNPX7ks8MoazmcSvKnvtYTVrBGNM")
  t.end();
})

test('Call the set_handle() function, expect entry address as a result', (t) => {

  const result_a = app_a.call("cointoss", "main", "set_handle", { handle: handle_player_a });
  const result_b = app_b.call("cointoss", "main", "set_handle", { handle: handle_player_b });

  console.log("set_handle() result: ");
  console.log(result_a, result_b);

  handle_address_a = result_a;
  handle_address_b = result_b;

  // t.equal(result);
  t.end();
})

test('Initiate a toss by calling request_toss()', (t) => {
  
  console.log("Agent key: ")
  console.log(handle_address_b);

  const result_request = app_b.call("cointoss", "main", "request_toss", { agent_key: handle_address_b.address });

  console.log(result_request);
  t.end();
})

test('Commit a seed and return the entry address', (t) => {

  // Q: Where should the "salt" be generated? How much freedom for the agent? Visibility?
  const seed_schema_a = { salt: "prdel", seed_value: 22 };
  const result_request = app_a.call("cointoss", "main", "commit_seed", { seed: seed_schema_a });

  g_seed_hash_a = result_request;

  console.log("commit_seed() result: ");
  console.log(g_seed_hash_a);

  t.end();
})

test('Receive the toss request', (t) => {

  const result_receive = app_b.call("cointoss", "main", "receive_request", { agent_key: handle_address_a.address, seed_hash: g_seed_hash_a.address });

  console.log("receive_request() result: ");
  console.log(result_receive);

  t.end();
})


// Misc learning bits:
// Decode from bs58 to hex, slice the leading 2 bytes, encode back to bs58
// var recoded_result = bs58.encode(Buffer.from(bs58.decode(result)).slice(2));
// var hashed_key1 = shajs('sha256').update(key1).digest();
// const b58_prdel = bs58.encode(Buffer.from(hashed_key1));  
   