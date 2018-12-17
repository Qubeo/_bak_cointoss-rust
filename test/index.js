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
const app_a = Container.loadAndInstantiate("dist/bundle.json");
const app_b = Container.loadAndInstantiate("dist/bundle.json");

// activate the new instance
app_a.start();
app_b.start();

const handle_player_a = "prdelA";
const handle_player_b = "prdelB";

test('Get my address', (t) => {

  const result_a = app_a.call("cointoss", "main", "get_my_address", {});  
  const result_b = app_b.call("cointoss", "main", "get_my_address", {});  
  console.log("Address A, Address B: ");
  console.log(result_a, result_b);

  //t.deepEqual(result_a, "QmeQPvoUwXXskAJtyBUNPX7ks8MoazmcSvKnvtYTVrBGNM")
  t.end()
})

test('Call the set_handle() function, expect entry address as a result', (t) => {

  const result_a = app_a.call("cointoss", "main", "set_handle", { handle: handle_player_a });
  const result_b = app_b.call("cointoss", "main", "set_handle", { handle: handle_player_b });

  console.log("set_handle() result: ");
  console.log(result_a, result_b);

  // t.equal(result);
  t.end();
})

test('Initiate a toss by calling request_toss()'), (t) => {
  
  const result = app_a.call("cointoss", "main", "request_toss", {});
  console.log(result);
  t.end();
}

test('Commit a seed and return the entry address', (t) => {

  const seed_schema = { salt: "prdel", seed_value: 22 };
  const result = app_a.call("cointoss", "main", "commit_seed", { seed: seed_schema });

  console.log("commit_seed() result: ");
  console.log(result);

  t.end();
})


// Misc learning bits:
// Decode from bs58 to hex, slice the leading 2 bytes, encode back to bs58
// var recoded_result = bs58.encode(Buffer.from(bs58.decode(result)).slice(2));
// var hashed_key1 = shajs('sha256').update(key1).digest();
// const b58_prdel = bs58.encode(Buffer.from(hashed_key1));  
   