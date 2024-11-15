# [Calculate] Module
This handles the calculation stage of a committed decision. 

## Single Player
Simply involves sending the [[Decision]] to the local API, which then calculates the aftermath. 

## Multi-Player
First, a [[Random Number]](! TODO !) must be requested from the server. This could have probably already been agreed upon before starting, or asynchronously to prevent latency. Then, the information along with the [[Decision]] is sent to the local API and server API. The local API runs the calculations and performs them instantly. Then, a hash of the game state is created. Once the server has finalized the calculation the same hash is calculated and passed.

If the hash values are invalid, the client must request a full re-sync with the server to determine what went wrong, and to adjust to the new state.

After validation, the server sends the other player the new state to run the [[Decision]] made alongside the validated updates.

---

During this time, neither players have any options besides reviewing their current fighters and abilities. This can be considered a "Locked" phase for both players.

It's essential that this phase must never enter "Hard-Locks", causing any, or both players to be completely locked out of the game. 

---------------------------


**Multi-Player POC-Implementation (simplified process flow):**
Clients get [[Decision]] , immediately passes it to other client (via server).
Both clients update state, calculate hash and send it to server.
Server compares hashes, approves the new state (State-Approval) and initiates next gameplay turn, else (State-Disapproval) which could be a simple exit at this point in time. 

Further development: 
- last approved state is stored by the clients with the corresponding hash
- in case of a State-Disapproval the server informs the clients to get the last valid state and calculate the effects of the decision again (re-enter standard process). If it fails again, exit. 

