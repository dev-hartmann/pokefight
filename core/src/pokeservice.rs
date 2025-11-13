use crate::errors::Result;
use crate::models::pokemon::Pokemon;
use reqwest::Client;

pub struct PokeService {
    client: Client,
}

impl PokeService {
    pub async fn get_random_pokemon(&self) -> Result<Pokemon> {
        let rand_id = rand::random_range(1..900);
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

impl Default for PokeService {
    fn default() -> Self {
        let client = reqwest::Client::new();
        PokeService { client }
    }
}
