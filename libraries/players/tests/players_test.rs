use board::Board;
use dice::Dice;
use players::{Act, Player};
use std::{cell::RefCell, rc::Rc};

#[cfg(test)]
mod player_tests {

    use super::*;

    #[test]
    fn add_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(0, board.clone(), None);
        assert_eq!(player.id(), 0);
        assert_eq!(player.board().as_ptr(), board.as_ptr());
    }

    // #[test]
    // fn add_all_player_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let board2 = Rc::new(RefCell::new(Board::new()));
    //     let player0 = Player::new(0, board.clone(), None);
    //     let player1 = Player::new(1, board.clone(), None);
    //     let player2 = Player::new(2, board.clone(), None);
    //     let player3 = Player::new(3, board.clone(), None);

    //     assert_eq!(player0.id(), 0);
    //     assert_eq!(player1.id(), 1);
    //     assert_eq!(player2.id(), 2);
    //     assert_eq!(player3.id(), 3);

    //     assert_eq!(player0.board().as_ptr(), board.as_ptr());
    //     assert_eq!(player1.board().as_ptr(), player0.board().as_ptr());
    //     assert_eq!(player2.board().as_ptr(), player1.board().as_ptr());
    //     assert_eq!(player3.board().as_ptr(), player2.board().as_ptr());
    //     assert_eq!(board.as_ptr(), player3.board().as_ptr());
    //     assert_ne!(board2.as_ptr(), player0.board().as_ptr());
    // }

    // #[test]
    // #[should_panic]
    // fn invalid_player_id_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let mut player = Player::new(4, board, None);
    //     assert_eq!(player.id(), 4);
    //     player.free_piece(0);
    // }

    // #[test]
    // fn get_pieces_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let mut player = Player::new(0, board, None);

    //     let piece = player.piece(0);
    //     assert_eq!(piece.id(), 0);

    //     let piece = player.piece(1);
    //     assert_eq!(piece.id(), 1);

    //     let piece = player.piece(2);
    //     assert_eq!(piece.id(), 2);

    //     let piece = player.piece(3);
    //     assert_eq!(piece.id(), 3);
    // }

    // #[test]
    // fn get_piece_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let mut player = Player::new(0, board, None);
    //     (0..4).for_each(|i| {
    //         let piece = player.piece(i);
    //         assert_eq!(piece.id(), i);
    //         assert!(player.piece(i).is_home());
    //         assert!(player.piece(i).is_safe());
    //     });
    // }

    // #[test]
    // fn player_with_dice_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let mut player = Player::new(0, board, None);

    //     let result = player.roll_dice();
    //     assert!(result == 0);
    //     let dice = Rc::new(RefCell::new(Dice::new()));

    //     player.take_dice(dice);
    //     let result = player.roll_dice();
    //     assert!(result > 0 && result < 7);

    //     player.give_dice();
    //     let result = player.roll_dice();
    //     assert!(result == 0);
    // }

    // #[test]
    // fn player_turn_test() {
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let mut player = Player::new(0, board, Some(dice));
    //     assert!(!player.is_player_turn());
    //     player.my_turn();
    //     assert!(player.is_player_turn());

    //     let mut result = player.roll_dice();
    //     while result != 6 {
    //         result = player.roll_dice();
    //     }
    //     player.can_continue();
    //     assert!(player.is_player_turn());

    //     while result == 6 {
    //         result = player.roll_dice();
    //     }
    //     player.can_continue();
    //     assert!(!player.is_player_turn());
    // }
}

// mod move_single_piece_test {

//     use super::*;

//     #[test]
//     fn free_piece_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let mut player = Player::new(0, board, None);

//         assert!(player.piece(0).is_home());
//         assert_eq!(player.board().borrow().home[0].pieces, 4);

//         player.free_piece(0);
//         assert!(!player.piece(0).is_home());
//         assert!(player.piece(0).is_dangerous());
//         assert_eq!(player.piece(0).position(), 0);
//         assert_eq!(player.board().borrow().home[0].pieces, 3);
//         assert_eq!(player.board().borrow().outside[0].pieces, 1);
//     }

