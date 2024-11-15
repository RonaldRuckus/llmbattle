# Main.gd
extends Node2D

const CreatureScene: PackedScene = preload("res://Scenes/Creature.tscn")

var creatures: Dictionary = {}
var current_turn: int = 0
var game_state: Dictionary = {}


func _ready():
	load_game_state(current_turn)


func load_game_state(turn):
	var file_path = "res://Data/game_state_%d.json" % turn
	if FileAccess.file_exists(file_path):
		var file = FileAccess.open(file_path, FileAccess.READ)
		var json_string = file.get_as_text()
		file.close()

		var json = JSON.new()
		var error = json.parse(json_string)
		if error == OK:
			var data = json.get_data()
			if data is Dictionary and data.has("creatures") and data.has("turn"):
				game_state = data
				current_turn = game_state["turn"]
				print("Game state loaded successfully for turn: ", current_turn)
				update_creatures(game_state["creatures"])
			else:
				print("Invalid JSON format: missing 'creatures' or 'turn' key.")
		else:
			print("Failed to parse JSON: ", json.get_error_message())
	else:
		print("JSON file not found at: " + file_path)


func validate_creature_data(data: Dictionary) -> bool:
	# Try to create a CreatureData instance from the dictionary
	# If any required fields are missing or of wrong type, it will fail
	if !data.has("position") or !data["position"].has("x") or !data["position"].has("y"):
		push_error("Missing or invalid position data")
		return false	
	return true


func update_creatures(creature_list):
	print("Processing creatures. List length: ", len(creature_list))
	for creature_data in creature_list:
		var creature_id = creature_data["id"]
		if creatures.has(creature_id):  # Update existing creature
			if creature_data["changed"]:  # Only update if 'changed' is true
				var creature = creatures[creature_id]
				creature.setup(creature_data)
				print("Updated creature: ", creature_data["name"])
		else:  # Create new creature if it doesn't exist
			print("Creating new creature: ", creature_data["name"])
			var creature_scene = preload("res://Scenes/Creature.tscn")
			var new_creature = creature_scene.instantiate()
			add_child(new_creature)
			new_creature.setup(creature_data)
			creatures[creature_id] = new_creature
			print("Creature created: ", creature_data["name"])


func _on_Button_pressed():
	# Increment the turn and load the new game state
	current_turn += 1
	print("Button pressed, loading turn: ", current_turn)
	load_game_state(current_turn)
