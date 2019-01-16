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
holochain-nodejs expets JsonString as a message result, giving out JSON.parse error, when it shouldn't.

**Learnings**




**Resources**