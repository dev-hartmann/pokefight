use crate::models::pokemon::Pokemon;
use rand::Rng;
use reqwest::Client;
use std::collections::HashMap;

pub struct PokeService {
    client: Client,
    cache: HashMap<String, String>,
}

impl PokeService {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        let cache = HashMap::new();
        PokeService { client, cache }
    }

    pub async fn get_random_pokemon(&mut self) -> Result<Pokemon, reqwest::Error> {
        let mut rng = rand::rng();
        let rand_id = rng.random_range(1..900);
        let pokemon = self
            .client
            .get(format!("https://pokeapi.co/api/v2/pokemon/{rand_id}"))
            .send()
            .await?
            .json::<Pokemon>()
            .await?;
        Ok(pokemon)
    }
}
