use board::Board;
use pieces::Color;
use dice::Dice;
use players::Player;
use std::{cell::RefCell, rc::Rc};

#[cfg(test)]
mod player_0_tests {

    use super::*;
    static PLAYER_ID: i8 = 0;

    #[test]
    fn add_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(PLAYER_ID, board.clone());
        assert_eq!(player.id(), 0);
        assert_eq!(player.board().as_ptr(), board.as_ptr());
    }

    #[test]
    fn get_pieces_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(PLAYER_ID, board);
        (0..4).for_each(|i| {
            let piece = player.piece(i);
            assert_eq!(piece.borrow().id(), i);
            assert_eq!(piece.borrow().color(), Color::Green);
            assert_ne!(piece.borrow().color(), Color::Yellow);
            assert_ne!(piece.borrow().color(), Color::Blue);
            assert_ne!(piece.borrow().color(), Color::Red);
        });
    }

    #[test]
    #[should_panic]
    fn get_pieces_test_2() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(PLAYER_ID, board);
        let piece = player.piece(0);
        assert_eq!(piece.borrow().id(), 0);
        assert_eq!(piece.borrow().color(), Color::Yellow);
    }

    #[test]
    #[should_panic]
    fn get_pieces_test_3() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(PLAYER_ID, board);
        let piece = player.piece(0);
        assert_eq!(piece.borrow().id(), 0);
        assert_eq!(piece.borrow().color(), Color::Red);
    }

    #[test]
    #[should_panic]
    fn get_pieces_test_4() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(PLAYER_ID, board);
        let piece = player.piece(0);
        assert_eq!(piece.borrow().id(), 0);
        assert_eq!(piece.borrow().color(), Color::Blue);
    }

    #[test]
    #[should_panic]
    fn get_pieces_test_5() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(PLAYER_ID, board);
        let piece = player.piece(0);
        assert_eq!(piece.borrow().id(), 0);
        assert_ne!(piece.borrow().color(), Color::Green);
    }

    #[test]
    fn player_with_dice_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        let result = player.roll_dice();
        assert!(result == 0);

        let dice = Dice::default();

        player.take_dice(dice);
        let result = player.roll_dice();
        assert!(result > 0 && result < 7);

        player.give_dice();
        let result = player.roll_dice();
        assert!(result == 0);
    }

    #[test]
    fn move_piece_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let piece_id = 0;
        player.free_piece(piece_id);
        for i in 0..50 {
            player.move_piece(piece_id, 1);
            assert_eq!(player.piece(piece_id).borrow().position(), i + 1);
            assert_eq!(
                player.board().borrow_mut().outside(i + 1).pieces.len(),
                1
            );
            assert_eq!(
                player.board().borrow_mut().outside(i + 1).player_id,
                Some(board::PlayerID::Player0)
            );
            assert_eq!(player.board().borrow_mut().outside(i).pieces.len(), 0);
        }
        player.enter_inside(piece_id, 50, 52);
        let vec = (52..=57).chain((52..57).rev()).collect::<Vec<_>>();
        for i in 52..62 {
            let oldpos = player.piece(piece_id).borrow().position();
            if (i + 1) % 57 == 0 {
                continue;
            }

            player.update_piece(piece_id, oldpos, i + 1);
            assert_eq!(player.piece(piece_id).borrow().position(), vec[i as usize - 51]);
            assert_eq!(
                player.board().borrow_mut().inside(vec[i as usize - 51]).pieces.len(),
                1
            );
            assert_eq!(
                player.board().borrow_mut().inside(vec[i as usize - 51]).player_id,
                Some(board::PlayerID::Player0)
            );
            assert_eq!(player.board().borrow_mut().inside(vec[i as usize - 52]).pieces.len(), 0);
        }
    }

    #[test]
    fn move_piece_test_2() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let piece_id = 0;
        player.free_piece(piece_id);
        for i in 0..50 {
            player.move_piece(piece_id, 1);
            assert_eq!(player.piece(piece_id).borrow().position(), i + 1);
            assert_eq!(
                player.board().borrow_mut().outside(i + 1).pieces.len(),
                1
            );
            assert_eq!(
                player.board().borrow_mut().outside(i + 1).player_id,
                Some(board::PlayerID::Player0)
            );
            assert_eq!(player.board().borrow_mut().outside(i).pieces.len(), 0);
        }
        player.enter_inside(piece_id, 50, 52);
        let vec = (52..=57).collect::<Vec<_>>();
        for i in 52..56 {
            let oldpos = player.piece(piece_id).borrow().position();

            player.update_piece(piece_id, oldpos, i + 1);
            assert_eq!(player.piece(piece_id).borrow().position(), vec[i as usize - 51]);
            assert_eq!(
                player.board().borrow_mut().inside(vec[i as usize - 51]).pieces.len(),
                1
            );
            assert_eq!(
                player.board().borrow_mut().inside(vec[i as usize - 51]).player_id,
                Some(board::PlayerID::Player0)
            );
            assert_eq!(player.board().borrow_mut().inside(vec[i as usize - 52]).pieces.len(), 0);
        }
        player.enter_goal(piece_id, 56);
        assert_eq!(player.piece(piece_id).borrow().position(), 99);
    }
    
    #[test]
    fn move_all_pieces_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        for piece_id in 0..4 {
            player.free_piece(piece_id);
            for i in 0..50 {
                player.move_piece(piece_id, 1);
                assert_eq!(player.piece(piece_id).borrow().position(), i + 1);
                assert_eq!(
                    player.board().borrow_mut().outside(i + 1).pieces.len(),
                    1
                );
                assert_eq!(
                    player.board().borrow_mut().outside(i + 1).player_id,
                    Some(board::PlayerID::Player0)
                );
                assert_eq!(player.board().borrow_mut().outside(i).pieces.len(), 0);
            }
            player.enter_inside(piece_id, 50, 52);
            let vec = (52..=57).chain((52..57).rev()).collect::<Vec<_>>();
            for i in 52..62 {
                let oldpos = player.piece(piece_id).borrow().position();
                if (i + 1) % 57 == 0 {
                    continue;
                }

                player.update_piece(piece_id, oldpos, i + 1);
                assert_eq!(player.piece(piece_id).borrow().position(), vec[i as usize - 51]);

                assert_eq!(
                    player.board().borrow_mut().inside(vec[i as usize - 51]).player_id,
                    Some(board::PlayerID::Player0)
                );
            }
        }   
        assert_eq!(player.board().borrow_mut().inside(52).pieces.len(), 4);
    }
    
    #[test]
    fn move_all_pieces_test_2() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        for piece_id in 0..4 {
            player.free_piece(piece_id);
            for i in 0..50 {
                player.move_piece(piece_id, 1);
                assert_eq!(player.piece(piece_id).borrow().position(), i + 1);
                assert_eq!(
                    player.board().borrow_mut().outside(i + 1).pieces.len(),
                    1
                );
                assert_eq!(
                    player.board().borrow_mut().outside(i + 1).player_id,
                    Some(board::PlayerID::Player0)
                );
                assert_eq!(player.board().borrow_mut().outside(i).pieces.len(), 0);
            }
            player.enter_inside(piece_id, 50, 52);
            let vec = (52..=57).collect::<Vec<_>>();
            for i in 52..56 {
                let oldpos = player.piece(piece_id).borrow().position();

                player.update_piece(piece_id, oldpos, i + 1);
                assert_eq!(player.piece(piece_id).borrow().position(), vec[i as usize - 51]);
                assert_eq!(
                    player.board().borrow_mut().inside(vec[i as usize - 51]).player_id,
                    Some(board::PlayerID::Player0)
                );
            }
            player.enter_goal(piece_id, 56);
            assert_eq!(player.piece(piece_id).borrow().position(), 99);
        }
        assert!(player.is_finished());
    }

    #[test]
    fn safety_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let piece_id = 0;

        player.free_piece(piece_id);
        player.move_piece(piece_id, 7);
        player.save_piece(piece_id, 1);

        assert!(player.piece(piece_id).borrow().is_safe());
        assert!(player.piece(piece_id).borrow().is_dangerous());
        assert_eq!(player.piece(piece_id).borrow().position(), 8);

        player.update_piece(piece_id, 8, 50);
        assert!(!player.piece(piece_id).borrow().is_safe());

        player.save_piece(piece_id, 1);
        assert!(player.piece(piece_id).borrow().is_safe());
        assert!(!player.piece(piece_id).borrow().is_dangerous());
        assert_eq!(player.piece(piece_id).borrow().position(), 52);
    }

    #[test]
    fn safety_test_2() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        for piece_id in 0..4 {
            player.free_piece(piece_id);
            player.move_piece(piece_id, 7);
            player.save_piece(piece_id, 1);

            assert!(player.piece(piece_id).borrow().is_safe());
            assert!(player.piece(piece_id).borrow().is_dangerous());
            assert_eq!(player.piece(piece_id).borrow().position(), 8);

            player.update_piece(piece_id, 8, 50);
            assert!(!player.piece(piece_id).borrow().is_safe());

            player.save_piece(piece_id, 1);
            assert!(player.piece(piece_id).borrow().is_safe());
            assert!(!player.piece(piece_id).borrow().is_dangerous());
            assert_eq!(player.piece(piece_id).borrow().position(), 52);
        }
    }

    #[test]
    fn safety_test_3() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        for piece_id in 0..4 {
            player.free_piece(piece_id);
            for i in 1..=56 {
                if i == 50 {
                    continue;
                }
                let is_globe = player.board().borrow().is_globe(i);
                if is_globe {
                    player.save_piece(piece_id, 1);
                    assert!(player.piece(piece_id).borrow().is_safe());
                    assert!(player.piece(piece_id).borrow().is_dangerous());
                } else {
                    player.move_piece(piece_id, 1);
                }
            }
        }
    }

    #[test]
    fn starjump_to_goal_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        let piece_id = 0;
        player.free_piece(piece_id);
        player.move_piece(piece_id, 44);
        
        assert_eq!(player.piece(piece_id).borrow().position(), 44);

        player.win_piece(piece_id, 6);
        assert_eq!(player.piece(piece_id).borrow().position(), 99);
        assert!(player.piece(piece_id).borrow().is_goal());
        assert_eq!(player.board().borrow_mut().goal(PLAYER_ID).pieces.len(), 1);
    }

    #[test]
    fn starjump_to_goal_test_2() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        for piece_id in 0..4 {

            player.free_piece(piece_id);
            player.move_piece(piece_id, 44);
            
            assert_eq!(player.piece(piece_id).borrow().position(), 44);
            
            player.win_piece(piece_id, 6);
            assert_eq!(player.piece(piece_id).borrow().position(), 99);
            assert!(player.piece(piece_id).borrow().is_goal());
            assert_eq!(player.board().borrow_mut().goal(PLAYER_ID).pieces.len(), piece_id as usize + 1);
        }
        assert!(player.is_finished());
    }

    #[test]
    fn starjump_to_goal_test_3() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        for piece_id in 0..4 {
            player.free_piece(piece_id);
            for position in 1..=50 {
                if position < 50 {
                    player.move_piece(piece_id, 1);
                    assert_eq!(player.piece(piece_id).borrow().position(), position);
                } else {
                    player.win_piece(piece_id, 1);
                    assert_eq!(player.piece(piece_id).borrow().position(), 99);
                    assert!(player.piece(piece_id).borrow().is_goal());
                }
            }
            assert_eq!(player.board().borrow_mut().goal(PLAYER_ID).pieces.len(), piece_id as usize + 1);
        }
        assert!(player.is_finished());
    }

    #[test]
    fn enter_goal_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        let piece_id = 0;
        player.free_piece(piece_id);
        player.move_piece(piece_id, 50);
        player.win_piece(piece_id, 6);
        assert_eq!(player.piece(piece_id).borrow().position(), 99);
        assert!(player.piece(piece_id).borrow().is_goal());
        assert_eq!(player.board().borrow_mut().goal(PLAYER_ID).pieces.len(), 1);
    }

    #[test]
    fn enter_goal_test_2() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        for piece_id in 0..4 {
            player.free_piece(piece_id);
            player.move_piece(piece_id, 50);
            player.win_piece(piece_id, 6);
            assert_eq!(player.piece(piece_id).borrow().position(), 99);
            assert!(player.piece(piece_id).borrow().is_goal());
            assert_eq!(player.board().borrow_mut().goal(PLAYER_ID).pieces.len(), piece_id as usize + 1);
        }
        assert!(player.is_finished());
    }

    #[test]
    fn enter_goal_test_3() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board.clone());

        for dice_number in (1..=6).rev() {
            for piece_id in 0..4 {
                player.free_piece(piece_id);
                player.move_piece(piece_id, 50);
                player.move_piece(piece_id, 6 - dice_number);
                player.win_piece(piece_id, dice_number);
                assert_eq!(player.piece(piece_id).borrow().position(), 99);
                assert!(player.piece(piece_id).borrow().is_goal());
                assert_eq!(player.board().borrow_mut().goal(PLAYER_ID).pieces.len(), piece_id as usize + 1);
            }
            assert!(player.is_finished());
            board.borrow_mut().reset();
        }
    }


    // #[test]
    // fn starjump_test () {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let mut player = Player::new(PLAYER_ID, board.clone());

    //     let piece_id = 0;
    //     player.free_piece(piece_id);
    //     player.move_piece(piece_id, 44);
    //     player.win_piece(piece_id, 6);
    //     assert_eq!(player.piece(piece_id).borrow().position(), 99);
    //     assert!(player.piece(piece_id).borrow().is_goal());
    //     assert_eq!(player.board().borrow_mut().goal(PLAYER_ID).pieces.len(), 1);
    //     assert!(player.is_finished());
    //     board.borrow_mut().reset();
    // }
