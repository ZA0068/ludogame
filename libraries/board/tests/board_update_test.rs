use board::{Board, PlayerID};
use rand::{thread_rng, Rng};

#[cfg(test)]
mod board_update_test {
    use super::*;

    #[test]
    fn move_from_home_test() {
        let mut board = Board::new();
        let piece_id = 0;
        let player_id = 0;
        let new_position = 0;

        board.move_from_home(player_id, piece_id, new_position);
        assert_eq!(board.home(player_id).pieces.len(), 3);
        assert_eq!(board.outside(new_position).pieces.len(), 1);
        assert_eq!(board.invincible(new_position).pieces.len(), 1);
        assert_eq!(
            board.outside(new_position).player_id,
            Some(PlayerID::Player0)
        );
        assert_eq!(board.outside(new_position).pieces[0].borrow().id(), 0);
        assert_eq!(board.outside(new_position).piece(piece_id).borrow().id(), 0);
        assert_eq!(
            !board
                .outside(new_position)
                .piece(piece_id)
                .borrow()
                .position(),
            0
        );
    }

    #[test]
    fn move_all_from_home_test() {
        let mut board = Board::new();
        for piece_id in 0..4 {
            let player_id = 0;
            let new_position = 0;
            board.move_from_home(player_id, piece_id, new_position);
            assert_eq!(board.outside(0).piece(piece_id).borrow().id(), piece_id);
        }
        assert!(board.home(0).pieces.is_empty());
        assert_eq!(board.home(0).player_id, None);
        assert_eq!(board.outside(0).pieces.len(), 4);
        assert_eq!(board.invincible(0).pieces.len(), 4);
        assert_eq!(board.outside(0).player_id, Some(PlayerID::Player0));
    }

    #[test]
    fn move_all_from_home_test2() {
        let mut board = Board::new();
        let player_ids = vec![
            PlayerID::Player0,
            PlayerID::Player1,
            PlayerID::Player2,
            PlayerID::Player3,
        ];
        for player_id in 0..4 {
            for piece_id in 0..4 {
                let new_position = board.invincible[player_id as usize] as i8;
                board.move_from_home(player_id, piece_id, new_position);
                assert_eq!(
                    board.outside(new_position).piece(piece_id).borrow().id(),
                    piece_id
                );
                assert_eq!(
                    board.invincible(player_id).piece(piece_id).borrow().id(),
                    piece_id
                );
            }
            assert_eq!(board.invincible(player_id).pieces.len(), 4);
            assert_eq!(
                board
                    .outside(board.invincible[player_id as usize] as i8)
                    .pieces
                    .len(),
                4
            );
            assert!(board.home(player_id).pieces.is_empty());
            assert_eq!(board.home(player_id).player_id, None);
            assert_eq!(
                board.invincible(player_id).player_id,
                Some(player_ids[player_id as usize].clone())
            );
        }
    }

    #[test]
    fn move_into_home_test() {
        let mut board = Board::new();

        let piece_id = 0;
        let player_id = 0;
        let new_position = 0;

        board.move_from_home(player_id, piece_id, new_position);
        assert_eq!(board.home(player_id).pieces.len(), 3);
        assert_eq!(board.home(player_id).player_id, Some(PlayerID::Player0));
        assert_eq!(board.outside(new_position).pieces.len(), 1);
        assert_eq!(
            board.outside(new_position).player_id,
            Some(PlayerID::Player0)
        );

        let old_position = 0;
        board.move_into_home(piece_id, old_position, old_position);
        assert_eq!(board.home(player_id).pieces.len(), 4);
        assert_eq!(board.home(player_id).player_id, Some(PlayerID::Player0));
        assert_eq!(board.outside(old_position).pieces.len(), 0);
        assert_eq!(board.outside(old_position).player_id, None);
    }

