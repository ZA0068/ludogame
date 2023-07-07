// use board::Board;
// use pieces::Color;
// use players::Player;
// use std::{cell::RefCell, rc::Rc};

// #[cfg(test)]
// mod player_0_tests {

//     use super::*;
//     static PLAYER_ID: i8 = 0;

//     #[test]
//     fn add_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let player = Player::new(PLAYER_ID, board.clone(), None);
//         assert_eq!(player.id(), 0);
//         assert_eq!(player.board().as_ptr(), board.as_ptr());
//     }

//     #[test]
//     fn get_pieces_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let player = Player::new(PLAYER_ID, board, None);
//         (0..4).for_each(|i| {
//             let piece = player.piece(i);
//             assert_eq!(piece.borrow().id(), i);
//             assert_eq!(piece.borrow().color(), Color::Green);
//             assert_ne!(piece.borrow().color(), Color::Yellow);
//             assert_ne!(piece.borrow().color(), Color::Blue);
//             assert_ne!(piece.borrow().color(), Color::Red);
//         });
//     }

//     #[test]
//     #[should_panic]
//     fn get_pieces_test_2() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let player = Player::new(PLAYER_ID, board, None);
//         let piece = player.piece(0);
//         assert_eq!(piece.borrow().id(), 0);
//         assert_eq!(piece.borrow().color(), Color::Yellow);
//     }

//     #[test]
//     #[should_panic]
//     fn get_pieces_test_3() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let player = Player::new(PLAYER_ID, board, None);
//         let piece = player.piece(0);
//         assert_eq!(piece.borrow().id(), 0);
//         assert_eq!(piece.borrow().color(), Color::Red);
//     }

//     #[test]
//     #[should_panic]
//     fn get_pieces_test_4() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let player = Player::new(PLAYER_ID, board, None);
//         let piece = player.piece(0);
//         assert_eq!(piece.borrow().id(), 0);
//         assert_eq!(piece.borrow().color(), Color::Blue);
//     }

//     #[test]
//     #[should_panic]
//     fn get_pieces_test_5() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let player = Player::new(PLAYER_ID, board, None);
//         let piece = player.piece(0);
//         assert_eq!(piece.borrow().id(), 0);
//         assert_ne!(piece.borrow().color(), Color::Green);
//     }
// }

    // #[test]
    // fn starjump_to_goal_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));

    //     let piece_id = 0;
    //     player.free_piece(piece_id);
    //     player.move_piece(piece_id, 44);
    //     let choice = player.valid_choices(piece_id, 6, Act::Goal);
    //     player.make_move(piece_id, 6, choice);
    //     assert_eq!(player.piece(piece_id).borrow().position(), 99);
    //     assert!(player.piece(piece_id).borrow().is_goal());
    //     assert_eq!(player.board().borrow_mut().goal(0).pieces.len(), 1);
    //     assert_eq!(
    //         player.board().borrow().goal[0].player_id,
    //         Some(board::PlayerID::Player0)
    //     );
    // }

    // #[test]
    // fn update_piece_by_dice_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Dice::default();
    //     let mut player = Player::new(PLAYER_ID, board);
    //     let piece_id = 0;

    //     player.take_dice(dice);
    //     player.free_piece(0);
    //     let result = player.roll_dice();
    //     player.move_piece(0, result);
    //     assert_eq!(player.piece(0).borrow_mut().position(), result);
    //     assert_eq!(player.board().borrow_mut().outside(result).pieces.len(), 1);
    //     assert_eq!(
    //         player.board().borrow_mut().outside(result).player_id,
    //         Some(board::PlayerID::Player0)
    //     );
    //     assert_eq!(player.board().borrow_mut().outside(0).pieces.len(), 0);
    // }

    // #[test]
    // fn move_by_dice_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Dice::default();
    //     let mut player = Player::new(PLAYER_ID, board);
    //     let piece_id = 0;

    //     player.take_dice(dice);
    //     let mut dice_number = player.roll_dice();
    //     let mut choice = player.valid_choices(piece_id, dice_number, Act::Free);

    //     while choice != Act::Free {
    //         dice_number = player.roll_dice();
    //         choice = player.valid_choices(piece_id, dice_number, Act::Free);
    //     }

    //     player.make_move(piece_id, dice_number, choice);
    //     assert_eq!(player.piece(0).borrow_mut().position(), 0);
    //     assert_eq!(player.board().borrow_mut().outside(0).pieces.len(), 1);
    //     assert_eq!(
    //         player.board().borrow_mut().outside[0].player_id,
    //         Some(board::PlayerID::Player0)
    //     );
    //     player.die(piece_id);

    //     for dice_number in 1..7 {
    //         if dice_number == 5 {
    //             continue;
    //         }
    //         player.free_piece(piece_id);

    //         let choice = player.valid_choices(piece_id, dice_number, Act::Move);
    //         player.make_move(piece_id, dice_number, choice);
    //         assert_eq!(player.piece(0).borrow_mut().position(), dice_number);
    //         assert_eq!(
    //             player
    //                 .board()
    //                 .borrow_mut()
    //                 .outside(dice_number)
    //                 .pieces
    //                 .len(),
    //             1
    //         );
    //         assert_eq!(
    //             player.board().borrow_mut().outside[dice_number as usize].player_id,
    //             Some(board::PlayerID::Player0)
    //         );
    //         assert_eq!(player.board().borrow_mut().outside(0).pieces.len(), 0);
    //         player.die(piece_id);
    //     }
    // }