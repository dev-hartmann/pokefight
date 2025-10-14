use std::marker::PhantomData;

use super::pokemon::BattlePokemon;
use super::trainer::Trainer;

// State markers (zero-sized types)
pub struct Ready;
pub struct InProgress;
pub struct Finished;

pub struct Battle<'a, State = Ready> {
    participants: BattleParticipants<'a>,
    state: PhantomData<State>,
}

pub struct BattleParticipants<'a> {
    faster: BattleParticipant<'a>,
    slower: BattleParticipant<'a>,
}

pub struct BattleParticipant<'a> {
    pokemon: BattlePokemon<'a>,
    trainer: &'a Trainer,
}

// Battle construction and state transitions
impl<'a> Battle<'a, Ready> {
    pub fn new(trainer1: &'a Trainer, trainer2: &'a Trainer) -> Self {
        let p1 = trainer1.get_pokemon();
        let p2 = trainer2.get_pokemon();

        // Order by speed at construction - faster attacks first
        let (faster, slower) = if p1.get_speed() >= p2.get_speed() {
            (
                BattleParticipant {
                    pokemon: p1,
                    trainer: trainer1,
                },
                BattleParticipant {
                    pokemon: p2,
                    trainer: trainer2,
                },
            )
        } else {
            (
                BattleParticipant {
                    pokemon: p2,
                    trainer: trainer2,
                },
                BattleParticipant {
                    pokemon: p1,
                    trainer: trainer1,
                },
            )
        };

        Self {
            participants: BattleParticipants { faster, slower },
            state: PhantomData,
        }
    }

    pub fn begin(self) -> Battle<'a, InProgress> {
        Battle {
            participants: self.participants,
            state: PhantomData,
        }
    }
}

impl<'a> Battle<'a, InProgress> {
    pub fn rounds(&mut self) -> RoundIterator<'a, '_> {
        RoundIterator { battle: self }
    }

    pub fn finish(self) -> Battle<'a, Finished> {
        Battle {
            participants: self.participants,
            state: PhantomData,
        }
    }
}

impl<'a> Battle<'a, Finished> {
    pub fn winner(&self) -> &'a Trainer {
        if self.participants.faster.pokemon.is_fainted() {
            self.participants.slower.trainer
        } else {
            self.participants.faster.trainer
        }
    }
}

// Round iterator for executing battle rounds
pub struct RoundIterator<'a, 'b> {
    battle: &'b mut Battle<'a, InProgress>,
}

impl<'a, 'b> RoundIterator<'a, 'b> {
    pub fn execute_round(&mut self, effectiveness: impl Fn(&str, &str) -> f64) -> Option<()> {
        // Faster pokemon attacks first
        let damage = calculate_damage(
            &self.battle.participants.faster.pokemon,
            &self.battle.participants.slower.pokemon,
            &effectiveness,
        );
        self.battle.participants.slower.pokemon.take_damage(damage);

        // Check if slower pokemon fainted
        if self.battle.participants.slower.pokemon.is_fainted() {
            return None;
        }

        // Slower pokemon counter-attacks
        let damage = calculate_damage(
            &self.battle.participants.slower.pokemon,
            &self.battle.participants.faster.pokemon,
            &effectiveness,
        );
        self.battle.participants.faster.pokemon.take_damage(damage);

        // Check if faster pokemon fainted
        if self.battle.participants.faster.pokemon.is_fainted() {
            return None;
        }

        // Both pokemon still alive, round complete
        Some(())
    }
}

// Helper function to calculate damage
fn calculate_damage(
    attacker: &BattlePokemon,
    defender: &BattlePokemon,
    effectiveness: impl Fn(&str, &str) -> f64,
) -> i64 {
    let base_damage = attacker.get_attack() / defender.get_defense() * 20;
    let multiplier = effectiveness(attacker.get_type(), defender.get_type());
    let damage = (base_damage as f64 * multiplier) as i64;

    // Ensure minimum damage of 1 to prevent infinite battles
    damage.max(1)
}
