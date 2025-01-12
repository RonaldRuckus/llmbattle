use std::str::FromStr as _;

use serde::{Deserialize, Serialize};
use crate::embedding;
use super::creature::{Attribute, Element};

#[derive(Debug, Deserialize)]
pub struct SmolAbility {
    name: String,
    base_value: u8,
    modifier: f32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ability {
    pub name: String,
    pub description: String,
    pub base_damage: u16,
    pub available: (u8, u8), // (current, max)
    pub elements: Vec<Element>,
    pub modifiers: Vec<(i8, Attribute)>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AbilityCategory {
    Attack,
    Defense,
    Utility,
}

/// Request to create an ability
/// Entirely creative. This is the request to be matched with our static list
#[derive(Debug, Deserialize, Clone)]
pub struct AbilityRequest {
    pub name: String,
    pub description: String
}

impl AbilityRequest {
    /// Matches the ability with the static list of abilities using embeddings
    pub async fn fill(
        &self,
        abilities: &[SmolAbility],
        elements: &[Element]
    ) -> Ability {
        println!("Filling ability: {:?}", self);
        let query_embedding = embedding::embed(
            format!("{}: {}", self.name, self.description).as_str()
        ).await.unwrap();
        // Use a Vector Database
        let ability_name = embedding::search(
            embedding::Query::Vector(&query_embedding),
            embedding::Category::Ability,
            1
        ).await.first().unwrap().0.clone();

        let element_name = embedding::search(
            embedding::Query::Vector(&query_embedding),
            embedding::Category::Element,
            1
        ).await.first().unwrap().0.clone().trim().to_lowercase();

        println!("Ability: {:?}. Element: {:?}", ability_name, element_name);

        let ability = abilities.iter().find(|a| a.name == ability_name).unwrap();

        let element = Element::from_str(&element_name).unwrap();

        println!("Found element: {:?}", element);

        Ability {
            name: self.name.clone(),
            description: self.description.clone(),
            base_damage: ability.base_value as u16,
            available: (10, 10),
            elements: vec![element],
            modifiers: vec![]
        }
    }
}