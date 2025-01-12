use std::{collections::HashMap, fs, str::FromStr};
use serde::{Deserialize, Deserializer, Serialize};
use super::ability::{Ability, AbilityRequest, SmolAbility};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Element {
    Physical,
    Mental,
    Fire,
    Water,
    Earth,
    Air,
}

impl FromStr for Element {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "physical" => Ok(Element::Physical),
            "mental" => Ok(Element::Mental),
            "fire" => Ok(Element::Fire),
            "water" => Ok(Element::Water),
            "earth" => Ok(Element::Earth),
            "air" => Ok(Element::Air),
            _ => Err(format!("Unknown element: {}", input)),
        }
    }
}

impl<'de> Deserialize<'de> for Element {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize the input as a `String` to handle owned strings
        let s: String = Deserialize::deserialize(deserializer)?;
        Element::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Attribute {
    Strength,
    Defense,
    Perception,
    Intelligence,
    Wisdom,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Creature {
    //pub id: String,
    pub owner: i64,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub max_health: u32,
    pub attributes: HashMap<Attribute, u8>,
    pub elements: Vec<Element>,
    pub abilities: Vec<Ability>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
    pub health: u32,
    pub abilities: Vec<Ability>,
    pub modifiers: Vec<(i8, Attribute)>
}

/// Request to create a creature
#[derive(Debug, Deserialize, Clone)]
pub struct CreateRequest {
    pub game_id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: String,
    pub image: Option<String>, // A prompt of the image
    pub elements: Vec<Element>,
    pub abilities: Vec<AbilityRequest>,
    pub attributes: HashMap<Attribute, u8>
}

impl CreateRequest {
    pub fn validate(&self) -> Result<(), String> {

        let attribute_points = 25;
        let mut total = 0;
        println!("{:?}", self.attributes);
        for (_, value) in self.attributes.iter() {
            total += value;
        }
        if total != attribute_points {
            return Err(format!("Attributes must add up to {}", attribute_points));
        }


        // Ensure that there are a maximum of 2 elements
        if self.elements.len() > 2 {
            return Err("There can only be a maximum of 2 elements".to_string());
        }

        // Ensure that there are 4 abilities
        if self.abilities.len() != 4 {
            return Err("There must be 4 abilities".to_string());
        }

        println!("Validated");
        Ok(())
    }

    pub async fn transform(&self) -> Creature {

        // Reads the available static list of abilities and elements
        let available = fs::read_to_string("database/available.json").expect("Failed to read abilities.json");
        let available: serde_json::Value = serde_json::from_str(&available).unwrap();

        let abilities = available["Ability"].as_array().unwrap().iter().map(|e| serde_json::from_value::<SmolAbility>(e.clone()).unwrap()).collect::<Vec<SmolAbility>>();

        let elements = available["Element"].as_array().unwrap().iter().map(|e| serde_json::from_value::<Element>(e.clone()).unwrap()).collect::<Vec<Element>>();

        let mut filled_abilities = Vec::new();
        // TODO: Multi-threading this probably
        for ability in self.abilities.iter() {
            let result = ability.fill(
                &abilities,
                &elements
            ).await;
            filled_abilities.push(result);
        }

        let creature = Creature {
            owner: self.user_id,
            name: self.name.clone(),
            description: self.description.clone(),
            image: self.image.clone(),
            max_health: 100,
            attributes: self.attributes.clone(),
            elements: self.elements.clone(),
            abilities: filled_abilities
        };

        creature
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fill_abilities() {

        let example = fs::read_to_string("examples/creature.json").expect("Failed to read creature.json");
        let example = serde_json::from_str::<CreateRequest>(&example).unwrap();
        example.validate().unwrap();
        println!("Validated");
        let creature = example.transform().await;
        println!("{:?}", creature);

    }
}