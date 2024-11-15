# [Action] Module

This module covers the options for an action. Typically, a selection of 4 action is found here. Each option uses the following schema:

```json
{
	"name": string,                      // The name of the attack 
	"description": string,               // The description
	"action_type": string                // {standard, cooldown, limited}
	"cooldown": int,                     // The cooldown in turns for this action
	"cooldown-timer": int,               // The remaining cooldown until allowed
	"elements": Element[],               // The element(s) of this attack
	"causes": {                          // The causes of this attack
		"statuses": StatusEffect[],      // Status effects
		"attributes": AttributeEffect[], // Attribute effects (HP etc)
	},
	"times_allowed": int,                // Allowance in a game, either always,                                                once if cooldown, or number if limited                                             usage  
	"animation_prompt": string           // The prompt used to generate an                                                   animation
}
```



Added the following:
- action_type
- added cooldown-timer
- time_allowed


**times_allowed logic:**
- a standard action is always allowed
- a cooldown action is allowed only once and only if the cooldown_timer == 0
- a action with limited usage can be performed a given number of times
-> requires: action type enum, methods 


The actions for each monster become part of the game state due to variables. 
For the POC we can pass the all actions with all details to the LLMs. At a later stage we should simplify this for costs savings. 

