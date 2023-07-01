use board::{Board, PlayerID};
use pieces::Piece;
use std::{cell::RefCell, rc::Rc};

mod board_space_tests {
    use super::*;
    #[test]
    fn board_home_spaces_test() {
        let mut board = Board::new();
        let pieces = vec![
            Rc::new(RefCell::new(Piece::new(0))),
            Rc::new(RefCell::new(Piece::new(1))),
            Rc::new(RefCell::new(Piece::new(2))),
            Rc::new(RefCell::new(Piece::new(3))),
        ];
        let player_ids = vec![
            PlayerID::Player0,
            PlayerID::Player1,
            PlayerID::Player2,
            PlayerID::Player3,
        ];
        for i in 0..4 {
            let player = board.home(i).clone();

            assert_eq!(player.player_id, Some(player_ids[i as usize].clone()));
            assert_eq!(player.position, -1);
            assert_eq!(player.pieces.len(), pieces.len());
        }
    }

    #[test]
    fn board_goal_spaces_test() {
        let mut board = Board::new();
        for i in 0..4 {
            let state = board.goal(i as i8).clone();
            assert_eq!(state.player_id, None);
            assert_eq!(state.pieces, Vec::default());
            assert_eq!(state.position, 99);
        }
    }

    #[test]
    fn board_outside_spaces_test() {
        let mut board = Board::new();
        for cnt in 0..52 {
            let state = board.outside(cnt).clone();
            assert_eq!(state.player_id, None);
            assert_eq!(state.pieces, Vec::default());
            assert_eq!(state.position, cnt);
        }
    }

    #[test]
    fn board_inside_spaces_test() {
        let mut board = Board::new();
        for cnt in 52..72 {
            let pos = board.inside(cnt).position;
            assert!(board.inside(pos).position == pos);
        }
    }

    #[test]
    fn board_globe_spaces_test() {
        let mut board = Board::new();
        (0..4).for_each(|cnt| {
            let globe = board.globe(cnt).clone();
            assert!(board.is_globe(globe.position));
        });
    }

    #[test]
    fn invincible_test() {
        let mut board = Board::new();
        (0..4).for_each(|cnt| {
            let pos = board.invincible(cnt).clone();
            assert!(board.is_invincible(pos.position));
        });
    }

    #[test]
    fn board_star_spaces_test() {
        let mut board = Board::new();
        (0..8).for_each(|cnt| {
            let star = board.star(cnt).clone();
            assert!(board.is_star(star.position));
        });
    }
}