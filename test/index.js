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
const app = Container.loadAndInstantiate("dist/bundle.json");
//const app2 = Container.loadAndInstantiate("dist/bundle.json");

// activate the new instance
app.start();

test('Calls the set_handle() function, expects entry address as a result', (t) => {

  const handle1 = "prdel1";
  const result = app.call("cointoss", "main", "set_handle", { handle: handle1 });

  console.log("set_handle() result: ");
  console.log(result);

  // t.equal(result);
  t.end();
})

test('Commit a seed and return the entry address', (t) => {

  const seed_schema = { seed: 22 };
  const result = app.call("cointoss", "main", "commit_seed", { seed: seed_schema });

  console.log("commit_seed() result: ");
  console.log(result);

  t.end();
})


// Misc learning bits:
// Decode from bs58 to hex, slice the leading 2 bytes, encode back to bs58
// var recoded_result = bs58.encode(Buffer.from(bs58.decode(result)).slice(2));
// var hashed_key1 = shajs('sha256').update(key1).digest();
// const b58_prdel = bs58.encode(Buffer.from(hashed_key1));  
   