//     #[test]
//     fn update_piece_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let next_position = 4;
//         player.free_piece(0);
//         player.move_piece(0, next_position);
//         assert_eq!(player.piece(0).position(), next_position);
//         assert!(!player.piece(0).is_safe());
//     }

//     #[test]
//     fn update_piece_state_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let next_position_by_dice_number = 4;
//         player.free_piece(0);
//         player.move_piece(0, next_position_by_dice_number);

//         assert_eq!(player.board().borrow().outside[4].pieces, 1);
//         assert_eq!(player.board().borrow().outside[0].pieces, 0);

//         let next_position_by_dice_number = 2;
//         player.move_piece(0, next_position_by_dice_number);
//         assert_eq!(player.board().borrow().outside[6].pieces, 1);
//         assert_eq!(player.board().borrow().outside[4].pieces, 0);
//     }

//     #[test]
//     fn valid_move_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let piece_id = 0;
//         let piece_move = player.valid_moves(piece_id, 1);
//         assert!(!piece_move);

//         let piece_move = player.valid_moves(piece_id, 7);
//         assert!(!piece_move);

//         let piece_move = player.valid_moves(piece_id, 6);
//         assert!(piece_move);
//         player.free_piece(piece_id);

//         let piece_move = player.valid_moves(piece_id, 6);
//         assert!(piece_move);

//         let piece_id = 4;
//         let piece_move = player.valid_moves(piece_id, 1);
//         assert!(!piece_move);
//     }

//     #[test]
//     fn choice_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let piece_id: i8 = 0;
//         let dice_number: i8 = 6;
//         let action = Act::Free;

//         let selected_action = player.select_choice(piece_id, dice_number, action);

//         assert_eq!(selected_action, Act::Free);
//         assert_ne!(selected_action, Act::Nothing);

//         let piece_id: i8 = 0;
//         let dice_number: i8 = 6;
//         let action = Act::Move;

//         let selected_action = player.select_choice(piece_id, dice_number, action);

//         assert_eq!(selected_action, Act::Move);
//         assert_ne!(selected_action, Act::Free);
//         assert_ne!(selected_action, Act::Nothing);
//     }

//     #[test]
//     fn update_piece_by_dice_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         player.free_piece(0);
//         let mut result = player.roll_dice();
//         if result == 5 {
//             result = 1;
//         }
//         player.move_piece(0, result);
//         assert_eq!(player.piece(0).position(), result);
//         assert_eq!(
//             player.board().borrow().outside[result as usize].pieces,
//             1
//         );
//         assert_eq!(
//             player.board().borrow().outside[result as usize].player_id,
//             Some(board::PlayerID::Player0)
//         );
//         assert_eq!(player.board().borrow().outside[0].pieces, 0);
//     }

//     #[test]
//     fn move_by_dice_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         for i in 1..7 {
//             if i == 5 {
//                 continue;
//             }

//             let piece_id = 0;
//             let mut dice_roll = player.roll_dice();
//             let mut choice = player.select_choice(piece_id, dice_roll, Act::Free);

//             while choice != Act::Free {
//                 dice_roll = player.roll_dice();
//                 choice = player.select_choice(piece_id, dice_roll, Act::Free);
//             }

//             player.make_move(piece_id, dice_roll, choice);
//             assert_eq!(player.piece(0).position(), 0);
//             assert_eq!(player.board().borrow().outside[0].pieces, 1);
//             assert_eq!(
//                 player.board().borrow().outside[0].player_id,
//                 Some(board::PlayerID::Player0)
//             );

//             let choice = player.select_choice(piece_id, i, Act::Move);
//             player.make_move(piece_id, i, choice);
//             assert_eq!(player.piece(0).position(), i);
//             assert_eq!(
//                 player.board().borrow().outside[i as usize].pieces,
//                 1
//             );
//             assert_eq!(
//                 player.board().borrow().outside[i as usize].player_id,
//                 Some(board::PlayerID::Player0)
//             );
//             assert_eq!(player.board().borrow().outside[0].pieces, 0);
//             player.die(piece_id);
//         }
//     }

