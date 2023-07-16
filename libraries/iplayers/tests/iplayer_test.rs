use players::{Act, Player};
use iplayers::{IPlayer, Playstyle, Behavior};
use dice::Dice;
use std::cell::RefCell;
use std::rc::Rc;
use board::Board;

#[cfg(test)]
mod iplayer_tests {
    use super::*;

    static DEFAULT_ACTIONS: [Act; 10] = [
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
        
        let actions = DEFAULT_ACTIONS.to_vec();
        player.set_actions(actions);
        assert_eq!(player.get_actions(), &DEFAULT_ACTIONS.to_vec());

        let playstyle = Playstyle::Random;
        player.set_playstyle(playstyle);
        assert_eq!(player.get_playstyle(), &Playstyle::Random);
    }

    #[test]
    fn play_test() {
        let mut player = IPlayer::new(0);
        let mut simple_player = Player::new(0);
        let board = Rc::new(RefCell::new(Board::new()));
        player.setup_iplayer(board.clone());
        player.set_playstyle(Playstyle::Random);
        player.take_dice(Dice::default());
        player.play(true);

        simple_player.setup(board);

        assert_eq!(player.get_actions(), &DEFAULT_ACTIONS.to_vec());
        assert_eq!(simple_player.board().as_ptr(), player.player().board().as_ptr());
        assert_eq!(simple_player.piece(0).as_ptr(), player.player().piece(0).as_ptr());
        assert_eq!(simple_player.piece(1).as_ptr(), player.player().piece(1).as_ptr());
        assert_eq!(simple_player.piece(2).as_ptr(), player.player().piece(2).as_ptr());
        assert_eq!(simple_player.piece(3).as_ptr(), player.player().piece(3).as_ptr());
    }

    #[test]
    fn aggro_player_test() {
        let mut player = IPlayer::create(0, Playstyle::Aggressive);
        let dice = Dice::default();
        let board = Rc::new(RefCell::new(Board::new()));
        player.setup_iplayer(board);
        player.take_dice(dice);
        player.play(true);
        assert_eq!(player.get_actions(), &AGGRO_ACTIONS.to_vec());
    }

    #[test]
    fn fast_aggro_player_test() {
        let mut player = IPlayer::create(0, Playstyle::FastAggressive);
        let dice = Dice::default();
        let board = Rc::new(RefCell::new(Board::new()));
        player.setup_iplayer(board);
        player.take_dice(dice);
        player.play(true);
        assert_eq!(player.get_actions(), &FAST_AGGRO_ACTIONS.to_vec());
    }

    #[test]
    fn fast_player_test() {
        let mut player = IPlayer::create(0, Playstyle::Fast);
        let dice = Dice::default();
        let board = Rc::new(RefCell::new(Board::new()));
        player.setup_iplayer(board);
        player.take_dice(dice);
        player.play(true);
        assert_eq!(player.get_actions(), &FAST_ACTIONS.to_vec());
    }

    #[test]
    fn safe_player_test() {
        let mut player = IPlayer::create(0, Playstyle::Safe);
        let dice = Dice::default();
        let board = Rc::new(RefCell::new(Board::new()));
        player.setup_iplayer(board);
        player.take_dice(dice);
        player.play(true);
        assert_eq!(player.get_actions(), &SAFE_ACTIONS .to_vec());
    }
    
}

