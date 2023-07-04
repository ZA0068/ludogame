// use board::Board;
// use dice::Dice;
// use players::{Act, Player};
// use std::{cell::RefCell, rc::Rc};

// mod move_single_piece_test {

//     use super::*;

//     #[test]
//     fn free_piece_test() {
//         let player_id = 0;
//         let piece_id = 0;
//         let new_position = 0;

//         let board = Rc::new(RefCell::new(Board::new()));
//         let mut player = Player::new(player_id, board, None);
//         let piece = player.piece(piece_id);

//         assert!(piece.borrow_mut().is_home());
//         assert!(player
//             .board()
//             .borrow_mut()
//             .home(player_id)
//             .piece(0)
//             .borrow_mut()
//             .is_home());
//         assert_eq!(player.board().borrow_mut().home(player_id).pieces.len(), 4);
//         player.free_piece(piece_id);

//         assert!(!piece.borrow_mut().is_home());
//         assert!(piece.borrow_mut().is_dangerous());
//         assert_eq!(piece.borrow_mut().position(), 0);

//         assert_eq!(player.board().borrow_mut().home(player_id).pieces.len(), 3);
//         assert_eq!(
//             player
//                 .board()
//                 .borrow_mut()
//                 .outside(new_position)
//                 .pieces
//                 .len(),
//             1
//         );
//         assert!(!player
//             .board()
//             .borrow_mut()
//             .outside(new_position)
//             .piece(piece_id)
//             .borrow_mut()
//             .is_home());
//         assert!(player
//             .board()
//             .borrow_mut()
//             .outside(new_position)
//             .piece(piece_id)
//             .borrow_mut()
//             .is_dangerous());
//     }

//     #[test]
//     fn update_piece_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));
//         let piece_id = 0;
//         let next_position = 4;

//         player.free_piece(0);
//         player.move_piece(0, next_position);

//         let piece = player.piece(piece_id);
//         assert_eq!(piece.borrow_mut().position(), next_position);
//         assert!(!piece.borrow_mut().is_safe());
//         assert!(!player
//             .board()
//             .borrow_mut()
//             .outside(next_position)
//             .piece(piece_id)
//             .borrow_mut()
//             .is_safe());
//         assert_eq!(
//             player
//                 .board()
//                 .borrow_mut()
//                 .outside(next_position)
//                 .piece(piece_id)
//                 .borrow_mut()
//                 .position(),
//             next_position
//         );
//     }

//     #[test]
//     fn update_piece_state_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let piece_id = 0;
//         let mut next_position: i8 = 4;

//         player.free_piece(piece_id);
//         player.move_piece(piece_id, next_position);

//         let piece = player.piece(piece_id);
//         assert_eq!(piece.borrow_mut().position(), next_position);
//         assert!(!piece.borrow_mut().is_safe());
//         assert!(!player
//             .board()
//             .borrow_mut()
//             .outside(next_position)
//             .piece(piece_id)
//             .borrow_mut()
//             .is_safe());
//         assert_eq!(
//             player
//                 .board()
//                 .borrow_mut()
//                 .outside(next_position)
//                 .piece(piece_id)
//                 .borrow_mut()
//                 .position(),
//             next_position
//         );
//         assert_eq!(
//             player
//                 .board()
//                 .borrow_mut()
//                 .outside(next_position)
//                 .pieces
//                 .len(),
//             1
//         );
//         assert_eq!(player.board().borrow_mut().outside(0).pieces.len(), 0);

//         next_position = 2;
//         player.move_piece(0, next_position);
//         assert_eq!(player.board().borrow_mut().outside(6).pieces.len(), 1);
//         assert_eq!(player.board().borrow_mut().outside(4).pieces.len(), 0);
//     }

//     #[test]
//     fn valid_move_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let piece_id = 0;
//         let piece_move = player.valid_moves(piece_id, 1).1;
//         assert!(!piece_move);

//         let piece_move = player.valid_moves(piece_id, 7).1;
//         assert!(!piece_move);

//         let piece_move = player.valid_moves(piece_id, 6).1;
//         assert!(piece_move);
//         player.free_piece(piece_id);

//         let piece_move = player.valid_moves(piece_id, 6).1;
//         assert!(piece_move);

//         let piece_id = 4;
//         let piece_move = player.valid_moves(piece_id, 1).1;
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

//         let selected_action = player.valid_choices(piece_id, dice_number, action);

//         assert_eq!(selected_action, Act::Free);
//         assert_ne!(selected_action, Act::Nothing);
//         player.free_piece(piece_id);

//         let piece_id: i8 = 0;
//         let dice_number: i8 = 6;
//         let action = Act::Move;

