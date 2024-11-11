# [Action] Module

This module covers the options for an action. Typically, a selection of 4 action is found here. Each option uses the following schema:

```json
{
	"name": string,                      // The name of the attack 
	"description": string,               // The description
	"cooldown": int,                     // The cooldown in turns
	"elements": Element[],               // The element(s) of this attack
	"causes": {                          // The causes of this attack
		"statuses": StatusEffect[],      // Status effects
		"attributes": AttributeEffect[], // Attribute effects (HP etc)
	},
	"times_allowed": int,                // Allowance in a game
	"animation_prompt": string           // The prompt used to generate an                                                   animation
}
```