//     #[test]
//     fn enter_inside_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let piece_id = 0;
//         let mut dice_roll = player.roll_dice();
//         let action = Act::Free;
//         let mut valid_choice = player.select_choice(piece_id, dice_roll, action);

//         while valid_choice != Act::Free {
//             dice_roll = player.roll_dice();
//             valid_choice = player.select_choice(piece_id, dice_roll, Act::Free);
//         }
//         player.make_move(piece_id, dice_roll, valid_choice);

//         dice_roll = 5;

//         player.move_piece(piece_id, 50);
//         valid_choice = player.select_choice(piece_id, dice_roll, Act::Move);
//         player.make_move(piece_id, dice_roll, valid_choice);
//         assert_eq!(player.piece(piece_id).position(), 51 + dice_roll);
//         assert_eq!(
//             player
//                 .board()
//                 .borrow()
//                 .inside((51 + dice_roll) as usize)
//                 .unwrap()
//                 .pieces,
//             1
//         );
//         assert_eq!(
//             player.board().borrow().inside[(dice_roll as usize) - 1].pieces,
//             1
//         );
//     }

//     #[test]
//     fn enter_goal_from_outside_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let piece_id = 0;
//         player.free_piece(piece_id);
//         player.move_piece(piece_id, 44);
//         player.move_piece(piece_id, 6);

//         assert_eq!(player.piece(piece_id).position(), 99);
//         assert!(player.piece(piece_id).is_goal());
//         assert_eq!(player.board().borrow().goal[0].pieces, 1);
//         assert_eq!(
//             player.board().borrow().goal[0].player_id,
//             Some(board::PlayerID::Player0)
//         );
//     }

//     #[test]
//     fn enter_goal_from_inside_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let piece_id = 0;
//         player.free_piece(piece_id);
//         player.move_piece(piece_id, 49);
//         player.move_piece(piece_id, 4);

//         assert_eq!(player.piece(piece_id).position(), 54);
//         assert!(!player.piece(piece_id).is_goal());
//         assert_eq!(
//             player.board().borrow().inside(54).unwrap().pieces,
//             1
//         );
//         assert_eq!(
//             player.board().borrow().inside[2].player_id,
//             Some(board::PlayerID::Player0)
//         );

//         player.move_piece(piece_id, 3);

//         assert_eq!(player.piece(piece_id).position(), 99);
//         assert!(player.piece(piece_id).is_goal());
//         assert_eq!(player.board().borrow().goal[0].pieces, 1);
//         assert_eq!(
//             player.board().borrow().goal[0].player_id,
//             Some(board::PlayerID::Player0)
//         );
//     }

//     #[test]
//     fn move_back_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let piece_id = 0;
//         player.free_piece(piece_id);
//         player.move_piece(piece_id, 50);
//         player.move_piece(piece_id, 4);

//         assert_eq!(player.piece(piece_id).position(), 55);
//         assert_eq!(player.board().borrow().inside[3].pieces, 1);

//         player.move_piece(piece_id, 6);
//         assert_eq!(player.piece(piece_id).position(), 53);
//         assert_eq!(player.board().borrow().inside[1].pieces, 1);
//     }

//     #[test]
//     fn death_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let piece_id = 0;
//         player.free_piece(piece_id);
//         player.move_piece(piece_id, 50);

//         player.die(piece_id);
//         assert_eq!(player.piece(piece_id).position(), -1);
//         assert_eq!(player.board().borrow().outside[50].pieces, 0);
//         assert_eq!(player.board().borrow().outside[0].pieces, 0);
//         assert_eq!(player.board().borrow().home[0].pieces, 4);
//     }

//     #[test]
//     fn in_globe_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let piece_id = 0;
//         player.free_piece(piece_id);
//         player.make_move(piece_id, 2, Act::Move);
//         assert_eq!(player.piece(piece_id).position(), 2);
//         assert!(!player.piece(piece_id).is_safe());
//         assert!(!player.piece(piece_id).is_dangerous());

