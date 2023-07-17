use board::Board;
use dice::Dice;
use iplayers::{Behavior, IPlayer, Playstyle};
use players::{Act, Player};
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod iplayer_tests {
    use super::*;

    static ACTIONS: [Act; 10] = [
        Act::Move,
        Act::Free,
        Act::Kill,
        Act::Join,
        Act::Leave,
        Act::Die,
        Act::Goal,
        Act::Safe,
        Act::Starjump,
        Act::Nothing,
    ];

    static AGGRO_ACTIONS: [Act; 10] = [
        Act::Kill,
        Act::Move,
        Act::Join,
        Act::Free,
        Act::Goal,
        Act::Starjump,
        Act::Leave,
        Act::Safe,
        Act::Nothing,
        Act::Die,
    ];

    static FAST_AGGRO_ACTIONS: [Act; 10] = [
        Act::Kill,
        Act::Goal,
        Act::Starjump,
        Act::Move,
        Act::Join,
        Act::Free,
        Act::Leave,
        Act::Safe,
        Act::Nothing,
        Act::Die,
    ];

    static SAFE_ACTIONS: [Act; 10] = [
        Act::Join,
        Act::Safe,
        Act::Goal,
        Act::Move,
        Act::Kill,
        Act::Starjump,
        Act::Free,
        Act::Leave,
        Act::Nothing,
        Act::Die,
    ];

    static FAST_ACTIONS: [Act; 10] = [
        Act::Goal,
        Act::Starjump,
        Act::Leave,
        Act::Join,
        Act::Move,
        Act::Kill,
        Act::Free,
        Act::Safe,
        Act::Nothing,
        Act::Die,
    ];

    #[test]
    fn instantiation_test() {
        let mut player = IPlayer::new(0);

        assert_eq!(player.player().id(), 0);

        player.set_actions(ACTIONS);
        assert_eq!(player.get_actions().to_vec(), ACTIONS.to_vec());

        let playstyle = Playstyle::Random;
        player.set_playstyle(playstyle);
        assert_eq!(player.get_playstyle(), &Playstyle::Random);
    }

    #[test]
    fn play_test() {
        let mut iplayer = IPlayer::new(0);
        let mut simple_player = Player::new(0);
        let board = Rc::new(RefCell::new(Board::new()));
        iplayer.setup_board(board.clone());
        iplayer.set_playstyle(Playstyle::Random);
        iplayer.take_dice(Dice::default());
        iplayer.play(true);

        simple_player.setup(board);

        assert_eq!(iplayer.get_actions().to_vec(), ACTIONS.to_vec());
        assert_eq!(
            simple_player.board().as_ptr(),
            iplayer.player().board().as_ptr()
        );
        assert_eq!(
            simple_player.piece(0).as_ptr(),
            iplayer.player().piece(0).as_ptr()
        );
        assert_eq!(
            simple_player.piece(1).as_ptr(),
            iplayer.player().piece(1).as_ptr()
        );
        assert_eq!(
            simple_player.piece(2).as_ptr(),
            iplayer.player().piece(2).as_ptr()
        );
        assert_eq!(
            simple_player.piece(3).as_ptr(),
            iplayer.player().piece(3).as_ptr()
        );
    }

    #[test]
    fn aggro_player_test() {
        let mut player =  IPlayer::create(0, Playstyle::Aggressive);
        let dice = Dice::default();
        let board = Rc::new(RefCell::new(Board::new()));
        player.setup_board(board);
        player.take_dice(dice);
        player.play(true);
        assert_eq!(player.get_actions().to_owned(), AGGRO_ACTIONS.to_owned());
    }

    #[test]
    fn fast_aggro_player_test() {
        let mut player = IPlayer::create(0, Playstyle::FastAggressive);
        let dice = Dice::default();
        let board = Rc::new(RefCell::new(Board::new()));
        player.setup_board(board);
        player.take_dice(dice);
        player.play(true);
        assert_eq!(player.get_actions().to_owned(), FAST_AGGRO_ACTIONS.to_owned());
    }

    #[test]
    fn fast_player_test() {
        let mut player = IPlayer::create(0, Playstyle::Fast);
        let dice = Dice::default();
        let board = Rc::new(RefCell::new(Board::new()));
        player.setup_board(board);
        player.take_dice(dice);
        player.play(true);
        assert_eq!(player.get_actions().to_owned(), FAST_ACTIONS.to_owned());
    }

    #[test]
    fn safe_player_test() {
        let mut player = IPlayer::create(0, Playstyle::Safe);
        let dice = Dice::default();
        let board = Rc::new(RefCell::new(Board::new()));
        player.setup_board(board);
        player.take_dice(dice);
        player.play(true);
        assert_eq!(player.get_actions().to_owned(), SAFE_ACTIONS.to_owned());
    }
}
