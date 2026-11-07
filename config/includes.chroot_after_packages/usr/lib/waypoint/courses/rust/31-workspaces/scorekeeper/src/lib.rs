use std::collections::HashMap;

pub struct Scoreboard {
    scores: HashMap<String, u32>,
}

impl Scoreboard {
    pub fn new() -> Scoreboard {
        Scoreboard { scores: HashMap::new() }
    }

    /// Adds points, starting at 0 for a new player.
    pub fn add_points(&mut self, player: &str, points: u32) {
        todo!()
    }

    /// The player's score, or 0 if they haven't played.
    pub fn score_of(&self, player: &str) -> u32 {
        todo!()
    }

    /// Every player and score, highest first. Ties are alphabetical.
    pub fn ranking(&self) -> Vec<(String, u32)> {
        todo!()
    }

    /// The top player and score, or None.
    pub fn leader(&self) -> Option<(String, u32)> {
        todo!()
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