//     #[test]
//     fn update_piece_by_dice_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Dice::default();
//         let mut player = Player::new(PLAYER_ID, board);
//         let piece_id = 0;

//         player.take_dice(dice);
//         player.free_piece(0);
//         let result = player.roll_dice();
//         player.move_piece(0, result);
//         assert_eq!(player.piece(0).borrow_mut().position(), result);
//         assert_eq!(player.board().borrow_mut().outside(result).pieces.len(), 1);
//         assert_eq!(
//             player.board().borrow_mut().outside(result).player_id,
//             Some(board::PlayerID::Player0)
//         );
//         assert_eq!(player.board().borrow_mut().outside(0).pieces.len(), 0);
//     }

//     #[test]
//     fn move_by_dice_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Dice::default();
//         let mut player = Player::new(PLAYER_ID, board);
//         let piece_id = 0;

//         player.take_dice(dice);
//         let mut dice_number = player.roll_dice();
//         let mut choice = player.valid_choices(piece_id, dice_number, Act::Free);

//         while choice != Act::Free {
//             dice_number = player.roll_dice();
//             choice = player.valid_choices(piece_id, dice_number, Act::Free);
//         }

//         player.make_move(piece_id, dice_number, choice);
//         assert_eq!(player.piece(0).borrow_mut().position(), 0);
//         assert_eq!(player.board().borrow_mut().outside(0).pieces.len(), 1);
//         assert_eq!(
//             player.board().borrow_mut().outside[0].player_id,
//             Some(board::PlayerID::Player0)
//         );
//         player.die(piece_id);

