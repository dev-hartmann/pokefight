use crate::models::trainer::Trainer;

use super::battle::Battle;
use super::tournament_reporter::TournamentReporter;

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub fighter1: String,
    pub fighter2: String,
    pub winner: String,
    pub round: usize,
}

pub struct Tournament {
    name: String,
    participants: Vec<Trainer>,
    reporter: TournamentReporter,
}

impl Tournament {
    pub fn new(participants: Vec<Trainer>, name: &str) -> Self {
        Self {
            name: name.into(),
            participants,
            reporter: TournamentReporter::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    fn get_type_effectiveness(&self, attacker_type: &str, defender_type: &str) -> f64 {
        match (attacker_type, defender_type) {
            // Fire advantages
            ("fire", "grass") | ("fire", "ice") | ("fire", "bug") | ("fire", "steel") => 2.0,
            ("fire", "water") | ("fire", "rock") | ("fire", "dragon") => 0.5,

            // Water advantages
            ("water", "fire") | ("water", "ground") | ("water", "rock") => 2.0,
            ("water", "grass") | ("water", "dragon") => 0.5,

            // Grass advantages
            ("grass", "water") | ("grass", "ground") | ("grass", "rock") => 2.0,
            ("grass", "fire")
            | ("grass", "grass")
            | ("grass", "poison")
            | ("grass", "flying")
            | ("grass", "bug")
            | ("grass", "dragon")
            | ("grass", "steel") => 0.5,

            // Electric advantages
            ("electric", "water") | ("electric", "flying") => 2.0,
            ("electric", "grass") | ("electric", "electric") | ("electric", "dragon") => 0.5,
            ("electric", "ground") => 0.0,

            _ => 1.0, // Neutral
        }
    }

    fn fight<'a>(&self, fighter_one: &'a Trainer, fighter_two: &'a Trainer) -> &'a Trainer {
        let mut battle = Battle::new(fighter_one, fighter_two).begin();
        let mut rounds = battle.rounds();

        while rounds
            .execute_round(|a, d| self.get_type_effectiveness(a, d))
            .is_some()
        {
            // Round completed, continue
        }

        battle.finish().winner()
    }
    pub fn start(&self) -> Option<&Trainer> {
        if self.participants.len() < 2 {
            self.reporter.print_insufficient_participants();
            return None;
        }

        let mut current_fighters: Vec<&Trainer> = self.participants.iter().collect();
        let mut round_num = 1;
        let mut all_matches: Vec<MatchResult> = Vec::new();

        while current_fighters.len() > 1 {
            let round_matches = self.run_elimination_round(current_fighters, round_num);
            current_fighters = round_matches.iter().map(|m| {
                self.participants.iter().find(|t| t.get_name() == m.winner).unwrap()
            }).collect();
            all_matches.extend(round_matches);
            round_num += 1;
        }

        current_fighters.into_iter().next().map(|champion| {
            self.reporter.print_bracket(&all_matches, champion);
            champion
        })
    }

    fn run_elimination_round<'a>(
        &self,
        fighters: Vec<&'a Trainer>,
        round_num: usize,
    ) -> Vec<MatchResult> {
        let (pairs, bye) = self.create_pairings(fighters);
        let mut matches = Vec::new();

        if let Some(bye_fighter) = bye {
            matches.push(MatchResult {
                fighter1: bye_fighter.get_name().to_string(),
                fighter2: "free_pass".to_string(),
                winner: bye_fighter.get_name().to_string(),
                round: round_num,
            });
        }

        for (f1, f2) in pairs.into_iter() {
            let winner = self.fight(f1, f2);
            matches.push(MatchResult {
                fighter1: f1.get_name().to_string(),
                fighter2: f2.get_name().to_string(),
                winner: winner.get_name().to_string(),
                round: round_num,
            });
        }

        matches
    }

    fn create_pairings<'a>(
        &self,
        mut fighters: Vec<&'a Trainer>,
    ) -> (Vec<(&'a Trainer, &'a Trainer)>, Option<&'a Trainer>) {
        let bye = if fighters.len() % 2 == 1 {
            fighters.pop()
        } else {
            None
        };

        let pairs = fighters
            .chunks_exact(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect();

        (pairs, bye)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::test_utils::create_test_pokemon;

    #[test]
    fn test_tournament_with_two_trainers() {
        // Create two test Pokemon
        let pikachu = create_test_pokemon("pikachu", 39, 55, 40, 90, "electric");
        let charmander = create_test_pokemon("charmander", 39, 52, 43, 65, "fire");

        // Create two trainers
        let trainer1 = Trainer::new("Ash".to_string(), pikachu);
        let trainer2 = Trainer::new("Gary".to_string(), charmander);

        // Create tournament
        let mut tournament = Tournament::new(vec![trainer1, trainer2], "Test Tournament");

        // Start tournament
        let winner = tournament.start();

        // Assert we have a winner
        assert!(winner.is_some(), "Tournament should have a winner");

        let champion = winner.unwrap();

        // Assert winner is one of the two trainers
        assert!(champion.get_name() == "Ash");

        // With these stats, Pikachu (higher speed) should win
        // Speed: Pikachu=90, Charmander=65, so Pikachu attacks first
        assert_eq!(
            champion.get_name(),
            "Ash",
            "Ash with Pikachu should win due to higher speed"
        );
    }

    #[test]
    fn test_tournament_with_zero_trainers() {
        let mut tournament = Tournament::new(vec![], "Empty Tournament");
        let winner = tournament.start();
        assert!(
            winner.is_none(),
            "Tournament with no trainers should return None"
        );
    }

    #[test]
    fn test_tournament_with_one_trainer() {
        let pikachu = create_test_pokemon("pikachu", 35, 55, 40, 90, "electric");
        let trainer = Trainer::new("Ash".to_string(), pikachu);

        let mut tournament = Tournament::new(vec![trainer], "Single Trainer Tournament");
        let winner = tournament.start();

        assert!(
            winner.is_none(),
            "Tournament with one trainer should return None"
        );
    }

    #[test]
    fn test_tournament_with_three_trainers_free_pass() {
        // Create three test Pokemon with varying stats
        let pikachu = create_test_pokemon("pikachu", 35, 55, 40, 90, "electric");
        let charmander = create_test_pokemon("charmander", 500, 52, 43, 95, "fire");
        let bulbasaur = create_test_pokemon("bulbasaur", 45, 49, 49, 45, "grass");

        // Create three trainers
        let trainer1 = Trainer::new("Ash".to_string(), pikachu);
        let trainer2 = Trainer::new("Gary".to_string(), charmander);
        let trainer3 = Trainer::new("Misty".to_string(), bulbasaur);

        // Create tournament with 3 participants
        let mut tournament = Tournament::new(
            vec![trainer1, trainer2, trainer3],
            "Three Trainer Tournament",
        );

        // Start tournament
        let winner = tournament.start();

        // Assert we have a winner
        assert!(winner.is_some(), "Tournament should have a winner");

        let champion = winner.unwrap();

        // Gary's Charmander should win (500 HP, 95 speed beats everyone)
        assert_eq!(
            champion.get_name(),
            "Gary",
            "Gary with overpowered Charmander should win"
        );
    }

    #[test]
    fn test_tournament_bracket_with_eight_trainers() {
        // Create 8 test Pokemon with predictable stats
        let pokemon1 = create_test_pokemon("pikachu", 35, 100, 40, 90, "electric");
        let pokemon2 = create_test_pokemon("charmander", 39, 90, 43, 85, "fire");
        let pokemon3 = create_test_pokemon("bulbasaur", 45, 80, 49, 80, "grass");
        let pokemon4 = create_test_pokemon("squirtle", 44, 70, 65, 75, "water");
        let pokemon5 = create_test_pokemon("jigglypuff", 115, 60, 20, 70, "normal");
        let pokemon6 = create_test_pokemon("meowth", 40, 50, 35, 65, "normal");
        let pokemon7 = create_test_pokemon("psyduck", 50, 40, 48, 60, "water");
        let pokemon8 = create_test_pokemon("geodude", 40, 30, 100, 55, "rock");

        let trainers = vec![
            Trainer::new("Ash".to_string(), pokemon1),
            Trainer::new("Gary".to_string(), pokemon2),
            Trainer::new("Misty".to_string(), pokemon3),
            Trainer::new("Brock".to_string(), pokemon4),
            Trainer::new("Jessie".to_string(), pokemon5),
            Trainer::new("James".to_string(), pokemon6),
            Trainer::new("Nurse Joy".to_string(), pokemon7),
            Trainer::new("Officer Jenny".to_string(), pokemon8),
        ];

        let tournament = Tournament::new(trainers, "Eight-Player Tournament");
        let winner = tournament.start();

        assert!(winner.is_some(), "Tournament should have a winner");
        // Just verify there is a champion - actual winner depends on battle logic
        let champion = winner.unwrap();
        assert!(!champion.get_name().is_empty(), "Champion should have a name");
    }
}
