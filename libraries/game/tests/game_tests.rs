use game::Game;
use iplayers::{Playstyle};

#[cfg(test)]
mod init_game_test {

    use super::*;

    #[test]
    fn initialization_test() {
        let mut game = Game::new();
        game.setup_board();
        assert_eq!(game.iplayer(0).player().id(), 0);
        assert_eq!(game.iplayer(1).player().id(), 1);
        assert_eq!(game.iplayer(2).player().id(), 2);
        assert_eq!(game.iplayer(3).player().id(), 3);
    }

    #[test]
    fn beginning_test() {
        let mut game = Game::new();
        game.setup_board();
        game.beginning();
        println!("1st Player: {:?}\n", game.iplayer(0).player().id());
        println!("2nd Player: {:?}\n", game.iplayer(1).player().id());
        println!("3rd Player: {:?}\n", game.iplayer(2).player().id());
        println!("4th Player: {:?}\n", game.iplayer(3).player().id());
    }

    #[test]
    fn run_game_random_test() {
        let mut game = Game::new();
        game.setup_board();
        game.give_iplayer_a_playstyle(0, Playstyle::Random);
        game.give_iplayer_a_playstyle(1, Playstyle::Random);
        game.give_iplayer_a_playstyle(2, Playstyle::Random);
        game.give_iplayer_a_playstyle(3, Playstyle::Random);
        game.run();
    }

    #[test]
    fn run_game_aggro_test() {
        let mut game = Game::new();
        game.setup_board();
        game.give_iplayer_a_playstyle(0, Playstyle::Aggressive);
        game.give_iplayer_a_playstyle(1, Playstyle::Aggressive);
        game.give_iplayer_a_playstyle(2, Playstyle::Aggressive);
        game.give_iplayer_a_playstyle(3, Playstyle::Aggressive);
        game.run();
    }

    
    #[test]
    fn run_game_fast_test() {
        let mut game = Game::new();
        game.setup_board();
        game.give_iplayer_a_playstyle(0, Playstyle::Fast);
        game.give_iplayer_a_playstyle(1, Playstyle::Fast);
        game.give_iplayer_a_playstyle(2, Playstyle::Fast);
        game.give_iplayer_a_playstyle(3, Playstyle::Fast);
        game.run();
    }

    #[test]
    fn run_game_mixed_test() {
        let mut game = Game::new();
        game.setup_board();
        game.give_iplayer_a_playstyle(0, Playstyle::Fast);
        game.give_iplayer_a_playstyle(1, Playstyle::Random);
        game.give_iplayer_a_playstyle(2, Playstyle::Safe);
        game.give_iplayer_a_playstyle(3, Playstyle::Aggressive);
        game.run();
    }

    #[test]
    fn proper_beginning_test() {
        let mut game = Game::new();
        game.setup_board();
        game.give_iplayer_a_playstyle(0, Playstyle::Fast);
        game.give_iplayer_a_playstyle(1, Playstyle::Random);
        game.give_iplayer_a_playstyle(2, Playstyle::Safe);
        game.give_iplayer_a_playstyle(3, Playstyle::Aggressive);
        game.beginning();
        game.run();
    }

    #[test]
    fn play_game_test() {
        let mut game = Game::new();
        game.setup_board();
        game.give_iplayer_a_playstyle(0, Playstyle::Fast);
        game.give_iplayer_a_playstyle(1, Playstyle::Random);
        game.give_iplayer_a_playstyle(2, Playstyle::Safe);
        game.give_iplayer_a_playstyle(3, Playstyle::Aggressive);
        game.start_game(1000);
    }

}