//         for dice_number in 1..7 {
//             if dice_number == 5 {
//                 continue;
//             }
//             player.free_piece(piece_id);

//             let choice = player.valid_choices(piece_id, dice_number, Act::Move);
//             player.make_move(piece_id, dice_number, choice);
//             assert_eq!(player.piece(0).borrow_mut().position(), dice_number);
//             assert_eq!(
//                 player
//                     .board()
//                     .borrow_mut()
//                     .outside(dice_number)
//                     .pieces
//                     .len(),
//                 1
//             );
//             assert_eq!(
//                 player.board().borrow_mut().outside[dice_number as usize].player_id,
//                 Some(board::PlayerID::Player0)
//             );
//             assert_eq!(player.board().borrow_mut().outside(0).pieces.len(), 0);
//             player.die(piece_id);
//         }

//         #[test]
//     fn all_pieces_in_goal_test_1() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         player.free_piece(0);
//         player.free_piece(1);
//         player.free_piece(2);
//         player.free_piece(3);

//         player.move_piece(0, 50);
//         player.move_piece(1, 50);
//         player.move_piece(2, 50);
//         player.move_piece(3, 50);

//         let win_choice = player.valid_choices(0, 6, Act::Goal);

//         player.make_move(0, 6, win_choice);
//         player.make_move(1, 6, win_choice);
//         player.make_move(2, 6, win_choice);
//         player.make_move(3, 6, win_choice);

