# res://Resources/creature_data.gd
extends Resource
class_name CreatureData

@export var id: int = 0
@export var creature_name: String = ""
@export var health: int = 0
@export var max_health: int = 0
@export var attack: int = 0
@export var defense: int = 0
@export var position_data: Vector2 = Vector2()
@export var status_effects: Array[String] = []
@export var team: String = ""

static func from_dictionary(data: Dictionary) -> CreatureData:
	var creature_data = CreatureData.new()
	creature_data.id = data.get("id", 0)
	creature_data.creature_name = data.get("name", "")
	creature_data.health = data.get("health", 0)
	creature_data.max_health = data.get("max_health", 0)
	creature_data.attack = data.get("attack", 0)
	creature_data.defense = data.get("defense", 0)

	# Handle position conversion
	var pos_x = data["position"].get("x", 0)
	var pos_y = data["position"].get("y", 0)
	creature_data.position_data = Vector2(pos_x, pos_y)

	# Handle status effects - clear and append instead of reassigning
	creature_data.status_effects.clear()  # Clear existing array
	var raw_effects = data.get("status_effects", [])
	for effect in raw_effects:
		creature_data.status_effects.append(str(effect))

	creature_data.team = data.get("team", "")
	return creature_data