//         player.make_move(piece_id, 6, Act::Move);
//         assert_eq!(player.piece(piece_id).position(), 8);
//         assert!(player.piece(piece_id).is_safe());
//         assert!(player.piece(piece_id).is_dangerous());
//     }

//     #[test]
//     fn star_jump_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let piece_id = 0;
//         player.free_piece(piece_id);
//         player.make_move(piece_id, 5, Act::Move);
//         assert_eq!(player.piece(piece_id).position(), 11);
//         assert!(!player.piece(piece_id).is_safe());
//         assert!(!player.piece(piece_id).is_dangerous());

//         player.move_piece(piece_id, 1);
//         player.move_piece(piece_id, 6);
//         assert_eq!(player.piece(piece_id).position(), 24);
//         assert!(!player.piece(piece_id).is_safe());
//         assert!(!player.piece(piece_id).is_dangerous());
//     }

//     #[test]
//     fn starjump_to_goal_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let piece_id = 0;
//         player.free_piece(piece_id);
//         player.move_piece(piece_id, 44);
//         player.move_piece(piece_id, 6);
//         assert_eq!(player.piece(piece_id).position(), 99);
//         assert!(player.piece(piece_id).is_goal());
//         assert_eq!(player.board().borrow().goal[0].pieces, 1);
//         assert_eq!(
//             player.board().borrow().goal[0].player_id,
//             Some(board::PlayerID::Player0)
//         );
//     }
// }

// mod multipiece_test {
//     use super::*;

//     #[test]
//     fn free_all_pieces_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         for piece_id in 0..4 {
//             player.free_piece(piece_id);
//             assert!(!player.piece(piece_id).is_home());
//             assert!(player.piece(piece_id).is_dangerous());
//             assert!(player.piece(piece_id).is_safe());
//             assert_eq!(player.piece(piece_id).position(), 0);
//         }
//         assert_eq!(player.board().borrow().home[0].pieces, 0);
//         assert_eq!(player.board().borrow().outside[0].pieces, 4);
//     }

//     #[test]
//     fn joining_other_pieces_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));
//         player.free_piece(0);
//         player.free_piece(1);

//         player.make_move(0, 6, Act::Move);
//         player.make_move(1, 6, Act::Join);

//         assert_eq!(player.piece(0).position(), 6);
//         assert_eq!(player.piece(1).position(), 6);
//         assert_eq!(player.board().borrow().outside[6].pieces, 2);
//         assert!(player.piece(0).is_dangerous());
//         assert!(player.piece(1).is_dangerous());
//         assert!(player.piece(0).is_safe());
//         assert!(player.piece(1).is_safe());
//     }

//     #[test]
//     fn leaving_other_pieces_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));
//         player.free_piece(0);
//         player.free_piece(1);

//         player.make_move(0, 6, Act::Move);
//         player.make_move(1, 6, Act::Join);
//         assert_eq!(player.piece(0).position(), 6);
//         assert_eq!(player.piece(1).position(), 6);
//         assert_eq!(player.board().borrow().outside[6].pieces, 2);
//         assert!(player.piece(0).is_dangerous());
//         assert!(player.piece(1).is_dangerous());

//         player.make_move(0, 6, Act::Leave);
//         assert_eq!(player.piece(0).position(), 12);
//         assert_eq!(player.piece(1).position(), 6);
//         assert_eq!(player.board().borrow().outside[6].pieces, 1);
//         assert_eq!(player.board().borrow().outside[12].pieces, 1);
//         assert!(!player.piece(0).is_dangerous());
//         assert!(!player.piece(1).is_dangerous());
//     }

//     #[test]
//     fn all_pieces_at_same_place_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         player.free_piece(0);
//         player.free_piece(1);
//         player.free_piece(2);
//         player.free_piece(3);

//         player.make_move(0, 6, Act::Move);
//         player.make_move(1, 6, Act::Join);
//         player.make_move(2, 6, Act::Join);
//         player.make_move(3, 6, Act::Join);