//         assert!(player.piece(0).borrow().is_goal());
//         assert!(player.piece(1).borrow_mut().is_goal());
//         assert!(player.piece(2).borrow_mut().is_goal());
//         assert!(player.piece(3).borrow_mut().is_goal());

//         assert!(player.is_finished());
//     }

//     #[test]
//     fn all_pieces_in_goal_test_part_2() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         player.free_piece(0);
//         player.free_piece(1);
//         player.free_piece(2);
//         player.free_piece(3);

//         player.move_piece(0, 49);
//         player.move_piece(1, 49);
//         player.move_piece(2, 49);
//         player.move_piece(3, 49);

//         player.move_piece(0, 4);
//         player.move_piece(1, 4);
//         player.move_piece(2, 4);
//         player.move_piece(3, 4);

//         let win_choice = player.valid_choices(0, 3, Act::Goal);

//         player.make_move(0, 3, win_choice);
//         player.make_move(1, 3, win_choice);
//         player.make_move(2, 3, win_choice);
//         player.make_move(3, 3, win_choice);

//         assert!(player.piece(0).borrow().is_goal());
//         assert!(player.piece(1).borrow_mut().is_goal());
//         assert!(player.piece(2).borrow_mut().is_goal());
//         assert!(player.piece(3).borrow_mut().is_goal());

