use pokefight_core::{MatchResult, TournamentResult};
use std::collections::HashMap;
use std::fmt;

// ANSI color codes
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

pub struct TournamentDisplay<'a> {
    result: &'a TournamentResult,
}

impl<'a> TournamentDisplay<'a> {
    pub fn new(result: &'a TournamentResult) -> Self {
        Self { result }
    }
}

impl<'a> fmt::Display for TournamentDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Header
        let separator = "=".repeat(100);
        writeln!(f, "\n{}", separator)?;
        writeln!(
            f,
            "{}{}🏆 {} - Battling for: {} 🏆{}",
            BOLD, GREEN, self.result.name, self.result.chore, RESET
        )?;
        writeln!(f, "{}\n", separator)?;

        // Group matches by round
        let mut rounds: HashMap<usize, Vec<&MatchResult>> = HashMap::new();
        let mut max_round = 0;

        for match_result in &self.result.matches {
            rounds
                .entry(match_result.round)
                .or_insert_with(Vec::new)
                .push(match_result);
            max_round = max_round.max(match_result.round);
        }

        // Build sideways bracket
        let mut bracket_lines: Vec<Vec<String>> = Vec::new();

        for round_idx in 1..=max_round {
            if let Some(round_matches) = rounds.get(&round_idx) {
                let round_name = get_round_name(round_idx - 1, max_round);
                let mut round_lines = vec![round_name, String::new()];

                for match_result in round_matches {
                    if match_result.fighter2 == "free_pass" {
                        round_lines.push(format!(
                            "{}{}({}) [BYE]{}",
                            BOLD,
                            match_result.fighter1,
                            match_result.fighter1_pokemon,
                            RESET
                        ));
                        round_lines.push(String::new());
                    } else {
                        // Fighter 1
                        let f1_color = if match_result.fighter1 == match_result.winner {
                            format!("{}{}", BOLD, GREEN)
                        } else {
                            RED.to_string()
                        };
                        round_lines.push(format!(
                            "{}{}({}){}",
                            f1_color,
                            match_result.fighter1,
                            match_result.fighter1_pokemon,
                            RESET
                        ));

                        // Fighter 2
                        let f2_color = if match_result.fighter2 == match_result.winner {
                            format!("{}{}", BOLD, GREEN)
                        } else {
                            RED.to_string()
                        };
                        round_lines.push(format!(
                            "{}{}({}){}",
                            f2_color,
                            match_result.fighter2,
                            match_result.fighter2_pokemon,
                            RESET
                        ));
                        round_lines.push(String::new());
                    }
                }

                bracket_lines.push(round_lines);
            }
        }

        // Calculate the maximum width for each column (accounting for ANSI codes)
        let mut column_widths: Vec<usize> = Vec::new();
        for round in &bracket_lines {
            let max_visible_len = round
                .iter()
                .map(|line| {
                    // Remove ANSI escape codes to get actual visible length
                    let without_ansi = line
                        .replace("\x1b[32m", "")
                        .replace("\x1b[31m", "")
                        .replace("\x1b[0m", "")
                        .replace("\x1b[1m", "");
                    without_ansi.len()
                })
                .max()
                .unwrap_or(0);
            column_widths.push(max_visible_len);
        }

        // Print sideways - each column is a round
        if !bracket_lines.is_empty() {
            let max_lines = bracket_lines.iter().map(|r| r.len()).max().unwrap_or(0);

            for line_idx in 0..max_lines {
                let mut line_parts = Vec::new();

                for (round_idx, round) in bracket_lines.iter().enumerate() {
                    let width = column_widths[round_idx];
                    if line_idx < round.len() {
                        let line = &round[line_idx];
                        // Calculate visible length (without ANSI codes)
                        let visible_len = line
                            .replace("\x1b[32m", "")
                            .replace("\x1b[31m", "")
                            .replace("\x1b[0m", "")
                            .replace("\x1b[1m", "")
                            .len();
                        let padding = width.saturating_sub(visible_len);
                        line_parts.push(format!("{}{}", line, " ".repeat(padding)));
                    } else {
                        line_parts.push(" ".repeat(width));
                    }
                }

                writeln!(f, "{}", line_parts.join("  "))?;
            }
        }

        // Footer
        writeln!(f, "\n{}", separator)?;
        writeln!(
            f,
            "{}{}🏆 CHAMPION: {} 🏆{}{}",
            BOLD, GREEN, self.result.champion, RESET, RESET
        )?;
        writeln!(f, "{}\n", separator)?;

        Ok(())
    }
}

fn get_round_name(round_idx: usize, total_rounds: usize) -> String {
    match total_rounds - round_idx {
        1 => "FINALS".to_string(),
        2 => "SEMI-FINALS".to_string(),
        3 => "QUARTER-FINALS".to_string(),
        _ => format!("ROUND {}", round_idx + 1),
    }
}
