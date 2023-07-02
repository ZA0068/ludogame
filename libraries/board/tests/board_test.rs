use board::{Board};
use std::any::{Any, TypeId};
#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn create_a_board_test() {
        let board = Board::new();
        assert_eq!(TypeId::of::<Board>(), board.type_id());
    }

    #[test]
    fn create_default_board_test() {
        let board = Board::default();
        assert_eq!(TypeId::of::<Board>(), board.type_id());
    }
}