use super::tournament::MatchResult;
use super::trainer::Trainer;

pub struct TournamentReporter;

// ANSI color codes
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

impl TournamentReporter {
    pub fn new() -> Self {
        Self
    }

    pub fn print_bracket(&self, matches: &[MatchResult], champion: &Trainer) {
        if matches.is_empty() {
            return;
        }

        let max_round = matches.iter().map(|m| m.round).max().unwrap_or(0);

        println!("\n{}", "=".repeat(100));
        println!("{}🏆 TOURNAMENT BRACKET 🏆{}", BOLD, RESET);
        println!("{}\n", "=".repeat(100));

        // Organize matches by round
        let mut rounds: Vec<Vec<&MatchResult>> = vec![Vec::new(); max_round];
        for m in matches {
            rounds[m.round - 1].push(m);
        }

        self.print_vertical_bracket(&rounds, max_round);

        println!("\n{}", "=".repeat(100));
        println!("{}{}🏆 CHAMPION: {} 🏆{}{}", BOLD, GREEN, champion.get_name(), RESET, RESET);
        println!("{}\n", "=".repeat(100));
    }

    fn print_vertical_bracket(&self, rounds: &[Vec<&MatchResult>], max_round: usize) {
        if rounds.is_empty() {
            return;
        }

        // Print each round
        for (round_idx, round_matches) in rounds.iter().enumerate() {
            let round_name = match max_round - round_idx {
                1 => "FINALS",
                2 => "SEMI-FINALS",
                3 => "QUARTER-FINALS",
                _ => &format!("ROUND {}", round_idx + 1),
            };

            println!("\n{}", round_name);
            println!("{}", "-".repeat(round_name.len()));

            for m in round_matches.iter() {
                if m.fighter2 == "free_pass" {
                    // Free pass
                    println!("{}{} -> fp{}", GREEN, m.fighter1, RESET);
                } else {
                    // Regular match - show both fighters and winner
                    let f1_color = if &m.fighter1 == &m.winner { GREEN } else { RED };
                    let f2_color = if &m.fighter2 == &m.winner { GREEN } else { RED };

                    println!("{}{}{}", f1_color, m.fighter1, RESET);
                    println!("{}{}{} : {}-> {}{}\n",
                        f2_color, m.fighter2, RESET,
                        GREEN, m.winner, RESET
                    );
                }
            }
        }
    }

    fn pad_name(&self, name: &str, width: usize) -> String {
        if name.len() >= width {
            name[..width].to_string()
        } else {
            format!("{:<width$}", name, width = width)
        }
    }

    pub fn print_insufficient_participants(&self) {
        println!("Tournament needs at least 2 participants!");
    }
}
