use board::Board;
use std::any::{TypeId, Any};

#[cfg(test)]
mod board_instantiation_test {
    use super::*;

    #[test]
    fn create_a_board_test() {
        let board = Board::new();
        assert_eq!(TypeId::of::<Board>(), board.type_id());
    }

    #[test]
    fn board_home_spaces_test() {
        let board = Board::new();
        assert_eq!(board.home(), [-1; 16]);
    }

    #[test]
    fn board_goal_spaces_test() {
        let board = Board::new();
        assert_eq!(board.goal(), [99; 4]);
    }

    #[test]
    fn board_outside_spaces_test() {
        let board = Board::new();
        assert_eq!(board.outside(),
                   <Vec<i8> as std::convert::TryInto<[i8; 52]>>::try_into((0..52).map(|i| i as i8)
                          .collect::<Vec<i8>>())
                          .unwrap());
    }

    #[test]
    fn board_inside_spaces_test() {
        let board = Board::new();
        assert_eq!(board.inside(),
                   <Vec<i8> as std::convert::TryInto<[i8; 20]>>::try_into((52..72).map(|i| i as i8)
                          .collect::<Vec<i8>>())
                          .unwrap());
    }

    #[test]
    fn board_globe_spaces_test() {
        let board = Board::new();
        assert_eq!(board.globe(), [0, 8, 13, 21, 26, 34, 39, 47]);
    }

    #[test]
    fn board_star_spaces_test() {
        let board = Board::new();
        assert_eq!(board.star(), [5, 12, 18, 25, 31, 38, 44, 51]);
    }
}