    #[test]
    fn move_all_into_home_test() {
        let mut board = Board::new();
        let player_id = 0;
        for piece_id in 0..4 {
            let new_position = 0;
            board.move_from_home(player_id, piece_id, new_position);
            assert_eq!(board.outside(0).piece(piece_id).borrow().id(), piece_id);
        }
        assert!(board.home(player_id).pieces.is_empty());
        assert_eq!(board.home(player_id).player_id, None);
        assert_eq!(board.outside(0).pieces.len(), 4);
        assert_eq!(board.invincible(0).pieces.len(), 4);
        assert_eq!(board.outside(0).player_id, Some(PlayerID::Player0));

        for piece_id in 0..4 {
            let old_position = 0;
            board.move_into_home(0, piece_id, old_position);
            assert_eq!(board.home(player_id).pieces.len(), (piece_id as usize) + 1);
            assert_eq!(board.home(player_id).player_id, Some(PlayerID::Player0));
            assert_eq!(
                board.outside(old_position).pieces.len(),
                4 - (piece_id as usize) - 1
            );
        }
        assert_eq!(board.home(player_id).pieces.len(), 4);
        assert_eq!(board.home(player_id).player_id, Some(PlayerID::Player0));
        assert_eq!(board.outside(0).pieces.len(), 0);
        assert_eq!(board.outside(0).player_id, None);
    }

    #[test]
    fn move_all_into_home_test_2() {
        let mut board = Board::new();
        let player_ids = vec![
            PlayerID::Player0,
            PlayerID::Player1,
            PlayerID::Player2,
            PlayerID::Player3,
        ];
        for player_id in 0..4 {
            let new_position = board.invincible[player_id as usize] as i8;
            for piece_id in 0..4 {
                board.move_from_home(player_id, piece_id, new_position);
                assert_eq!(
                    board.outside(new_position).piece(piece_id).borrow().id(),
                    piece_id
                );
                assert_eq!(
                    board.invincible(player_id).piece(piece_id).borrow().id(),
                    piece_id
                );
            }
            assert!(board.home(player_id).pieces.is_empty());
            assert_eq!(board.home(player_id).player_id, None);
            assert_eq!(board.outside(new_position).pieces.len(), 4);
            assert_eq!(board.invincible(player_id).pieces.len(), 4);
            assert_eq!(
                board.outside(new_position).player_id,
                Some(player_ids[player_id as usize].clone())
            );

            let old_position = new_position;
            for piece_id in 0..4 {
                board.move_into_home(player_id, piece_id, old_position);
                assert_eq!(board.home(player_id).pieces.len(), (piece_id as usize) + 1);
                assert_eq!(
                    board.home(player_id).player_id,
                    Some(player_ids[player_id as usize].clone())
                );
                assert_eq!(
                    board.outside(old_position).pieces.len(),
                    4 - (piece_id as usize) - 1
                );
            }
            assert_eq!(board.home(player_id).pieces.len(), 4);
            assert_eq!(
                board.home(player_id).player_id,
                Some(player_ids[player_id as usize].clone())
            );
            assert_eq!(board.outside(old_position).pieces.len(), 0);
            assert_eq!(board.outside(old_position).player_id, None);
            assert_eq!(board.invincible(player_id).player_id, None);
        }
    }

    #[test]
    fn update_outside_test() {
        let mut board = Board::new();
        let piece_id = 0;
        let player_id = 0;
        let mut new_position = 0;

        board.move_from_home(player_id, piece_id, new_position);
        assert_eq!(board.outside(new_position).pieces.len(), 1);
        assert_ne!(board.outside(1).pieces.len(), 1);

        let mut old_position = new_position;
        new_position = 1;
        board.update_outside(player_id, piece_id, old_position, new_position);
        assert_eq!(board.outside(new_position).pieces.len(), 1);
        assert_ne!(board.outside(old_position).pieces.len(), 1);

        old_position = new_position;
        new_position = 9;
        board.update_outside(player_id, piece_id, old_position, new_position);
        assert_eq!(board.outside(new_position).pieces.len(), 1);
        assert_ne!(board.outside(old_position).pieces.len(), 1);

        old_position = new_position;
        new_position = 10;
        board.update_outside(player_id, piece_id, old_position, new_position);
        assert_eq!(board.outside(new_position).pieces.len(), 1);
        assert_ne!(board.outside(old_position).pieces.len(), 1);
    }

