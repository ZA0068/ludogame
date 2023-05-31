use game::Game;

#[cfg(test)]
mod game_initialization_test {
    use super::*;

    #[test]
    fn start_the_game() {
        let mut game = Game::new();
        game.start_the_game();
    }
}
