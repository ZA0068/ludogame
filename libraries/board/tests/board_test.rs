use board::{Board, BoardState, PlayerID};
use pieces::Piece;
use std::any::{Any, TypeId};
use std::{cell::RefCell, rc::Rc};

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
            Rc::new(RefCell::new(Piece::new(0))),
            Rc::new(RefCell::new(Piece::new(1))),
            Rc::new(RefCell::new(Piece::new(2))),
            Rc::new(RefCell::new(Piece::new(3))),
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
}

mod player_id_tests {
    use super::*;

    #[test]
    fn player_id_test() {
        let player_id = PlayerID::Player0;
        assert_eq!(player_id, PlayerID::Player0);
    }

    #[test]
    fn player_id_test_2() {
        let player_id = [
            PlayerID::Player0,
            PlayerID::Player1,
            PlayerID::Player2,
            PlayerID::Player3,
        ];
        let board = Board::default();

        for id in 0..4 {
            assert_eq!(
                board.clone().get_player_id(id),
                Some(player_id[id as usize].clone())
            );
        }
    }
}

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

// #[cfg(test)]
// mod board_update_test {
//     use super::*;

//     #[test]
//     fn move_from_home_test() {
//         let mut board = Board::new();

//         let piece_id = 0;
//         let player_id = 0;
//         let new_position = 0;

//         board.move_from_home(player_id, piece_id, new_position);
//         assert_eq!(board.home(player_id).pieces.len(), 3);
//         assert_eq!(board.outside(new_position).pieces.len(), 1);
//         assert_eq!(
//             board.outside(new_position).player_id,
//             Some(PlayerID::Player0)
//         );
//     }

//     #[test]
//     fn move_all_from_home_test() {
//         let mut board = Board::new();
//         for idx in 0..4 {
//             let piece_id = idx;
//             let player_id = 0;
//             let new_position = 0;
//             board.move_from_home(player_id, piece_id, new_position);
//         }
//         assert!(board.home(0).pieces.is_empty());
//         assert_eq!(board.home(0).player_id, None);
//         assert_eq!(board.outside(0).pieces.len(), 4);
//         assert_eq!(board.outside(0).player_id, Some(PlayerID::Player0));
//     }

//     #[test]
//     fn move_into_home_test() {
//         let mut board = Board::new();

//         let piece_id = 0;
//         let player_id = 0;
//         let new_position = 0;

//         board.move_from_home(player_id, piece_id, new_position);
//         assert_eq!(board.home(player_id).pieces.len(), 3);
//         assert_eq!(board.home(player_id).player_id, Some(PlayerID::Player0));
//         assert_eq!(board.outside(new_position).pieces.len(), 1);
//         assert_eq!(
//             board.outside(new_position).player_id,
//             Some(PlayerID::Player0)
//         );

//         let old_position = 0;
//         board.move_into_home(piece_id, old_position, old_position);
//         assert_eq!(board.home(player_id).pieces.len(), 4);
//         assert_eq!(board.home(player_id).player_id, Some(PlayerID::Player0));
//         assert_eq!(board.outside(old_position).pieces.len(), 0);
//         assert_eq!(board.outside(old_position).player_id, None);
//     }

//     #[test]
//     fn move_pieces_test() {
//         let mut board = Board::new();
//         let piece_id = 0;
//         let player_id = 0;
//         let mut new_position = 0;

//         board.move_from_home(player_id, piece_id, new_position);
//         assert_eq!(board.outside(new_position).pieces.len(), 1);
//         assert_ne!(board.outside(1).pieces.len(), 1);

//         let mut old_position = new_position;
//         new_position = 1;
//         board.update_outside(player_id, piece_id, old_position, new_position);
//         assert_eq!(board.outside(new_position).pieces.len(), 1);
//         assert_ne!(board.outside(old_position).pieces.len(), 1);

//         old_position = new_position;
//         new_position = 9;
//         board.update_outside(player_id, piece_id, old_position, new_position);
//         assert_eq!(board.outside(new_position).pieces.len(), 1);
//         assert_ne!(board.outside(old_position).pieces.len(), 1);

//         old_position = new_position;
//         new_position = 10;
//         board.update_outside(player_id, piece_id, old_position, new_position);
//         assert_eq!(board.outside(new_position).pieces.len(), 1);
//         assert_ne!(board.outside(old_position).pieces.len(), 1);
//     }

//     #[test]
//     fn move_pieces_inside_test() {
//         let mut board = Board::new();
//         let piece_id = 0;
//         let player_id = 0;
//         let mut new_position = 0;

//         board.move_from_home(player_id, piece_id, new_position);

//         let mut old_position = new_position;
//         new_position = 51;

//         board.update_outside(player_id, piece_id, old_position, new_position);

//         old_position = new_position;
//         new_position = 52;
//         board.move_inside(player_id, piece_id, old_position, new_position);