//         assert_eq!(player.piece(0).position(), 6);
//         assert_eq!(player.piece(1).position(), 6);
//         assert_eq!(player.piece(2).position(), 6);
//         assert_eq!(player.piece(3).position(), 6);

//         assert!(player.piece(0).is_safe());
//         assert!(player.piece(1).is_safe());
//         assert!(player.piece(2).is_safe());
//         assert!(player.piece(3).is_safe());

//         assert!(player.piece(0).is_dangerous());
//         assert!(player.piece(1).is_dangerous());
//         assert!(player.piece(2).is_dangerous());
//         assert!(player.piece(3).is_dangerous());

//         player.make_move(0, 1, Act::Leave);
//         assert_eq!(player.piece(0).position(), 7);
//         assert!(!player.piece(0).is_safe());
//         assert!(!player.piece(0).is_dangerous());

//         assert!(player.piece(1).is_safe());
//         assert!(player.piece(2).is_safe());
//         assert!(player.piece(3).is_safe());
//         assert!(player.piece(1).is_dangerous());
//         assert!(player.piece(2).is_dangerous());
//         assert!(player.piece(3).is_dangerous());

//         player.make_move(1, 3, Act::Leave);
//         assert_eq!(player.piece(1).position(), 9);
//         assert!(!player.piece(1).is_safe());
//         assert!(!player.piece(1).is_dangerous());

//         assert!(player.piece(2).is_safe());
//         assert!(player.piece(3).is_safe());
//         assert!(player.piece(2).is_dangerous());
//         assert!(player.piece(3).is_dangerous());

//         player.make_move(2, 4, Act::Leave);
//         assert_eq!(player.piece(2).position(), 10);
//         assert!(!player.piece(2).is_safe());
//         assert!(!player.piece(2).is_dangerous());
//         assert!(!player.piece(3).is_safe());
//         assert!(!player.piece(3).is_dangerous());
//     }

//     #[test]
//     fn joining_choice_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         player.free_piece(0);
//         player.free_piece(1);

//         player.make_move(0, 6, Act::Move);
//         let joining_choice = player.select_choice(1, 6, Act::Join);
//         assert_eq!(joining_choice, Act::Join);
//     }

//     #[test]
//     fn all_pieces_in_goal_test_0() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         player.free_piece(0);
//         player.free_piece(1);
//         player.free_piece(2);
//         player.free_piece(3);

//         player.move_piece(0, 99);
//         player.move_piece(1, 99);
//         player.move_piece(2, 99);
//         player.move_piece(3, 99);

//         assert!(player.piece(0).is_goal());
//         assert!(player.piece(1).is_goal());
//         assert!(player.piece(2).is_goal());
//         assert!(player.piece(3).is_goal());

//         assert!(player.is_finished());
//     }

//     #[test]
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

//         player.move_piece(0, 6);
//         player.move_piece(1, 6);
//         player.move_piece(2, 6);
//         player.move_piece(3, 6);

//         assert!(player.piece(0).is_goal());
//         assert!(player.piece(1).is_goal());
//         assert!(player.piece(2).is_goal());
//         assert!(player.piece(3).is_goal());

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

//         player.move_piece(0, 3);
//         player.move_piece(1, 3);
//         player.move_piece(2, 3);
//         player.move_piece(3, 3);

//         assert!(player.piece(0).is_goal());
//         assert!(player.piece(1).is_goal());
//         assert!(player.piece(2).is_goal());
//         assert!(player.piece(3).is_goal());

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

//         player.move_piece(0, 1);
//         player.move_piece(1, 1);
//         player.move_piece(2, 1);
//         player.move_piece(3, 1);

//         assert!(player.piece(0).is_goal());
//         assert!(player.piece(1).is_goal());
//         assert!(player.piece(2).is_goal());
//         assert!(player.piece(3).is_goal());

//         assert!(player.is_finished());
//     }

