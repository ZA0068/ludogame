use board::{BoardState, PlayerID};
use pieces::{Color, Piece};
use std::any::{Any, TypeId};
use std::{cell::RefCell, rc::Rc};

mod board_state_tests {
    use super::*;

    #[test]
    fn create_a_board_state_test() {
        let board_state = BoardState::new(-1, Vec::default(), None);
        assert_eq!(board_state.position, -1);
        assert_eq!(board_state.pieces, Vec::default());
        assert_eq!(board_state.player_id, None);
    }

    #[test]
    fn create_default_board_state_test() {
        let board_state = BoardState::default();
        assert_eq!(board_state.position, -1);
        assert_eq!(board_state.pieces, Vec::default());
        assert_eq!(board_state.player_id, None);
    }

    #[test]
    fn create_a_board_state_test_2() {
        let pieces = vec![
            Rc::new(RefCell::new(Piece::new(0, Color::Green))),
            Rc::new(RefCell::new(Piece::new(1, Color::Green))),
            Rc::new(RefCell::new(Piece::new(2, Color::Green))),
            Rc::new(RefCell::new(Piece::new(3, Color::Green))),
        ];
        let board_state = BoardState::new(-1, pieces.clone(), Some(PlayerID::Player1));
        assert_eq!(TypeId::of::<BoardState>(), board_state.type_id());
        assert_eq!(board_state.position, -1);
        assert_eq!(board_state.pieces, pieces);
        assert_eq!(board_state.player_id, Some(PlayerID::Player1));
    }

    #[test]
    fn create_board_state_test_3() {
        let boardstate = BoardState::default();
        let boardstate_get = boardstate.get();
        assert_eq!(boardstate_get.position, -1);
        assert_eq!(boardstate_get.pieces, Vec::default());
        assert_eq!(boardstate_get.player_id, None);
    }

    #[test]
    fn pieces_test() {
        let pieces = vec![
            Rc::new(RefCell::new(Piece::new(0, Color::Green))),
            Rc::new(RefCell::new(Piece::new(1, Color::Green))),
            Rc::new(RefCell::new(Piece::new(2, Color::Green))),
            Rc::new(RefCell::new(Piece::new(3, Color::Green))),
        ];
        let mut board_state = BoardState::new(-1, pieces.clone(), Some(PlayerID::Player0));
        for i in 0..4 {
            assert_eq!(board_state.piece(i), pieces[i as usize]);
            assert_eq!(board_state.piece(i).borrow().id(), i);
        }
    }
}