//         assert_eq!(board.inside(new_position).pieces.len(), 1);
//         assert_eq!(
//             board.inside(new_position).piece(piece_id).borrow_mut().id(),
//             0
//         );
//         assert_eq!(board.inside(new_position).position, 52);
//         assert_eq!(board.inside(new_position).state, State::Inside);
//         assert_eq!(board.outside(old_position).pieces.len(), 0);
//     }

//     #[test]
//     fn move_other_pieces_inside_test() {
//         let mut board = Board::new();
//         let piece_id = 3;
//         let player_id = 3;
//         let mut new_position = 0;

//         board.move_from_home(player_id, piece_id, new_position);

//         let mut old_position = new_position;
//         new_position = 51;

//         board.update_outside(player_id, piece_id, old_position, new_position);

//         old_position = new_position;
//         new_position = 67;
//         board.move_inside(player_id, piece_id, old_position, new_position);

//         assert_eq!(board.inside(new_position).pieces.len(), 1);
//         assert_eq!(
//             board.inside(new_position).piece(piece_id).borrow_mut().id(),
//             3
//         );
//         assert_eq!(
//             board.inside(new_position).player_id,
//             Some(PlayerID::Player3)
//         );
//         assert_eq!(board.inside(new_position).position, 67);
//         assert_eq!(board.inside(new_position).state, State::Inside);
//         assert_eq!(board.outside(old_position).pieces.len(), 0);
//     }

//     #[test]
//     fn update_inside_test() {
//         let mut board = Board::new();
//         let piece_id = 2;
//         let player_id = 1;
//         let mut new_position = 0;

//         board.move_from_home(player_id, piece_id, new_position);

//         let mut old_position = new_position;
//         new_position = 51;
//         board.update_outside(player_id, piece_id, old_position, new_position);

//         old_position = new_position;
//         new_position = 52;
//         board.move_inside(player_id, piece_id, old_position, new_position);

//         old_position = new_position;
//         new_position = 56;
//         board.update_inside(player_id, piece_id, old_position, new_position);

//         assert_eq!(board.inside(new_position).pieces.len(), 1);
//         assert_eq!(board.inside(new_position).position, 56);
//         assert_eq!(
//             board.inside(new_position).piece(piece_id).borrow_mut().id(),
//             2
//         );
//         assert_eq!(
//             board.inside(new_position).player_id,
//             Some(PlayerID::Player1)
//         );
//         assert_eq!(board.inside(old_position).pieces.len(), 0);
//         assert_eq!(board.inside(old_position).player_id, None);
//     }

//     #[test]
//     fn move_piece_test_enter_goal() {
//         let mut board = Board::new();
//         let piece_id = 0;
//         let player_id = 0;
//         let mut new_position = 0;

//         board.move_from_home(player_id, piece_id, new_position);

//         let mut old_position = new_position;
//         new_position = 50;
//         board.update_outside(player_id, piece_id, old_position, new_position);

//         old_position = new_position;
//         board.enter_goal(player_id, piece_id, old_position);

//         assert_eq!(board.goal(player_id).pieces.len(), 1);

//         let piece_id = 1;
//         new_position = 0;
//         board.move_from_home(player_id, piece_id, new_position);

//         old_position = new_position;
//         new_position = 50;
//         board.update_outside(player_id, piece_id, old_position, new_position);

//         old_position = new_position;
//         new_position = 56;
//         board.move_inside(player_id, piece_id, old_position, new_position);

//         assert_eq!(board.inside(new_position).pieces.len(), 1);

//         old_position = new_position;
//         board.enter_goal(player_id, piece_id, old_position);
//         assert_eq!(board.goal(player_id).pieces.len(), 2);
//     }

//     #[test]
//     #[should_panic]
//     fn enter_goal_error_test() {
//         let mut board = Board::new();
//         let piece_id = 0;
//         let player_id = 0;
//         let old_position = -1;
//         board.enter_goal(player_id, piece_id, old_position);
//     }

//     #[test]
//     #[should_panic]
//     fn enter_goal_error_2_test() {
//         let mut board = Board::new();
//         let piece_id = 0;
//         let player_id = 0;
//         let old_position = 72;
//         board.enter_goal(player_id, piece_id, old_position);
//     }

//     #[test]
//     #[should_panic]
//     fn enter_goal_error_3_test() {
//         let mut board = Board::new();
//         let piece_id = 0;
//         let player_id = 0;
//         let old_position = 0;
//         board.enter_goal(player_id, piece_id, old_position);
//     }

//     #[test]
//     fn move_all_to_goal_test() {
//         let mut board = Board::new();
//         let mut start_position = 0;
//         let new_position = 50;
//         for player_id in 0..4 {
//             for piece_id in 0..4 {
//                 board.move_from_home(player_id, piece_id, start_position);
//                 board.update_outside(player_id, piece_id, start_position, new_position);