    #[test]
    fn update_outside_test_2() {
        let mut board = Board::new();
        for player_id in 0..4 {
            let start_position = board.invincible[player_id as usize] as i8;
            for piece_id in 0..4 {
                board.move_from_home(player_id, piece_id, start_position);
                assert_eq!(
                    board.outside(start_position).pieces.len(),
                    piece_id as usize + 1
                );
                assert!(board.outside(start_position + 1).pieces.len() < piece_id as usize + 1);
            }
            let mut old_position = start_position;
            let mut new_position = old_position + 1;
            for piece_id in 0..4 {
                board.update_outside(player_id, piece_id, old_position, new_position);
                assert_eq!(
                    board.outside(new_position).pieces.len(),
                    piece_id as usize + 1
                );
                assert!(board.outside(old_position).pieces.len() <= 3 - piece_id as usize);
            }
            old_position = new_position;
            new_position = old_position + 9;
            for piece_id in 0..4 {
                board.update_outside(player_id, piece_id, old_position, new_position);
                assert_eq!(
                    board.outside(new_position).pieces.len(),
                    piece_id as usize + 1
                );
                assert!(board.outside(old_position).pieces.len() <= 3 - piece_id as usize);
            }
            old_position = new_position;
            new_position = old_position + 10;
            for piece_id in 0..4 {
                board.update_outside(player_id, piece_id, old_position, new_position);
                assert_eq!(
                    board.outside(new_position).pieces.len(),
                    piece_id as usize + 1
                );
                assert!(board.outside(old_position).pieces.len() <= 3 - piece_id as usize);
            }
        }
    }

    #[test]
    fn update_outside_test_3() {
        let mut board = Board::new();
        let player_ids = vec![
            PlayerID::Player0,
            PlayerID::Player1,
            PlayerID::Player2,
            PlayerID::Player3,
        ];
        for player_id in 0..4 {
            let start_position = board.invincible[player_id as usize] as i8;
            for piece_id in 0..4 {
                board.move_from_home(player_id, piece_id, start_position);
                assert_eq!(
                    board.outside(start_position).pieces.len(),
                    piece_id as usize + 1
                );
            }
            assert_eq!(board.outside(start_position).pieces.len(), 4);
            let mut old_position = start_position;
            for _ in 0..52 {
                let new_position = old_position + 1;
                for piece_id in 0..4 {
                    board.update_outside(player_id, piece_id, old_position, new_position);
                    assert_eq!(
                        board.outside(new_position).player_id,
                        Some(player_ids[player_id as usize].clone())
                    );
                    assert_eq!(
                        board.outside(new_position).pieces.len(),
                        1 + (piece_id as usize)
                    );
                    assert_eq!(
                        board.outside(old_position).pieces.len(),
                        3 - (piece_id as usize)
                    );
                }
                assert_eq!(board.outside(new_position).pieces.len(), 4);
                old_position = new_position;
            }

            for piece_id in 0..4 {
                board.move_into_home(player_id, piece_id, old_position);
                assert_eq!(
                    board.outside(old_position).pieces.len(),
                    3 - (piece_id as usize)
                );
            }
            assert_eq!(board.outside(old_position).player_id, None);
            assert!(board.outside(old_position).pieces.is_empty());
        }
    }

    #[test]
    fn move_pieces_inside_test() {
        let mut board = Board::new();
        let piece_id = 0;
        let player_id = 0;
        let mut new_position = 0;

        board.move_from_home(player_id, piece_id, new_position);

        let mut old_position = new_position;
        new_position = 51;

        board.update_outside(player_id, piece_id, old_position, new_position);

        old_position = new_position;
        new_position = 52;
        board.move_inside(player_id, piece_id, old_position, new_position);

        assert_eq!(board.inside(new_position).pieces.len(), 1);
        assert_eq!(
            board.inside(new_position).piece(piece_id).borrow_mut().id(),
            0
        );
        assert_eq!(board.inside(new_position).position, 52);
        assert_eq!(board.outside(old_position).pieces.len(), 0);
    }

    #[test]
    fn move_other_pieces_inside_test() {
        let mut board = Board::new();
        let piece_id = 3;
        let player_id = 3;
        let mut new_position = 0;

        board.move_from_home(player_id, piece_id, new_position);

        let mut old_position = new_position;
        new_position = 51;

        board.update_outside(player_id, piece_id, old_position, new_position);

        old_position = new_position;
        new_position = 67;
        board.move_inside(player_id, piece_id, old_position, new_position);

        assert_eq!(board.inside(new_position).pieces.len(), 1);
        assert_eq!(
            board.inside(new_position).piece(piece_id).borrow_mut().id(),
            3
        );
        assert_eq!(
            board.inside(new_position).player_id,
            Some(PlayerID::Player3)
        );
        assert_eq!(board.inside(new_position).position, 67);
        assert_eq!(board.outside(old_position).pieces.len(), 0);
    }

