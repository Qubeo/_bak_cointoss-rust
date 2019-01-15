const test = require('tape');
const tapSpec = require('tap-spec');
const shajs = require('sha.js');
const {Config, Container} = require('@holochain/holochain-nodejs');
const bs58 = require('bs58');
//const hash = require('hash.js');

test.createStream()
  .pipe(tapSpec())
  .pipe(process.stdout);

const dnaPath = "./dist/bundle.json";

const name_player_A = "prdelA";
const name_player_B = "prdelB";

// IIFE to keep config-only stuff out of test scope
const container = (() => {
  const agent_A = Config.agent(name_player_A)
  const agent_B = Config.agent(name_player_B)
  
  const dna = Config.dna(dnaPath)
  
  const instance_A = Config.instance(agent_A, dna)
  const instance_B = Config.instance(agent_B, dna)
  
  const containerConfig = Config.container([instance_A, instance_B])
  return new Container(containerConfig)
})()                                          // Q: What does this syntax mean / do?
  
  // Initialize the Container
container.start()
  
const player_A = container.makeCaller(name_player_A, dnaPath)       // Q: What is this?
const player_B = container.makeCaller(name_player_B, dnaPath)
  
var handle_address_a;
var handle_address_b;

var g_seed_hash_a;
var g_received_toss;

// Q: How to get the agent address?

test('Get my address', (t) => {

  const result_a = player_A.call("cointoss", "main", "get_my_address", {});  
  const result_b = player_B.call("cointoss", "main", "get_my_address", {});  
  console.log("JS/ Address A, Address B: ");
  console.log(result_a, result_b);

  //t.deepEqual(result_a, "QmeQPvoUwXXskAJtyBUNPX7ks8MoazmcSvKnvtYTVrBGNM")
  t.end();
});

test('Call the set_handle() function, expect entry address as a result', (t) => {

  const result_a = player_A.call("cointoss", "main", "set_handle", { handle: name_player_A });
  const result_b = player_B.call("cointoss", "main", "set_handle", { handle: name_player_B });

  console.log("JS/ set_handle() result: ");
  console.log(result_a, result_b);

  handle_address_a = result_a;
  handle_address_b = result_b;

  // t.equal(result);
  t.end();
});

test('Initiate a toss by calling request_toss()', (t) => {
  
  console.log("JS/ Agent key: ")
  console.log(handle_address_b);

  const result_request = player_B.call("cointoss", "main", "request_toss", { agent_key: handle_address_b.address });

  console.log(result_request);
  t.end();
});

test('Agent A/ Commit a seed and return the entry address', (t) => {

  // Q: Where should the "salt" be generated? How much freedom for the agent? Visibility?
  const seed_schema_a = { salt: "prdel", seed_value: 22 };
  const result_request = player_A.call("cointoss", "main", "commit_seed", { seed: seed_schema_a });

  g_seed_hash_a = result_request;

  console.log("JS/ commit_seed() result: ");
  console.log(g_seed_hash_a);

  t.end();
});

test('Agent B/ Receive the toss request and commit the toss', (t) => {

  const result_receive = player_B.call("cointoss", "main", "receive_request", { agent_key: handle_address_a.address, seed_hash: g_seed_hash_a.address });
  g_received_toss = result_receive;

  console.log("JS/ receive_request() result: ");
  console.log(result_receive);

  t.end();
});

test('Agent A/ Receive the toss response, confirm the toss and commit it too', (t) => {

  const result_confirm = player_B.call("cointoss", "main", "confirm_toss", { toss: g_received_toss });

  console.log("JS/ receive_request() result: ");
  console.log(result_confirm);

  t.end();
});

test('Agent A/ reveals the result', (t) => {

  // console.log("JS/ ... ");
  t.end();
});


// Misc learning bits:
// Decode from bs58 to hex, slice the leading 2 bytes, encode back to bs58
// var recoded_result = bs58.encode(Buffer.from(bs58.decode(result)).slice(2));
// var hashed_key1 = shajs('sha256').update(key1).digest();
// const b58_prdel = bs58.encode(Buffer.from(hashed_key1));  
   