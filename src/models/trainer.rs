use super::pokemon::{BattlePokemon, Pokemon};

#[derive(Debug, Clone)]
pub struct Trainer {
    name: String,
    pokemon: Pokemon,
}

impl Trainer {
    pub fn new(name: String, pokemon: Pokemon) -> Self {
        Self { name, pokemon }
    }

    pub fn get_pokemon(&self) -> BattlePokemon<'_> {
        BattlePokemon::new(&self.pokemon)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