    #[test]
    fn move_all_inside_test() {
        let mut board = Board::new();
        let piece_id = 0;
        for player_id in 0..4 {
            let mut new_position = board.invincible[player_id as usize] as i8;

            board.move_from_home(player_id, piece_id, new_position);

            let mut old_position = new_position;
            new_position = 51;

            board.update_outside(player_id, piece_id, old_position, new_position);

            old_position = new_position;
            new_position = 52 + player_id * 5;
            board.move_inside(player_id, piece_id, old_position, new_position);

            assert_eq!(board.inside(new_position).pieces.len(), 1);
            assert_eq!(
                board.inside(new_position).piece(piece_id).borrow_mut().id(),
                piece_id
            );
            assert_eq!(board.inside(new_position).position, 52 + player_id * 5);
            assert_eq!(board.outside(old_position).pieces.len(), 0);
        }
    }

    #[test]
    fn update_inside_test() {
        let mut board = Board::new();
        let piece_id = 0;
        let player_id = 0;
        let mut new_position = 0;

        board.move_from_home(player_id, piece_id, new_position);

        let mut old_position = new_position;
        new_position = 51;
        board.update_outside(player_id, piece_id, old_position, new_position);

        old_position = new_position;
        new_position = 52;
        board.move_inside(player_id, piece_id, old_position, new_position);

        old_position = new_position;
        new_position = 56;
        board.update_inside(player_id, piece_id, old_position, new_position);

        assert_eq!(board.inside(new_position).pieces.len(), 1);
        assert_eq!(board.inside(new_position).position, 56);
        assert_eq!(
            board.inside(new_position).piece(piece_id).borrow_mut().id(),
            0
        );
        assert_eq!(
            board.inside(new_position).player_id,
            Some(PlayerID::Player0)
        );
        assert_eq!(board.inside(old_position).pieces.len(), 0);
        assert_eq!(board.inside(old_position).player_id, None);
    }

    #[test]
    fn update_all_inside_test() {
        let mut board = Board::new();
        let piece_id = 0;
        let player_ids = vec![
            PlayerID::Player0,
            PlayerID::Player1,
            PlayerID::Player2,
            PlayerID::Player3,
        ];

        for player_id in 0..4 {
            let mut new_position = board.invincible[player_id as usize] as i8;
            board.move_from_home(player_id, piece_id, new_position);

            let mut old_position = new_position;
            new_position = 51;
            board.update_outside(player_id, piece_id, old_position, new_position);

            old_position = new_position;
            new_position = 52 + player_id * 5;
            board.move_inside(player_id, piece_id, old_position, new_position);

            for i in 0..4 {
                old_position = new_position;
                new_position += 1;
                board.update_inside(player_id, piece_id, old_position, new_position);
                assert_eq!(board.inside(new_position).pieces.len(), 1);
                assert_eq!(
                    board.inside(new_position).position,
                    52 + player_id * 5 + i + 1
                );
                assert_eq!(
                    board.inside(new_position).piece(piece_id).borrow_mut().id(),
                    0
                );
                assert_eq!(
                    board.inside(new_position).player_id,
                    Some(player_ids[player_id as usize].clone())
                );
                assert_eq!(board.inside(old_position).pieces.len(), 0);
                assert_eq!(board.inside(old_position).player_id, None);
            }

            for i in 0..4 {
                old_position = new_position;
                new_position -= 1;
                board.update_inside(player_id, piece_id, old_position, new_position);
                assert_eq!(board.inside(new_position).pieces.len(), 1);
                assert_eq!(
                    board.inside(new_position).position,
                    56 + player_id * 5 - i - 1
                );
                assert_eq!(
                    board.inside(new_position).piece(piece_id).borrow_mut().id(),
                    0
                );
                assert_eq!(
                    board.inside(new_position).player_id,
                    Some(player_ids[player_id as usize].clone())
                );
                assert_eq!(board.inside(old_position).pieces.len(), 0);
                assert_eq!(board.inside(old_position).player_id, None);
            }
        }
    }

