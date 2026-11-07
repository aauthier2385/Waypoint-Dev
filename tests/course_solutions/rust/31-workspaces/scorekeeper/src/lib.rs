use std::collections::HashMap;

pub struct Scoreboard {
    scores: HashMap<String, u32>,
}

impl Scoreboard {
    pub fn new() -> Scoreboard {
        Scoreboard { scores: HashMap::new() }
    }

    pub fn add_points(&mut self, player: &str, points: u32) {
        *self.scores.entry(player.to_string()).or_insert(0) += points;
    }

    pub fn score_of(&self, player: &str) -> u32 {
        self.scores.get(player).copied().unwrap_or(0)
    }

    pub fn ranking(&self) -> Vec<(String, u32)> {
        let mut ranking: Vec<(String, u32)> = self.scores.iter().map(|(name, score)| (name.clone(), *score)).collect();
        ranking.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        ranking
    }

    pub fn leader(&self) -> Option<(String, u32)> {
        self.ranking().into_iter().next()
    }
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_points() {
        let mut board = Scoreboard::new();
        board.add_points("Ana", 30);
        board.add_points("Ana", 20);
        assert_eq!(board.score_of("Ana"), 50);
        assert_eq!(board.score_of("Nobody"), 0);
    }

    #[test]
    fn rankings() {
        let mut board = Scoreboard::new();
        board.add_points("Sam", 40);
        board.add_points("Ana", 50);
        board.add_points("Lee", 40);
        assert_eq!(board.ranking(), vec![("Ana".to_string(), 50), ("Lee".to_string(), 40), ("Sam".to_string(), 40)],
                   "highest first, and Lee comes before Sam in a tie");
    }

    #[test]
    fn leaders() {
        let mut board = Scoreboard::new();
        assert_eq!(board.leader(), None);
        board.add_points("Sam", 10);
        assert_eq!(board.leader(), Some(("Sam".to_string(), 10)));
    }
}
