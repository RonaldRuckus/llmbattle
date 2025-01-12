use serde::{Deserialize, Serialize};
use super::creature::Creature;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub creatures: Vec<Creature>,
    pub items: Vec<Item>,
    pub name: String,
}

impl Player {
    pub fn new(
        name: String,
        creatures: Vec<Creature>,
        items: Vec<Item>
    ) -> Self {
        Player { creatures, items, name }
    }
}