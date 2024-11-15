
**What to expect:**
The game loads the first JSON game_state_0 and instantiates the four creatures. 
On click the next file is loaded and the positions are updated according to game_state_1. The first creature does not change it's position. 
Click again and the creatures move again. 

It is simulating the process of receiving updated game files. 


- **Game Initialization:**
    
    - The `Main` scene is loaded, and the `_ready()` function in `Main.gd` is called.
    - The game state for `current_turn` (initially 0) is loaded using `load_game_state(current_turn)`.
- **Loading Game State:**
    
    - The `load_game_state(turn)` function constructs the file path for the JSON file corresponding to the current turn.
    - It reads and parses the JSON file into the `game_state` dictionary.
    - Validates the presence of `"creatures"` and `"turn"` keys.
    - Updates `current_turn` with the value from the JSON data.
    - Calls `update_creatures(game_state["creatures"])` to process the creature data.
- **Updating Creatures:**
    
    - The `update_creatures(creature_list)` function iterates over each creature in the provided list.
    - For each creature:
        - Checks if the creature already exists in the `creatures` dictionary.
        - If it exists and the `changed` flag is `true`, it updates the creature by calling `creature.setup(creature_data)`.
        - If it doesn't exist, it creates a new instance of the `Creature` scene, adds it to the scene tree, and calls `new_creature.setup(creature_data)`.
        - Stores the creature instance in the `creatures` dictionary with its `id` as the key.
- **Creature Setup and Visual Update:**
    
    - The `setup(json_data)` function in `Creature.gd` initializes the creature's data using `CreatureData.from_dictionary(json_data)`.
    - Sets the creature's position based on `position_data`.
    - Calls `update_visuals()` to refresh the health bar and sprite color.
- **Advancing Turns:**
    
    - When the user presses the `Button`, the `_on_Button_pressed()` function is called.
    - Increments `current_turn` by 1 and calls `load_game_state(current_turn)` to load the next turn's game state.
    - The process of updating creatures repeats with the new data.


**Additional Review:**

- Data Structure is just a placeholder regarding names and types. 
- I always use IDs for sprites. In this case 0-3 could be the creatures from player 1 and 4-7 from Player 2. 
- Creatures have a 'changed' item for faster processing but we are not talking about a major improvement here. 
- I added the turn number to the filename and the JSON object. Of course we can simplify this. 
- It is expected that if the server delivers a valid JSON object that the contents will be correct. 
	We can talk about how to inform the server if the received file is corrupted or the contents do not match the expected format/types. 