    #[test]
    fn enter_goal_test() {
        let mut board = Board::new();
        let piece_id = 0;
        let player_id = 0;
        let mut new_position = 0;

        board.move_from_home(player_id, piece_id, new_position);

        let mut old_position = new_position;
        new_position = 50;
        board.update_outside(player_id, piece_id, old_position, new_position);

        old_position = new_position;
        board.enter_goal(player_id, piece_id, old_position);

        assert_eq!(board.goal(player_id).pieces.len(), 1);

        let piece_id = 1;
        new_position = 0;
        board.move_from_home(player_id, piece_id, new_position);

        old_position = new_position;
        new_position = 50;
        board.update_outside(player_id, piece_id, old_position, new_position);

        old_position = new_position;
        new_position = 56;
        board.move_inside(player_id, piece_id, old_position, new_position);

        assert_eq!(board.inside(new_position).pieces.len(), 1);

        old_position = new_position;
        board.enter_goal(player_id, piece_id, old_position);
        assert_eq!(board.goal(player_id).pieces.len(), 2);
    }

    #[test]
    fn enter_all_goal_test() {
        let mut board = Board::new();

        for player_id in 0..4 {
            let piece_id = 0;
            let mut new_position = board.invincible[player_id as usize] as i8;
            board.move_from_home(player_id, piece_id, new_position);

            let mut old_position = new_position;
            new_position = (old_position + 50) % 52;
            board.update_outside(player_id, piece_id, old_position, new_position);

            old_position = new_position;
            board.enter_goal(player_id, piece_id, old_position);

            assert_eq!(board.goal(player_id).pieces.len(), 1);

            let piece_id = 1;
            new_position = board.invincible[player_id as usize] as i8;
            board.move_from_home(player_id, piece_id, new_position);

            old_position = new_position;
            new_position = (old_position + 50) % 52;
            board.update_outside(player_id, piece_id, old_position, new_position);

            old_position = new_position;
            new_position = 51 + 5 * (player_id + 1);
            board.move_inside(player_id, piece_id, old_position, new_position);

            assert_eq!(board.inside(new_position).pieces.len(), 1);

            old_position = new_position;
            board.enter_goal(player_id, piece_id, old_position);
            assert_eq!(board.goal(player_id).pieces.len(), 2);
        }
    }

    #[test]
    #[should_panic]
    fn enter_goal_error_test() {
        let mut board = Board::new();
        let piece_id = 0;
        let player_id = 0;
        let old_position = -1;
        board.enter_goal(player_id, piece_id, old_position);
    }

    #[test]
    #[should_panic]
    fn enter_goal_error_2_test() {
        let mut board = Board::new();
        let piece_id = 0;
        let player_id = 0;
        board.move_from_home(player_id, piece_id, 0);
        let old_position = -1;
        board.enter_goal(player_id, piece_id, old_position);
    }

    #[test]
    #[should_panic]
    fn enter_goal_error_3_test() {
        let mut board = Board::new();
        let piece_id = 0;
        let player_id = 0;
        let position = 0;
        board.move_from_home(player_id, piece_id, position);
        let position = 72;
        board.enter_goal(player_id, piece_id, position);
    }

    #[test]
    #[should_panic]
    fn enter_goal_error_4_test() {
        let mut board = Board::new();
        let piece_id = 0;
        let player_id = 0;
        let position = 0;
        board.move_from_home(player_id, piece_id, position);
        let position = 99;
        board.enter_goal(player_id, piece_id, position);
    }

    #[test]
    #[should_panic]
    fn enter_goal_error_5_test() {
        let mut board = Board::new();
        let piece_id = 0;
        let player_id = 0;
        let position = 0;
        board.move_from_home(player_id, piece_id, position);
        let position = 98;
        board.enter_goal(player_id, piece_id, position);
    }

    #[test]
    #[should_panic]
    fn enter_goal_error_6_test() {
        let mut board = Board::new();
        let piece_id = 0;
        let player_id = 0;
        let old_position = 0;
        board.enter_goal(player_id, piece_id, old_position);
    }

    #[test]
    fn move_all_to_goal_test() {
        let mut board = Board::new();
        let mut start_position = 0;
        let new_position = 50;
        for player_id in 0..4 {
            for piece_id in 0..4 {
                board.move_from_home(player_id, piece_id, start_position);
                board.update_outside(player_id, piece_id, start_position, new_position);

                board.enter_goal(player_id, piece_id, new_position);
            }
            assert_eq!(board.goal(player_id).pieces.len(), 4);
            start_position += 13;
        }
    }