//     #[test]
//     #[ignore = "Action selection not implemented"]
//     fn single_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         while !player.is_finished() {
//             player.my_turn();
//             player.random_play();
//             println!(
//                 "Piece 0: {:?}\nPiece 1: {:?}\nPiece 2: {:?}\nPiece 3: {:?}\n\n",
//                 player.piece(0).position(),
//                 player.piece(1).position(),
//                 player.piece(2).position(),
//                 player.piece(3).position()
//             );
//         }
//         assert!(player.is_finished());
//     }
// }

// mod multiplayer_test {
//     use super::*;

//     #[test]
//     fn two_players_free_test() {
//         let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
//         let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
//         let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut player2 = Player::new(1, board, Some(dice));

//         player1.free_piece(0);
//         player2.free_piece(0);

//         assert_eq!(player1.piece(0).position(), 0);
//         assert_eq!(player2.piece(0).position(), 13);
//         assert_eq!(player1.board().borrow().outside[0].pieces, 1);
//         assert_eq!(
//             player1.board().borrow().outside[0].player_id,
//             Some(board::PlayerID::Player0)
//         );
//         assert_eq!(player2.board().borrow().outside[13].pieces, 1);
//         assert_eq!(
//             player1.board().borrow().outside[13].player_id,
//             Some(board::PlayerID::Player1)
//         );
//     }

//     #[test]
//     fn all_players_free_test() {
//         let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
//         let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
//         let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut player2 = Player::new(1, board.clone(), Some(dice.clone()));
//         let mut player3 = Player::new(2, board.clone(), Some(dice.clone()));
//         let mut player4 = Player::new(3, board, Some(dice));

//         player1.free_piece(0);
//         player2.free_piece(0);
//         player3.free_piece(0);
//         player4.free_piece(0);

//         assert_eq!(player1.piece(0).position(), 0);
//         assert_eq!(player2.piece(0).position(), 13);
//         assert_eq!(player3.piece(0).position(), 26);
//         assert_eq!(player4.piece(0).position(), 39);
        
//         assert_eq!(player1.board().borrow().outside[0].pieces, 1);
//         assert_eq!(
//             player1.board().borrow().outside[0].player_id,
//             Some(board::PlayerID::Player0)
//         );
//         assert_eq!(player1.board().borrow().outside[13].pieces, 1);
//         assert_eq!(
//             player1.board().borrow().outside[13].player_id,
//             Some(board::PlayerID::Player1)
//         );
//         assert_eq!(player1.board().borrow().outside[26].pieces, 1);
//         assert_eq!(
//             player1.board().borrow().outside[26].player_id,
//             Some(board::PlayerID::Player2)
//         );
//         assert_eq!(player1.board().borrow().outside[39].pieces, 1);
//         assert_eq!(
//             player1.board().borrow().outside[39].player_id,
//             Some(board::PlayerID::Player3)
//         );
//     }

//     #[test]
//     fn two_players_move_test() {
//         let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
//         let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
//         let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut player2 = Player::new(1, board, Some(dice));

//         player1.free_piece(0);
//         player2.free_piece(0);

//         player1.move_piece(0, 6);
//         player2.move_piece(0, 6);

//         assert_eq!(player1.piece(0).position(), 6);
//         assert_eq!(player2.piece(0).position(), 19);

//         assert_eq!(player1.board().borrow().outside[0].pieces, 0);
//         assert_eq!(player1.board().borrow().outside[6].pieces, 1);
//         assert_eq!(
//             player1.board().borrow().outside[6].player_id,
//             Some(board::PlayerID::Player0)
//         );
//         assert_eq!(player2.board().borrow().outside[13].pieces, 0);
//         assert_eq!(player2.board().borrow().outside[19].pieces, 1);
//         assert_eq!(
//             player2.board().borrow().outside[19].player_id,
//             Some(board::PlayerID::Player1)
//         );
//     }

//     #[test]
//     fn all_players_move_test() {
//         let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
//         let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
//         let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut player2 = Player::new(1, board.clone(), Some(dice.clone()));
//         let mut player3 = Player::new(2, board.clone(), Some(dice.clone()));
//         let mut player4 = Player::new(3, board, Some(dice));

