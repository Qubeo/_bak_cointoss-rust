// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const tapSpec = require('tap-spec');
//const hash = require('hash.js');
var shajs = require('sha.js');
const Container = require('@holochain/holochain-nodejs');
const bs58 = require('bs58');



test.createStream()
  .pipe(tapSpec())
  .pipe(process.stdout);

// instantiate an app from the DNA JSON bundle
const app = Container.loadAndInstantiate("dist/bundle.json");

// activate the new instance
app.start();

test('Calls the who_am_I function, expects result to be proper handle', (t) => {
  // Make a call to a Zome function
  // indicating the capability and function, and passing it an input
 
  const result = app.call("cointoss", "main", "who_am_I", {});
  console.log("Result: ");
  console.log(result);
  // TODO: What bytes does the result exactly consist of?
  // Check for equality of the actual and expected results:

  // Decode from bs58 to hex, slice the leading 2 bytes, encode back to bs58
  // Q: Why the leading 2 bytes? No idea.
  var recodedResult = bs58.encode(Buffer.from(bs58.decode(result)).slice(2));
  
  console.log("Result slice(2)-ed & recoded: ");
  console.log(recodedResult);
  
  var hashedPrdel = shajs('sha256').update('prdel').digest();

  // TODO: Vypadá to, že když dám console log s úvodním stringem, ten recoded result a hashedPrdel jsou stejné, akorát recodedResult má dva bajty navíc.
  // JAKTO?? 34b. Je to už ten result?

  const b58Prdel = bs58.encode(Buffer.from(hashedPrdel));  
  console.log("SHA256-ed & bs58-ed prdel: ");
  console.log(hashedPrdel);

  t.equal(recodedResult, b58Prdel);

  t.end();
})