    #[test]
    fn is_self_occupied_test() {
        let mut board = Board::new();
        let piece_id: i8 = 0;
        let player_id: i8 = 0;
        let new_position: i8 = 0;
        board.move_from_home(player_id, piece_id, new_position);
        assert!(board.is_occupied_self(player_id, new_position));

        let old_position = new_position;
        let new_position = 4;
        board.update_outside(player_id, piece_id, old_position, new_position);
        assert!(board.is_occupied_self(player_id, new_position));
        assert!(!board.is_occupied_self(player_id, old_position));
    }

    #[test]
    fn is_self_occupied_more_test() {
        let piece_1: i8 = 0;
        let piece_2: i8 = 1;
        let player_id: i8 = 0;
        let new_position: i8 = 0;
        let mut board = Board::new();

        board.move_from_home(player_id, piece_1, new_position);
        board.move_from_home(player_id, piece_2, new_position);
        assert!(board.is_occupied_by_more_self(player_id, new_position));

        let old_position = new_position;
        let new_position = 4;
        board.update_outside(player_id, piece_1, old_position, new_position);
        assert!(!board.is_occupied_by_more_self(player_id, new_position));
        assert!(!board.is_occupied_by_more_self(player_id, old_position));

        board.update_outside(player_id, piece_2, old_position, new_position);
        assert!(board.is_occupied_by_more_self(player_id, new_position));
    }

    #[test]
    fn is_occupied_by_other_test() {
        let mut board = Board::new();
        let piece_id: i8 = 0;
        let player_0: i8 = 0;
        let player_1: i8 = 1;
        let new_position: i8 = 0;

        board.move_from_home(player_0, piece_id, new_position);
        assert!(board.is_occupied_by_other(player_1, new_position));

        let new_position: i8 = 4;
        board.move_from_home(player_1, piece_id, new_position);
        assert!(board.is_occupied_by_other(player_0, new_position));
        assert!(!board.is_occupied_by_other(player_0, new_position + 1));
    }

    #[test]
    fn is_occupied_by_other_more_test() {
        let mut board = Board::new();
        let piece_0: i8 = 0;
        let piece_1: i8 = 1;
        let player_0: i8 = 0;
        let player_1: i8 = 1;
        let new_position: i8 = 0;

        board.move_from_home(player_0, piece_0, new_position);
        board.move_from_home(player_0, piece_1, new_position);
        assert!(board.is_occupied_by_other_more(player_1, new_position));

        let new_position: i8 = 4;
        board.move_from_home(player_1, piece_0, new_position);
        board.move_from_home(player_1, piece_1, new_position);
        assert!(board.is_occupied_by_other_more(player_0, new_position));
        assert!(!board.is_occupied_by_other_more(player_0, new_position + 1));
    }

    #[test]
    fn internal_update_test() {
        let mut board = Board::new();
        let piece_id: i8 = 0;
        let player_id: i8 = 0;
        let new_position: i8 = 0;

        board.home(player_id).piece(piece_id).borrow_mut().free();
        board
            .home(player_id)
            .piece(piece_id)
            .borrow_mut()
            .set_position(new_position);
        board.move_from_home(player_id, piece_id, new_position);
        assert_eq!(board.outside(new_position).pieces.len(), 1);
        assert_eq!(
            board
                .outside(new_position)
                .piece(piece_id)
                .borrow_mut()
                .id(),
            piece_id
        );
        assert!(!board
            .outside(new_position)
            .piece(piece_id)
            .borrow_mut()
            .is_home());
        assert!(board
            .outside(new_position)
            .piece(piece_id)
            .borrow_mut()
            .is_free());
        assert_eq!(
            board
                .outside(new_position)
                .piece(piece_id)
                .borrow_mut()
                .position(),
            0
        );

        let old_position = new_position;
        let new_position = 4;
        board.update_outside(player_id, piece_id, old_position, new_position);
        assert_eq!(board.outside(old_position).pieces.len(), 0);
        assert_eq!(board.outside(new_position).pieces.len(), 1);
        assert_eq!(
            board
                .outside(new_position)
                .piece(piece_id)
                .borrow_mut()
                .id(),
            piece_id
        );
        assert!(board
            .outside(new_position)
            .piece(piece_id)
            .borrow()
            .is_free());
    }

