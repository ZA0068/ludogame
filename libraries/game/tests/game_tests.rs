use game::Game;
use iplayers::Playstyle;

#[cfg(test)]
mod init_game_test {

    use super::*;

    #[test]
    fn initialization_test() {
        let mut game = Game::new();
        game.setup_game();
        assert_eq!(game.iplayer(0).player().id(), 0);
        assert_eq!(game.iplayer(1).player().id(), 1);
        assert_eq!(game.iplayer(2).player().id(), 2);
        assert_eq!(game.iplayer(3).player().id(), 3);
    }

    #[test]
    fn beginning_test() {
        let mut game = Game::new();
        game.setup_game();
        game.beginning();
        println!("1st Player: {:?}\n", game.iplayer(0).player().id());
        println!("2nd Player: {:?}\n", game.iplayer(1).player().id());
        println!("3rd Player: {:?}\n", game.iplayer(2).player().id());
        println!("4th Player: {:?}\n", game.iplayer(3).player().id());
    }

    #[test]
    fn run_game_random_test() {
        let mut game = Game::new();
        game.setup_game();
        game.give_iplayer_a_playstyle(0, Playstyle::Random);
        game.give_iplayer_a_playstyle(1, Playstyle::Random);
        game.give_iplayer_a_playstyle(2, Playstyle::Random);
        game.give_iplayer_a_playstyle(3, Playstyle::Random);
        game.beginning();
        game.run();
    }

    #[test]
    fn run_game_aggro_test() {
        let mut game = Game::new();
        game.setup_game();
        game.give_iplayer_a_playstyle(0, Playstyle::Aggressive);
        game.give_iplayer_a_playstyle(1, Playstyle::Aggressive);
        game.give_iplayer_a_playstyle(2, Playstyle::Aggressive);
        game.give_iplayer_a_playstyle(3, Playstyle::Aggressive);
        game.beginning();
        game.run();
    }

    #[test]
    fn run_game_fast_test() {
        let mut game = Game::new();
        game.setup_game();
        game.give_iplayer_a_playstyle(0, Playstyle::Fast);
        game.give_iplayer_a_playstyle(1, Playstyle::Fast);
        game.give_iplayer_a_playstyle(2, Playstyle::Fast);
        game.give_iplayer_a_playstyle(3, Playstyle::Fast);
        game.beginning();
        game.run();
    }

    #[test]
    fn run_game_mixed_test() {
        let mut game = Game::new();
        game.setup_game();
        game.give_iplayer_a_playstyle(0, Playstyle::Fast);
        game.give_iplayer_a_playstyle(1, Playstyle::Random);
        game.give_iplayer_a_playstyle(2, Playstyle::Safe);
        game.give_iplayer_a_playstyle(3, Playstyle::Aggressive);
        game.beginning();
        game.run();
    }

    #[test]
    fn proper_beginning_test() {
        let mut game = Game::new();
        game.setup_game();
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
        game.setup_game();
        game.give_iplayer_a_playstyle(0, Playstyle::Fast);
        game.give_iplayer_a_playstyle(1, Playstyle::Random);
        game.give_iplayer_a_playstyle(2, Playstyle::Safe);
        game.give_iplayer_a_playstyle(3, Playstyle::Aggressive);
        game.start_game(1000);
        game.iplayer(0).calculate_winrate(1000);
        game.iplayer(1).calculate_winrate(1000);
        game.iplayer(2).calculate_winrate(1000);
        game.iplayer(3).calculate_winrate(1000);
        game.iplayer(0).print_winrate();
        game.iplayer(1).print_winrate();
        game.iplayer(2).print_winrate();
        game.iplayer(3).print_winrate();
    }

    #[test]
    fn get_iplayer_id_test() {
        let mut game = Game::new();
        game.setup_game();
        game.give_iplayer_a_playstyle(0, Playstyle::Fast);
        game.give_iplayer_a_playstyle(1, Playstyle::Random);
        game.give_iplayer_a_playstyle(2, Playstyle::Safe);
        game.give_iplayer_a_playstyle(3, Playstyle::Aggressive);

        for _ in 0..1000 {
            game.beginning();
            let id0 = game.iplayer(0).player().id();
            let id1 = game.iplayer(1).player().id();
            let id2 = game.iplayer(2).player().id();
            let id3 = game.iplayer(3).player().id();
            assert_eq!(id0, 0);
            assert_eq!(id1, 1);
            assert_eq!(id2, 2);
            assert_eq!(id3, 3);
        }
    }
}