//         assert!(player.is_finished());
//     }

//     #[test]
//     fn all_pieces_in_goal_test_part_3() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         player.free_piece(0);
//         player.free_piece(1);
//         player.free_piece(2);
//         player.free_piece(3);

//         player.move_piece(0, 49);
//         player.move_piece(1, 49);
//         player.move_piece(2, 49);
//         player.move_piece(3, 49);

//         let dice_number = 1;

//         let win_choice = player.valid_choices(0, dice_number, Act::Goal);

//         player.make_move(0, dice_number, win_choice);
//         player.make_move(1, dice_number, win_choice);
//         player.make_move(2, dice_number, win_choice);
//         player.make_move(3, dice_number, win_choice);

//         assert!(player.piece(0).borrow().is_goal());
//         assert!(player.piece(1).borrow_mut().is_goal());
//         assert!(player.piece(2).borrow_mut().is_goal());
//         assert!(player.piece(3).borrow_mut().is_goal());

//         assert!(player.is_finished());
//     }
    
//     }

//         #[test]
//     #[ignore = "long test"]
//     fn first_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board);
//         let actions = vec![
//             Act::Move,
//             Act::Free,
//             Act::Kill,
//             Act::Join,
//             Act::Leave,
//             Act::Die,
//             Act::Goal,
//             Act::Safe,
//             Act::Skip,
//             Act::Nothing,
//         ];
//         while !player.is_finished() {
//             player.my_turn();
//             player.random_play(actions.clone());
//             player.print_status();
//         }
//         assert!(player.is_finished());
    }