//         let selected_action = player.valid_choices(piece_id, dice_number, action);

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
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         for i in 1..7 {
//             if i == 5 {
//                 continue;
//             }

//             let piece_id = 0;
//             let mut dice_roll = player.roll_dice();
//             let mut choice = player.valid_choices(piece_id, dice_roll, Act::Free);

//             while choice != Act::Free {
//                 dice_roll = player.roll_dice();
//                 choice = player.valid_choices(piece_id, dice_roll, Act::Free);
//             }

//             player.make_move(piece_id, dice_roll, choice);
//             assert_eq!(player.piece(0).borrow_mut().position(), 0);
//             assert_eq!(player.board().borrow_mut().outside(0).pieces.len(), 1);
//             assert_eq!(
//                 player.board().borrow_mut().outside[0].player_id,
//                 Some(board::PlayerID::Player0)
//             );

//             let choice = player.valid_choices(piece_id, i, Act::Move);
//             player.make_move(piece_id, i, choice);
//             assert_eq!(player.piece(0).borrow_mut().position(), i);
//             assert_eq!(player.board().borrow_mut().outside(i).pieces.len(), 1);
//             assert_eq!(
//                 player.board().borrow_mut().outside[i as usize].player_id,
//                 Some(board::PlayerID::Player0)
//             );
//             assert_eq!(player.board().borrow_mut().outside(0).pieces.len(), 0);
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
//         let mut valid_choices = player.valid_choices(piece_id, dice_roll, action);

//         while valid_choices != Act::Free {
//             dice_roll = player.roll_dice();
//             valid_choices = player.valid_choices(piece_id, dice_roll, Act::Free);
//         }
//         player.make_move(piece_id, dice_roll, valid_choices);

//         dice_roll = 5;

//         player.move_piece(piece_id, 50);
//         valid_choices = player.valid_choices(piece_id, dice_roll, Act::Move);
//         player.make_move(piece_id, dice_roll, valid_choices);
//         assert_eq!(
//             player.piece(piece_id).borrow_mut().position(),
//             51 + dice_roll
//         );
//         assert_eq!(
//             player
//                 .board()
//                 .borrow_mut()
//                 .inside(51 + dice_roll)
//                 .pieces
//                 .len(),
//             1
//         );
//         assert_eq!(
//             player.board().borrow_mut().inside(51 + dice_roll).player_id,
//             Some(board::PlayerID::Player0)
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
//         let choice = player.valid_choices(piece_id, 6, Act::Goal);
//         player.make_move(piece_id, 6, choice);

//         assert_eq!(player.piece(piece_id).borrow().position(), 99);
//         assert!(player.piece(piece_id).borrow().is_goal());
//         assert_eq!(player.board().borrow_mut().goal(0).pieces.len(), 1);
//         assert_eq!(
//             player.board().borrow().goal[0].player_id,
//             Some(board::PlayerID::Player0)
//         );

//         player.free_piece(1);

//         player.move_piece(1, 49);

//         let _ = player.try_enter_goal(1, 49, 50);

//         assert_eq!(player.piece(1).borrow().position(), 99);
//         assert!(player.piece(1).borrow().is_goal());
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

//         assert_eq!(player.piece(piece_id).borrow().position(), 54);
//         assert!(!player.piece(piece_id).borrow().is_goal());
//         assert_eq!(player.board().borrow_mut().inside(54).pieces.len(), 1);
//         assert_eq!(
//             player.board().borrow().inside[2].player_id,
//             Some(board::PlayerID::Player0)
//         );

//         player.goal(piece_id);

//         assert_eq!(player.piece(piece_id).borrow().position(), 99);
//         assert!(player.piece(piece_id).borrow().is_goal());
//         assert_eq!(player.board().borrow_mut().goal(0).pieces.len(), 1);
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

//         assert_eq!(player.piece(piece_id).borrow_mut().position(), 55);
//         assert_eq!(player.board().borrow_mut().inside(55).pieces.len(), 1);

//         player.move_piece(piece_id, 6);
//         assert_eq!(player.piece(piece_id).borrow_mut().position(), 53);
//         assert_eq!(player.board().borrow_mut().inside(53).pieces.len(), 1);
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
//         assert_eq!(player.piece(piece_id).borrow_mut().position(), -1);
//         assert_eq!(player.board().borrow_mut().outside(50).pieces.len(), 0);
//         assert_eq!(player.board().borrow_mut().outside(0).pieces.len(), 0);
//         assert_eq!(player.board().borrow_mut().home(0).pieces.len(), 4);
//     }

//     #[test]
//     fn in_globe_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let piece_id = 0;
//         player.free_piece(piece_id);
//         player.make_move(piece_id, 2, Act::Move);
//         assert_eq!(player.piece(piece_id).borrow().position(), 2);
//         assert!(!player.piece(piece_id).borrow().is_safe());
//         assert!(!player.piece(piece_id).borrow().is_dangerous());

//         player.make_move(piece_id, 6, Act::Safe);
//         assert_eq!(player.piece(piece_id).borrow().position(), 8);
//         assert!(player.piece(piece_id).borrow().is_safe());
//         assert!(player.piece(piece_id).borrow().is_dangerous());
//     }

//     #[test]
//     fn star_jump_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let piece_id = 0;
//         player.free_piece(piece_id);
//         player.make_move(piece_id, 5, Act::Skip);
//         assert_eq!(player.piece(piece_id).borrow().position(), 11);
//         assert!(!player.piece(piece_id).borrow().is_safe());
//         assert!(!player.piece(piece_id).borrow().is_dangerous());

//         player.move_piece(piece_id, 1);
//         player.skip(piece_id, 6);
//         assert_eq!(player.piece(piece_id).borrow().position(), 24);
//         assert!(!player.piece(piece_id).borrow().is_safe());
//         assert!(!player.piece(piece_id).borrow().is_dangerous());
//     }

//     #[test]
//     fn starjump_to_goal_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));

//         let piece_id = 0;
//         player.free_piece(piece_id);
//         player.move_piece(piece_id, 44);
//         let choice = player.valid_choices(piece_id, 6, Act::Goal);
//         player.make_move(piece_id, 6, choice);
//         assert_eq!(player.piece(piece_id).borrow().position(), 99);
//         assert!(player.piece(piece_id).borrow().is_goal());
//         assert_eq!(player.board().borrow_mut().goal(0).pieces.len(), 1);
//         assert_eq!(
//             player.board().borrow().goal[0].player_id,
//             Some(board::PlayerID::Player0)
//         );
//     }
// }
