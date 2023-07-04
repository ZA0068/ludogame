use board::Board;
use dice::Dice;
use players::{Player};
use std::{cell::RefCell, rc::Rc};

#[cfg(test)]
mod default_player_tests {

    use super::*;

    #[test]
    fn add_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(0, board.clone(), None);
        assert_eq!(player.id(), 0);
        assert_eq!(player.board().as_ptr(), board.as_ptr());
    }



    #[test]
    fn get_pieces_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        for player_id in 0..4{
            let player = Player::new(player_id, board.clone(), None);
            for i in 0..4{
                let piece = player.board().borrow_mut().home(player_id).piece(i);
                assert_eq!(piece.borrow().id(), i);
                assert!(piece.borrow().is_home());
                assert!(piece.borrow().is_safe());
                assert_eq!(piece.as_ptr(), board.borrow_mut().home(player_id).piece(i).as_ptr());
            }
        }
    }

    #[test]
    fn player_with_dice_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(0, board, None);
        
        let result = player.roll_dice();
        assert!(result == 0);
        
        let dice = Rc::new(RefCell::new(Dice::default()));

        player.take_dice(dice);
        let result = player.roll_dice();
        assert!(result > 0 && result < 7);

        player.give_dice();
        let result = player.roll_dice();
        assert!(result == 0);
    }
}