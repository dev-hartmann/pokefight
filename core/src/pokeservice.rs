use crate::errors::Result;
use crate::models::pokemon::Pokemon;
use rand::Rng;
use reqwest::Client;

pub struct PokeService {
    client: Client,
}

impl PokeService {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        PokeService { client }
    }

    pub async fn get_random_pokemon(&mut self) -> Result<Pokemon> {
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