    #[test]
    fn reset_board_test() {
        let mut rng = thread_rng();
        let mut board = Board::new();
        for player_id in 0..4 {
            let new_position: i8 = board.invincible[player_id as usize] as i8;
            for piece_id in 0..4 {
                board.move_from_home(player_id, piece_id, new_position)
            }

            let mut old_position1 = new_position;
            let mut old_position2 = new_position;
            board.enter_goal(player_id, 3, old_position1);
            let new_position = 52 + player_id * 5;
            board.move_inside(player_id, 0, old_position1, new_position);
            let mut old_inside_position = new_position;

            for _ in 0..10 {
                let new_position1 = rng.gen_range(0..=51);
                let new_position2 = rng.gen_range(0..=51);
                let new_inside_position = rng.gen_range(52..72);
                board.update_outside(player_id, 1, old_position1, new_position1);
                board.update_outside(player_id, 2, old_position2, new_position2);
                board.update_inside(player_id, 0, old_inside_position, new_inside_position);
                old_position1 = new_position1;
                old_position2 = new_position2;
                old_inside_position = new_inside_position;
            }
        }

        board.reset();
        let player_ids = vec![
            PlayerID::Player0,
            PlayerID::Player1,
            PlayerID::Player2,
            PlayerID::Player3,
        ];
        for i in 0..4 {
            assert_eq!(board.home(i).pieces.len(), 4);
            for j in 0..4 {
                assert_eq!(board.home(i).piece(j).borrow().id(), j);
                assert_eq!(
                    board.home(i).player_id,
                    Some(player_ids[i as usize].clone())
                );
                assert!(board.home(i).piece(j).borrow().is_home());
                assert!(!board.home(i).piece(j).borrow().is_free());
                assert!(!board.home(i).piece(j).borrow().is_goal());
                assert_eq!(board.home(i).piece(j).borrow().position(), -1);
            }
        }
    }

    #[test]
    fn reset_board_test_temp() {
        let mut rng = thread_rng();
        let mut board = Board::new();
        for player_id in 0..4 {
            let new_position: i8 = board.invincible[player_id as usize] as i8;
            for piece_id in 0..4 {
                board.move_from_home(player_id, piece_id, new_position);
                board
                    .outside(new_position)
                    .piece(piece_id)
                    .borrow_mut()
                    .free();
                board
                    .outside(new_position)
                    .piece(piece_id)
                    .borrow_mut()
                    .set_position(new_position)
            }

            let mut old_position1 = new_position;
            let mut old_position2 = new_position;
            board.enter_goal(player_id, 3, old_position1);
            board.goal(player_id).piece(3).borrow_mut().goal();

            let new_position = 52 + player_id * 5;
            board.move_inside(player_id, 0, old_position1, new_position);
            board
                .inside(new_position)
                .piece(0)
                .borrow_mut()
                .set_position(new_position);
            let mut old_inside_position = new_position;

            for _ in 0..10 {
                let new_position1 = rng.gen_range(0..=51);
                let new_position2 = rng.gen_range(0..=51);
                let new_inside_position = rng.gen_range(52..72);
                board.update_outside(player_id, 1, old_position1, new_position1);
                board.update_outside(player_id, 2, old_position2, new_position2);
                board.update_inside(player_id, 0, old_inside_position, new_inside_position);
                board
                    .outside(new_position1)
                    .piece(1)
                    .borrow_mut()
                    .set_position(new_position1);
                board
                    .outside(new_position2)
                    .piece(2)
                    .borrow_mut()
                    .set_position(new_position2);
                board
                    .inside(new_inside_position)
                    .piece(0)
                    .borrow_mut()
                    .set_position(new_inside_position);
                old_position1 = new_position1;
                old_position2 = new_position2;
                old_inside_position = new_inside_position;
            }
        }

        board.reset();
        let player_ids = vec![
            PlayerID::Player0,
            PlayerID::Player1,
            PlayerID::Player2,
            PlayerID::Player3,
        ];
        for i in 0..4 {
            assert_eq!(board.home(i).pieces.len(), 4);
            for j in 0..4 {
                assert_eq!(board.home(i).piece(j).borrow().id(), j);
                assert_eq!(
                    board.home(i).player_id,
                    Some(player_ids[i as usize].clone())
                );
                assert!(board.home(i).piece(j).borrow().is_home());
                assert!(!board.home(i).piece(j).borrow().is_free());
                assert!(!board.home(i).piece(j).borrow().is_goal());
                assert_eq!(board.home(i).piece(j).borrow().position(), -1);
            }
        }
    }
}
