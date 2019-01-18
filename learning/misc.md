**TODO**

**Implementation ideas**
When agent B receives the toss request, he could get notified (how?) and prompted to play his move.
    If he doesn't, until a certain timeout, his hand gets played randomly and automatically.

Create a VSCode macro, copying any "//Q: " statements into this file as questions.
    Ideally even linking them and reflecting changes, ha :)


**Questions**
When defining 
In N2N messaging, how do I, as an agent, know where does the message come from??
How to decide which logic goes where? (N2N vs. zome calls etc))


**Issues**
Receive callback doesn't receive the sender address. What good are anonymous messages?
Call of the "send_message" expects the send_message to return String. Why not ZomeApiResult<String>?
    Issue#746
holochain-nodejs expets JsonS

Error: "const result_seedhash = container.callRaw("prdelA::./dist/bundle.json", "cointoss", "main", "send_message", JSON.stringify(init_message));
                                    ^unable to call zome function: InternalFailure(RibosomeFailed("Trap: Trap { kind: Unreachable }"))"
    Somethig killing my zome? In the process_received_message()?

**Learnings**




**Resources**