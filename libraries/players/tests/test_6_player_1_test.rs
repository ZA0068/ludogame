use board::Board;
use pieces::Color;
use players::Player;
use std::{cell::RefCell, rc::Rc};

#[cfg(test)]
mod player_1_tests {

    use super::*;

    static PLAYER_ID: i8 = 1;

    #[test]
    fn add_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(PLAYER_ID, board.clone(), None);
        assert_eq!(player.id(), 1);
        assert_eq!(player.board().as_ptr(), board.as_ptr());
    }

    #[test]
    fn get_pieces_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(PLAYER_ID, board, None);
        (0..4).for_each(|i| {
            let piece = player.piece(i);
            assert_eq!(piece.borrow().id(), i);
            assert_eq!(piece.borrow().color(), Color::Yellow);
            assert_ne!(piece.borrow().color(), Color::Green);
            assert_ne!(piece.borrow().color(), Color::Blue);
            assert_ne!(piece.borrow().color(), Color::Red);
        });
    }

    #[test]
    #[should_panic]
    fn get_pieces_test_2() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(PLAYER_ID, board, None);
        let piece = player.piece(0);
        assert_eq!(piece.borrow().id(), 0);
        assert_ne!(piece.borrow().color(), Color::Yellow);
    }

    #[test]
    #[should_panic]
    fn get_pieces_test_3() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(PLAYER_ID, board, None);
        let piece = player.piece(0);
        assert_eq!(piece.borrow().id(), 0);
        assert_eq!(piece.borrow().color(), Color::Red);
    }

    #[test]
    #[should_panic]
    fn get_pieces_test_4() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(PLAYER_ID, board, None);
        let piece = player.piece(0);
        assert_eq!(piece.borrow().id(), 0);
        assert_eq!(piece.borrow().color(), Color::Blue);
    }

    #[test]
    #[should_panic]
    fn get_pieces_test_5() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(PLAYER_ID, board, None);
        let piece = player.piece(0);
        assert_eq!(piece.borrow().id(), 0);
        assert_eq!(piece.borrow().color(), Color::Green);
    }
}