//         player1.free_piece(0);
//         player2.free_piece(0);
//         player3.free_piece(0);
//         player4.free_piece(0);

//         player1.move_piece(0, 6);
//         player2.move_piece(0, 6);
//         player3.move_piece(0, 6);
//         player4.move_piece(0, 6);

//         assert_eq!(player1.piece(0).position(), 6);
//         assert_eq!(player2.piece(0).position(), 19);
//         assert_eq!(player3.piece(0).position(), 32);
//         assert_eq!(player4.piece(0).position(), 45);

//         assert_eq!(player1.board().borrow().outside[0].pieces, 0);
//         assert_eq!(player1.board().borrow().outside[6].pieces, 1);
//         assert_eq!(
//             player1.board().borrow().outside[6].player_id,
//             Some(board::PlayerID::Player0)
//         );
//         assert_eq!(player2.board().borrow().outside[13].pieces, 0);
//         assert_eq!(player2.board().borrow().outside[19].pieces, 1);
//         assert_eq!(
//             player2.board().borrow().outside[19].player_id,
//             Some(board::PlayerID::Player1)
//         );
//         assert_eq!(player3.board().borrow().outside[26].pieces, 0);
//         assert_eq!(player3.board().borrow().outside[32].pieces, 1);
//         assert_eq!(
//             player3.board().borrow().outside[32].player_id,
//             Some(board::PlayerID::Player2)
//         );
//         assert_eq!(player4.board().borrow().outside[39].pieces, 0);
//         assert_eq!(player4.board().borrow().outside[45].pieces, 1);
//         assert_eq!(
//             player4.board().borrow().outside[45].player_id,
//             Some(board::PlayerID::Player3)
//         );
//     }

//     #[test]
//     fn other_player_circumvent_player_1() {
//         let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
//         let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));

//         let mut player2 = Player::new(1, board.clone(), Some(dice.clone()));
//         player2.free_piece(0);
//         player2.piece(0).set_position(50);
//         player2.move_piece(0, 1);
//         assert_eq!(player2.piece(0).position(), 51);

//         player2.piece(0).set_position(50);
//         player2.move_piece(0, 6);
//         assert_eq!(player2.piece(0).position(), 4);

//         let mut player3 = Player::new(2, board.clone(), Some(dice.clone()));
//         player3.free_piece(0);
//         player3.piece(0).set_position(50);
//         player3.move_piece(0, 1);
//         assert_eq!(player3.piece(0).position(), 51);

//         player3.piece(0).set_position(50);
//         player3.move_piece(0, 6);
//         assert_eq!(player3.piece(0).position(), 4);

//         let mut player4 = Player::new(3, board, Some(dice));
//         player4.free_piece(0);
//         player4.piece(0).set_position(50);
//         player4.move_piece(0, 1);
//         assert_eq!(player4.piece(0).position(), 51);
//         player4.piece(0).set_position(50);
//         player4.move_piece(0, 6);
//         assert_eq!(player4.piece(0).position(), 4);
//     }

//     #[test]
//     #[ignore = "not suceeded"]
//     fn two_player_kill_test() {
//         let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
//         let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
//         let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut player2 = Player::new(1, board, Some(dice));

//         player1.free_piece(0);
//         player2.free_piece(0);

//         player1.piece(0).set_position(6);
//         player2.piece(0).set_position(5);

//         let diceroll1 = 3;
//         let diceroll2 = 4;

//         let choice1 = player1.select_choice(0, diceroll1, Act::Move);
//         let choice2 = player1.select_choice(0, diceroll2, Act::Kill);

//         player1.make_move(0, diceroll1, choice1);
//         player2.make_move(0, diceroll2, choice2);

//         assert_eq!(player1.piece(0).position(), -1);
//         assert!(player1.piece(0).is_home());

//         assert_eq!(player2.piece(0).position(), 9);
//     }
// }
