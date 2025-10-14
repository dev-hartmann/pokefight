use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pokemon {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "stats")]
    pub stats: Vec<Stat>,

    #[serde(rename = "types")]
    pub types: Vec<Type>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stat {
    #[serde(rename = "base_stat")]
    pub base_stat: i64,

    #[serde(rename = "stat")]
    pub stat: StatName,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatName {
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Type {
    #[serde(rename = "type")]
    pub type_info: TypeInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TypeInfo {
    #[serde(rename = "name")]
    pub name: String,
}

// Helper methods
impl Pokemon {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_primary_type(&self) -> &str {
        self.types
            .first()
            .map(|t| t.type_info.name.as_str())
            .unwrap_or("normal")
    }

    pub fn get_hp(&self) -> i64 {
        self.stats
            .iter()
            .find(|s| s.stat.name == "hp")
            .map(|s| s.base_stat)
            .unwrap_or(50)
    }

    pub fn get_attack(&self) -> i64 {
        self.stats
            .iter()
            .find(|s| s.stat.name == "attack")
            .map(|s| s.base_stat)
            .unwrap_or(50)
    }

    pub fn get_defense(&self) -> i64 {
        self.stats
            .iter()
            .find(|s| s.stat.name == "defense")
            .map(|s| s.base_stat)
            .unwrap_or(50)
    }

    pub fn get_speed(&self) -> i64 {
        self.stats
            .iter()
            .find(|s| s.stat.name == "speed")
            .map(|s| s.base_stat)
            .unwrap_or(50)
    }
}

#[derive(Debug)]
pub struct BattlePokemon<'a> {
    pokemon: &'a Pokemon,
    current_hp: i64,
    max_hp: i64,
}

impl<'a> BattlePokemon<'a> {
    pub fn new(pokemon: &'a Pokemon) -> Self {
        let max_hp = pokemon.get_hp() * 2;
        Self {
            pokemon,
            max_hp,
            current_hp: max_hp,
        }
    }

    pub fn take_damage(&mut self, damage: i64) {
        if self.current_hp - damage <= 0 {
            self.current_hp = 0;
        } else {
            self.current_hp -= damage;
        }
    }

    pub fn is_fainted(&self) -> bool {
        self.current_hp == 0
    }

    pub fn get_name(&self) -> &str {
        self.pokemon.get_name()
    }
    pub fn get_type(&self) -> &str {
        self.pokemon.get_primary_type()
    }
    pub fn get_speed(&self) -> i64 {
        self.pokemon.get_speed()
    }

    pub fn get_attack(&self) -> i64 {
        self.pokemon.get_attack()
    }

    pub fn get_defense(&self) -> i64 {
        self.pokemon.get_defense()
    }
}
