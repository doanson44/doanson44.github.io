use crate::domain::games::{PongGame, PongTickResult};

/// Application service for the single-player Pong game.
#[derive(Debug, Default)]
pub struct PongService;

impl PongService {
    /// Creates a new Pong game.
    pub fn new_game() -> PongGame {
        PongGame::new()
    }

    /// Advances the game by one simulation tick.
    pub fn tick(game: &mut PongGame) -> PongTickResult {
        game.tick()
    }

    /// Moves the player's paddle.
    pub fn move_player(game: &mut PongGame, delta: i32) {
        game.move_player(delta);
    }

    /// Resets the game to its initial state.
    pub fn reset(game: &mut PongGame) {
        game.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_starts_at_zero() {
        let game = PongService::new_game();
        assert_eq!(game.score(), 0);
        assert!(!game.is_game_over());
    }
}
