use crate::models::pokemon::{Pokemon, Stat, StatName, Type, TypeInfo};

pub fn create_test_pokemon(
    name: &str,
    hp: i64,
    attack: i64,
    defense: i64,
    speed: i64,
    type_name: &str,
) -> Pokemon {
    Pokemon {
        name: name.to_string(),
        stats: vec![
            Stat {
                base_stat: hp,
                stat: StatName {
                    name: "hp".to_string(),
                },
            },
            Stat {
                base_stat: attack,
                stat: StatName {
                    name: "attack".to_string(),
                },
            },
            Stat {
                base_stat: defense,
                stat: StatName {
                    name: "defense".to_string(),
                },
            },
            Stat {
                base_stat: speed,
                stat: StatName {
                    name: "speed".to_string(),
                },
            },
        ],
        types: vec![Type {
            type_info: TypeInfo {
                name: type_name.to_string(),
            },
        }],
    }
}
