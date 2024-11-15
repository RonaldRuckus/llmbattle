# Creature.gd
extends Node2D

@onready var sprite_node = $Sprite2D
@onready var health_bar = $TextureProgressBar

var data: CreatureData


func _ready():
	print("Creature node ready: ", name)


func setup(json_data: Dictionary) -> void:
	print("Setting up creature with data: ", json_data)

	if !sprite_node or !health_bar:
		push_error("Nodes not properly initialized before setup")
		return

	data = CreatureData.from_dictionary(json_data)

	position = data.position_data
	print("Setting position for ", data.creature_name, " to: ", data.position_data)
	print("Final position for ", data.creature_name, ": ", position)

	update_visuals()


func update_visuals() -> void:
	if !is_inside_tree():
		push_error("Trying to update visuals before node is in scene tree")
		return

	if health_bar:
		health_bar.max_value = data.max_health
		health_bar.value = data.health
		print("Updated health bar for ", data.creature_name)
	else:
		push_error("Health bar not found")

	if sprite_node:
		var modulate_color = Color(0, 1, 0) if data.team == "player" else Color(1, 0, 0)
		sprite_node.modulate = modulate_color
		print("Updated sprite color for ", data.creature_name)
	else:
		push_error("Sprite node not found")