//                 board.enter_goal(player_id, piece_id, new_position);
//             }
//             assert_eq!(board.goal(player_id).pieces.len(), 4);
//             start_position += 13;
//         }
//     }

//     #[test]
//     fn is_occupied_test() {
//         let mut board = Board::new();
//         let piece_id: i8 = 0;
//         let player_id: i8 = 0;
//         let new_position: i8 = 0;
//         board.move_from_home(player_id, piece_id, new_position);
//         assert!(board.is_occupied_self(player_id, new_position));

//         let old_position = new_position;
//         let new_position = 4;
//         board.update_outside(player_id, piece_id, old_position, new_position);
//         assert!(board.is_occupied_self(player_id, new_position));
//         assert!(!board.is_occupied_self(player_id, old_position));
//     }

//     #[test]
//     fn is_occupied_by_more_test() {
//         let piece_1: i8 = 0;
//         let piece_2: i8 = 1;
//         let player_id: i8 = 0;
//         let new_position: i8 = 0;
//         let mut board = Board::new();

//         board.move_from_home(player_id, piece_1, new_position);
//         board.move_from_home(player_id, piece_2, new_position);
//         assert!(board.is_occupied_more(new_position));

//         let old_position = new_position;
//         let new_position = 4;
//         board.update_outside(player_id, piece_1, old_position, new_position);
//         assert!(!board.is_occupied_more(new_position));
//         assert!(!board.is_occupied_more(old_position));

//         board.update_outside(player_id, piece_2, old_position, new_position);
//         assert!(board.is_occupied_more(new_position));
//     }

//     #[test]
//     fn is_occupied_by_other_test() {
//         let mut board = Board::new();
//         let piece_id: i8 = 0;
//         let player_0: i8 = 0;
//         let player_1: i8 = 1;
//         let new_position: i8 = 0;

//         board.move_from_home(player_0, piece_id, new_position);
//         assert!(board.is_occupied_by_other(player_1, new_position));

//         let new_position: i8 = 4;
//         board.move_from_home(player_1, piece_id, new_position);
//         assert!(board.is_occupied_by_other(player_0, new_position));
//     }

//     #[test]
//     fn is_occupied_by_other_more_test() {
//         let mut board = Board::new();
//         let piece_0: i8 = 0;
//         let piece_1: i8 = 1;
//         let player_0: i8 = 0;
//         let player_1: i8 = 1;
//         let new_position: i8 = 0;

//         board.move_from_home(player_0, piece_0, new_position);
//         board.move_from_home(player_0, piece_1, new_position);
//         assert!(board.is_occupied_by_more_other(player_1, new_position));

//         let new_position: i8 = 4;
//         board.move_from_home(player_1, piece_0, new_position);
//         board.move_from_home(player_1, piece_1, new_position);
//         assert!(board.is_occupied_by_more_other(player_0, new_position));
//     }

//     #[test]
//     fn internal_update_test() {
//         let mut board = Board::new();
//         let piece_id: i8 = 0;
//         let player_id: i8 = 0;
//         let new_position: i8 = 0;

//         board.home(player_id).piece(piece_id).borrow_mut().free();
//         board.move_from_home(player_id, piece_id, new_position);
//         assert_eq!(board.outside(new_position).pieces.len(), 1);
//         assert_eq!(
//             board
//                 .outside(new_position)
//                 .piece(piece_id)
//                 .borrow_mut()
//                 .id(),
//             piece_id
//         );
//         assert!(!board
//             .outside(new_position)
//             .piece(piece_id)
//             .borrow_mut()
//             .is_home());
//         assert!(board
//             .outside(new_position)
//             .piece(piece_id)
//             .borrow_mut()
//             .is_dangerous());
//         assert_eq!(
//             board
//                 .outside(new_position)
//                 .piece(piece_id)
//                 .borrow_mut()
//                 .position(),
//             0
//         );

//         let old_position = new_position;
//         let new_position = 4;
//         board
//             .outside(player_id)
//             .piece(piece_id)
//             .borrow_mut()
//             .not_safe();
//         board.update_outside(player_id, piece_id, old_position, new_position);
//         assert_eq!(board.outside(old_position).pieces.len(), 0);
//         assert_eq!(board.outside(new_position).pieces.len(), 1);
//         assert_eq!(
//             board
//                 .outside(new_position)
//                 .piece(piece_id)
//                 .borrow_mut()
//                 .id(),
//             piece_id
//         );
//         assert!(!board
//             .outside(new_position)
//             .piece(piece_id)
//             .borrow()
//             .is_dangerous());
//         assert!(!board
//             .outside(new_position)
//             .piece(piece_id)
//             .borrow()
//             .is_safe());
//     }
//
