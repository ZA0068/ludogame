use game::Game;

#[cfg(test)]
mod game_initialization_test {
    use super::*;

    #[test]
    fn init_game_player_test() {
        let mut game = Game::new_default();
        assert_eq!(game.players().len(), 4);
        assert_eq!(game.player(0).id(), 0);
        assert_eq!(game.player(1).id(), 1);
        assert_eq!(game.player(2).id(), 2);
        assert_eq!(game.player(3).id(), 3);
    }
}
