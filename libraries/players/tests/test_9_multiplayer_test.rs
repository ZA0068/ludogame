use board::Board;

use players::Player;
use std::{cell::RefCell, rc::Rc};

#[cfg(test)]
mod init_players_tests {
    use super::*;
    #[test]
    fn add_all_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let board2 = Rc::new(RefCell::new(Board::new()));
        let player0 = Player::new(0, board.clone(), None);
        let player1 = Player::new(1, board.clone(), None);
        let player2 = Player::new(2, board.clone(), None);
        let player3 = Player::new(3, board.clone(), None);

        assert_eq!(player0.id(), 0);
        assert_eq!(player1.id(), 1);
        assert_eq!(player2.id(), 2);
        assert_eq!(player3.id(), 3);

        assert_eq!(player0.board().as_ptr(), board.as_ptr());
        assert_eq!(player1.board().as_ptr(), player0.board().as_ptr());
        assert_eq!(player2.board().as_ptr(), player1.board().as_ptr());
        assert_eq!(player3.board().as_ptr(), player2.board().as_ptr());
        assert_eq!(board.as_ptr(), player3.board().as_ptr());
        assert_ne!(board2.as_ptr(), player0.board().as_ptr());
    }

    #[test]
    #[should_panic]
    fn invalid_player_id_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(4, board, None);
        assert_eq!(player.id(), 4);
    }
}

// #[cfg(test)]
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

//         assert_eq!(player1.piece(0).borrow().position(), 0);
//         assert_eq!(player2.piece(0).borrow().position(), 13);
//         assert_eq!(player1.board().borrow().outside[0].pieces.len(), 1);
//         assert_eq!(
//             player1.board().borrow().outside[0].player_id,
//             Some(board::PlayerID::Player0)
//         );
//         assert_eq!(player2.board().borrow_mut().outside(13).pieces.len(), 1);
//         assert_eq!(
//             player2
//                 .board()
//                 .borrow_mut()
//                 .outside(13)
//                 .piece(0)
//                 .borrow()
//                 .position(),
//             13
//         );
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

//         assert_eq!(player1.piece(0).borrow().position(), 0);
//         assert_eq!(player2.piece(0).borrow().position(), 13);
//         assert_eq!(player3.piece(0).borrow().position(), 26);
//         assert_eq!(player4.piece(0).borrow().position(), 39);

//         assert_eq!(player1.board().borrow().outside[0].pieces.len(), 1);
//         assert_eq!(
//             player1.board().borrow().outside[0].player_id,
//             Some(board::PlayerID::Player0)
//         );
//         assert_eq!(player1.board().borrow().outside[13].pieces.len(), 1);
//         assert_eq!(
//             player1.board().borrow().outside[13].player_id,
//             Some(board::PlayerID::Player1)
//         );
//         assert_eq!(player1.board().borrow().outside[26].pieces.len(), 1);
//         assert_eq!(
//             player1.board().borrow().outside[26].player_id,
//             Some(board::PlayerID::Player2)
//         );
//         assert_eq!(player1.board().borrow().outside[39].pieces.len(), 1);
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

//         assert_eq!(player1.piece(0).borrow().position(), 6);
//         assert_eq!(player2.piece(0).borrow().position(), 19);

//         assert_eq!(player1.board().borrow().outside[0].pieces.len(), 0);
//         assert_eq!(player1.board().borrow().outside[6].pieces.len(), 1);
//         assert_eq!(
//             player1.board().borrow().outside[6].player_id,
//             Some(board::PlayerID::Player0)
//         );
//         assert_eq!(player2.board().borrow().outside[13].pieces.len(), 0);
//         assert_eq!(player2.board().borrow().outside[19].pieces.len(), 1);
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

//         assert_eq!(player1.piece(0).borrow().position(), 6);
//         assert_eq!(player2.piece(0).borrow().position(), 19);
//         assert_eq!(player3.piece(0).borrow().position(), 32);
//         assert_eq!(player4.piece(0).borrow().position(), 45);

//         assert_eq!(player1.board().borrow_mut().outside[0].pieces.len(), 0);
//         assert_eq!(player1.board().borrow_mut().outside[6].pieces.len(), 1);
//         assert_eq!(
//             player1.board().borrow_mut().outside[6].player_id,
//             Some(board::PlayerID::Player0)
//         );
//         assert_eq!(player2.board().borrow_mut().outside[13].pieces.len(), 0);
//         assert_eq!(player2.board().borrow_mut().outside[19].pieces.len(), 1);
//         assert_eq!(
//             player2.board().borrow_mut().outside[19].player_id,
//             Some(board::PlayerID::Player1)
//         );
//         assert_eq!(player3.board().borrow_mut().outside[26].pieces.len(), 0);
//         assert_eq!(player3.board().borrow_mut().outside[32].pieces.len(), 1);
//         assert_eq!(
//             player3.board().borrow_mut().outside[32].player_id,
//             Some(board::PlayerID::Player2)
//         );
//         assert_eq!(player4.board().borrow_mut().outside[39].pieces.len(), 0);
//         assert_eq!(player4.board().borrow_mut().outside[45].pieces.len(), 1);
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
//         player2.move_piece(0, 36);
//         player2.move_piece(0, 2);
//         assert_eq!(player2.piece(0).borrow().position(), 51);

//         player2.free_piece(1);
//         player2.move_piece(1, 36);
//         player2.move_piece(1, 6);
//         assert_eq!(player2.piece(1).borrow().position(), 3);

//         let mut player3 = Player::new(2, board.clone(), Some(dice.clone()));
//         player3.free_piece(0);
//         player3.move_piece(0, 23);
//         player3.move_piece(0, 2);
//         assert_eq!(player3.piece(0).borrow().position(), 51);

//         player3.free_piece(1);
//         player3.move_piece(1, 23);
//         player3.move_piece(1, 6);
//         assert_eq!(player3.piece(1).borrow().position(), 3);

//         let mut player4 = Player::new(3, board, Some(dice));
//         player4.free_piece(0);
//         player4.move_piece(0, 10);
//         player4.move_piece(0, 2);
//         assert_eq!(player4.piece(0).borrow().position(), 51);

//         player4.free_piece(1);
//         player4.move_piece(1, 10);
//         player4.move_piece(1, 6);
//         assert_eq!(player4.piece(1).borrow().position(), 3);
//     }

//     #[test]
//     fn two_player_kill_test() {
//         let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
//         let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
//         let mut player = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut opponent = Player::new(1, board, Some(dice));

//         player.free_piece(0);
//         player.make_move(0, 17, Act::Move);
//         assert_eq!(player.piece(0).borrow().position(), 17);

//         opponent.free_piece(0);
//         assert_eq!(opponent.piece(0).borrow().position(), 13);

//         let diceroll2 = 4;
//         let choice2 = opponent.valid_choices(0, diceroll2, Act::Kill);
//         assert_eq!(choice2, Act::Kill);
//         opponent.make_move(0, diceroll2, choice2);

//         assert_eq!(player.piece(0).borrow().position(), -1);
//         assert!(player.piece(0).borrow().is_home());
//         assert_eq!(player.board().borrow_mut().home(0).pieces.len(), 4);
//         assert!(player
//             .board()
//             .borrow_mut()
//             .home(0)
//             .piece(0)
//             .borrow()
//             .is_home());

//         assert_eq!(opponent.piece(0).borrow().position(), 17);
//         assert_eq!(
//             player.board().borrow_mut().outside(17).player_id,
//             Some(board::PlayerID::Player1)
//         );
//     }

//     #[test]
//     fn suicide_test() {
//         let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
//         let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
//         let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut player2 = Player::new(1, board, Some(dice));

//         player1.free_piece(0);
//         player1.free_piece(1);
//         player1.move_piece(0, 17);
//         player1.move_piece(1, 17);

//         player2.free_piece(0);
//         assert_eq!(player2.piece(0).borrow().position(), 13);

//         let diceroll2 = 4;
//         let choice2 = player2.valid_choices(0, diceroll2, Act::Kill);
//         assert_eq!(choice2, Act::Nothing);
//         let choice2 = player2.valid_choices(0, diceroll2, Act::Die);
//         assert_eq!(choice2, Act::Die);
//         player2.make_move(0, diceroll2, choice2);

//         assert_eq!(player1.piece(0).borrow().position(), 17);
//         assert!(!player1.piece(0).borrow().is_home());
//         assert_eq!(
//             player1
//                 .board()
//                 .borrow_mut()
//                 .outside(17)
//                 .piece(0)
//                 .borrow_mut()
//                 .position(),
//             17
//         );
//         assert!(!player1
//             .board()
//             .borrow_mut()
//             .outside(17)
//             .piece(0)
//             .borrow_mut()
//             .is_home());
//         assert_eq!(
//             player1.board().borrow_mut().outside(17).player_id,
//             Some(board::PlayerID::Player0)
//         );

//         assert_eq!(player2.piece(0).borrow().position(), -1);
//         assert!(player2.piece(0).borrow().is_home());

//         assert_eq!(
//             player2
//                 .board()
//                 .borrow_mut()
//                 .home(1)
//                 .piece(0)
//                 .borrow_mut()
//                 .position(),
//             -1
//         );
//         assert!(player2
//             .board()
//             .borrow_mut()
//             .home(1)
//             .piece(0)
//             .borrow_mut()
//             .is_home());
//         assert_eq!(player2.board().borrow_mut().home(1).pieces.len(), 4);
//         assert_eq!(
//             player2.board().borrow_mut().outside(17).player_id,
//             Some(board::PlayerID::Player0)
//         );
//     }

//     #[test]
//     fn two_player_star_kill_tests() {
//         let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
//         let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
//         let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut player2 = Player::new(1, board, Some(dice));
//         let piece_0 = 0;
//         let piece_1 = 1;

//         player1.free_piece(piece_0);
//         player1.free_piece(piece_1);
//         let mut dice_number = 18;
//         player1.move_piece(piece_0, dice_number);
//         assert_eq!(player1.piece(piece_0).borrow().position(), 18);

//         dice_number = 24;
//         player1.move_piece(piece_1, dice_number);
//         assert_eq!(player1.piece(piece_1).borrow().position(), 24);

//         player2.free_piece(piece_0);
//         dice_number = 5;
//         let choice2 = player2.valid_choices(piece_0, dice_number, Act::Kill);
//         assert_eq!(choice2, Act::Kill);
//         player2.make_move(piece_0, dice_number, choice2);

//         assert_eq!(player1.piece(piece_0).borrow().position(), -1);
//         assert_eq!(player1.piece(piece_1).borrow().position(), -1);
//         assert_eq!(player1.board().borrow_mut().outside(18).pieces.len(), 0);
//         assert_eq!(player1.board().borrow_mut().outside(18).player_id, None);
//         assert!(player1.piece(piece_0).borrow().is_home());
//         assert!(player1.piece(piece_1).borrow().is_home());

//         assert_eq!(player2.piece(0).borrow().position(), 24);
//         assert_eq!(player2.board().borrow_mut().outside(24).pieces.len(), 1);
//         assert_eq!(
//             player2.board().borrow_mut().outside(24).player_id,
//             Some(board::PlayerID::Player1)
//         );
//     }

//     #[test]
//     fn star_sucide_test() {
//         let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
//         let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
//         let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut player2 = Player::new(1, board, Some(dice));
//         let piece_0 = 0;
//         let piece_1 = 1;

//         player1.free_piece(piece_0);
//         player1.free_piece(piece_1);
//         let mut dice_number = 18;
//         player1.move_piece(piece_0, dice_number);
//         player1.move_piece(piece_1, dice_number);
//         assert_eq!(player1.piece(piece_0).borrow().position(), 18);
//         assert_eq!(player1.piece(piece_1).borrow().position(), 18);

//         player2.free_piece(piece_0);
//         dice_number = 5;

//         let choice2 = player2.valid_choices(piece_0, dice_number, Act::Kill);
//         assert_eq!(choice2, Act::Nothing);

//         let choice2 = player2.valid_choices(piece_0, dice_number, Act::Die);
//         assert_eq!(choice2, Act::Die);
//         player2.make_move(piece_0, dice_number, choice2);

//         assert_eq!(player1.piece(piece_0).borrow().position(), 18);
//         assert_eq!(player1.piece(piece_1).borrow().position(), 18);
//         assert_eq!(player1.board().borrow_mut().outside(18).pieces.len(), 2);
//         assert_eq!(
//             player1.board().borrow_mut().outside(18).player_id,
//             Some(board::PlayerID::Player0)
//         );

//         assert!(player2.piece(piece_0).borrow().is_home());
//         assert_eq!(player2.piece(piece_0).borrow().position(), -1);
//         assert_eq!(player2.board().borrow_mut().home(1).pieces.len(), 4);
//     }

//     #[test]
//     fn star_sucide_test_2() {
//         let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
//         let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
//         let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut player2 = Player::new(1, board, Some(dice));
//         let piece_0 = 0;
//         let piece_1 = 1;

//         player1.free_piece(piece_0);
//         player1.free_piece(piece_1);
//         let mut dice_number = 24;
//         player1.move_piece(piece_0, dice_number);
//         player1.move_piece(piece_1, dice_number);
//         assert_eq!(player1.piece(piece_0).borrow().position(), 24);
//         assert_eq!(player1.piece(piece_1).borrow().position(), 24);

//         player2.free_piece(piece_0);
//         dice_number = 5;

//         let choice2 = player2.valid_choices(piece_0, dice_number, Act::Kill);
//         assert_eq!(choice2, Act::Nothing);

//         let choice2 = player2.valid_choices(piece_0, dice_number, Act::Die);
//         assert_eq!(choice2, Act::Die);
//         player2.make_move(piece_0, dice_number, choice2);

//         assert_eq!(player1.piece(piece_0).borrow().position(), 24);
//         assert_eq!(player1.piece(piece_1).borrow().position(), 24);
//         assert_eq!(player1.board().borrow_mut().outside(18).pieces.len(), 0);
//         assert_eq!(player1.board().borrow_mut().outside(24).pieces.len(), 2);
//         assert_eq!(
//             player1.board().borrow_mut().outside(24).player_id,
//             Some(board::PlayerID::Player0)
//         );
//         assert!(player2.piece(piece_0).borrow().is_home());
//         assert_eq!(player2.piece(piece_0).borrow().position(), -1);
//         assert_eq!(player2.board().borrow_mut().home(1).pieces.len(), 4);
//     }

//     #[test]
//     fn try_to_die_test() {
//         let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
//         let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
//         let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut player2 = Player::new(1, board, Some(dice));

//         let piece_0 = 0;
//         let piece_1 = 1;
//         let dice_number = 18;
//         player1.free_piece(piece_0);
//         player1.free_piece(piece_1);
//         player1.move_piece(piece_0, dice_number);
//         player1.move_piece(piece_1, dice_number);
//         player2.free_piece(piece_0);

//         let action = player2.try_to_die(piece_0, 5);
//         assert_eq!(action, Act::Die);

//         player1.die(piece_0);
//         player1.die(piece_1);
//         player1.free_piece(piece_0);
//         player1.free_piece(piece_1);

//         let dice_number = 17;
//         player1.move_piece(piece_0, dice_number);
//         player1.move_piece(piece_1, dice_number);
//         let dice_number = 1;
//         player1.move_piece(piece_0, dice_number);
//         player1.move_piece(piece_1, dice_number);

//         let action = player2.try_to_die(piece_0, 5);
//         assert_eq!(action, Act::Die);

//         player1.free_piece(2);
//         player1.move_piece(2, 21);

//         let action = player2.try_to_die(piece_0, 8);
//         assert_eq!(action, Act::Die);
//     }

//     #[test]
//     fn try_to_kill_test() {
//         let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
//         let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
//         let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut player2 = Player::new(1, board, Some(dice));

//         let piece_0 = 0;
//         let piece_1 = 1;
//         let dice_number = 18;
//         player1.free_piece(piece_0);
//         player1.free_piece(piece_1);
//         player1.move_piece(piece_0, dice_number);
//         player1.move_piece(piece_1, 15);
//         player2.free_piece(piece_0);

//         let action = player2.try_to_kill(piece_0, 5);
//         assert_eq!(action, Act::Kill);

//         let action = player2.try_to_kill(piece_0, 2);
//         assert_eq!(action, Act::Kill);

//         player1.die(piece_0);
//         player1.free_piece(piece_0);

//         let dice_number = 17;
//         player1.move_piece(piece_0, dice_number);
//         let dice_number = 1;
//         player1.move_piece(piece_0, dice_number);

//         let action = player2.try_to_kill(piece_0, 5);
//         assert_eq!(action, Act::Kill);

//         player1.free_piece(2);
//         player1.move_piece(2, 21);

//         let action = player2.try_to_kill(piece_0, 8);
//         assert_eq!(action, Act::Nothing);
//     }

//     #[test]
//     #[ignore = "long test"]
//     fn first_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(0, board, Some(dice));
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
//     }

//     #[test]
//     #[ignore = "long test"]
//     fn second_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(1, board, Some(dice));

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
//     }

//     #[test]
//     #[ignore = "long test"]
//     fn third_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(2, board, Some(dice));
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
//     }

//     #[test]
//     #[ignore = "long test"]
//     fn fourth_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player = Player::new(3, board, Some(dice));
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
//     }
//     #[test]
//     fn all_single_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));

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

//         for i in 0..4 {
//             let mut player = Player::new(i, board.clone(), Some(dice.clone()));

//             while !player.is_finished() {
//                 player.my_turn();
//                 player.random_play(actions.clone());
//                 player.print_status();
//             }
//             assert!(player.is_finished());
//         }
//     }

//     #[test]
//     #[ignore = "long test"]
//     fn replay_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));

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

//         for i in 0..4 {
//             let mut player = Player::new(i, board.clone(), Some(dice.clone()));
//             for _ in 0..10 {
//                 while !player.is_finished() {
//                     player.my_turn();
//                     player.random_play(actions.clone());
//                 }
//                 assert!(player.is_finished());
//                 player.reset();
//             }
//         }
//     }

//     #[test]
//     #[ignore = "long test"]
//     fn two_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player0 = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut player1 = Player::new(1, board, Some(dice));
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
//         loop {
//             player0.my_turn();
//             player0.random_play(actions.clone());
//             if player0.is_finished() {
//                 println!("Player 0 wins");
//                 break;
//             }
//             player1.my_turn();
//             player1.random_play(actions.clone());
//             if player1.is_finished() {
//                 println!("Player 1 wins");
//                 break;
//             }
//         }
//         assert!(player0.is_finished() || player1.is_finished());
//     }

//     #[test]
//     #[ignore = "long test"]
//     fn two_players_repeated_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player0 = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut player1 = Player::new(1, board, Some(dice));
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
//         for _ in 0..20 {
//             loop {
//                 player0.my_turn();
//                 player0.random_play(actions.clone());
//                 if player0.is_finished() {
//                     println!("Player 0 wins");
//                     break;
//                 }
//                 player1.my_turn();
//                 player1.random_play(actions.clone());
//                 if player1.is_finished() {
//                     println!("Player 1 wins");
//                     break;
//                 }
//             }
//             assert!(player0.is_finished() || player1.is_finished());
//             player0.reset();
//             player1.reset();
//         }
//     }

//     #[test]
//     #[ignore = "long test"]
//     fn other_two_players_repeated_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player0 = Player::new(2, board.clone(), Some(dice.clone()));
//         let mut player1 = Player::new(3, board, Some(dice));
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
//         for _ in 0..20 {
//             loop {
//                 player0.my_turn();
//                 player0.random_play(actions.clone());
//                 player0.print_status();
//                 if player0.is_finished() {
//                     println!("Player 0 wins");
//                     break;
//                 }
//                 player1.my_turn();
//                 player1.random_play(actions.clone());
//                 player1.print_status();
//                 if player1.is_finished() {
//                     println!("Player 1 wins");
//                     break;
//                 }
//             }
//             assert!(player0.is_finished() || player1.is_finished());
//             player0.reset();
//             player1.reset();
//         }
//     }

//     #[test]
//     #[ignore = "super long test"]
//     fn all_player_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let dice = Rc::new(RefCell::new(Dice::new()));
//         let mut player0 = Player::new(0, board.clone(), Some(dice.clone()));
//         let mut player1 = Player::new(1, board.clone(), Some(dice.clone()));
//         let mut player2 = Player::new(2, board.clone(), Some(dice.clone()));
//         let mut player3 = Player::new(3, board, Some(dice));
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
//         let mut winrate: Vec<f32> = vec![0.0; 4];
//         for _ in 0..30 {
//             loop {
//                 player0.my_turn();
//                 player0.random_play(actions.clone());
//                 // player0.print_status();
//                 if player0.is_finished() {
//                     println!("Player 0 wins");
//                     winrate[0] += 1.0;
//                     break;
//                 }
//                 player1.my_turn();
//                 player1.random_play(actions.clone());
//                 // player1.print_status();
//                 if player1.is_finished() {
//                     println!("Player 1 wins");
//                     winrate[1] += 1.0;
//                     break;
//                 }

//                 player2.my_turn();
//                 player2.random_play(actions.clone());
//                 // player2.print_status();
//                 if player2.is_finished() {
//                     println!("Player 2 wins");
//                     winrate[2] += 1.0;
//                     break;
//                 }

//                 player3.my_turn();
//                 player3.random_play(actions.clone());
//                 // player3.print_status();
//                 if player3.is_finished() {
//                     println!("Player 3 wins");
//                     winrate[3] += 1.0;
//                     break;
//                 }
//             }
//             assert!(
//                 player0.is_finished()
//                     || player1.is_finished()
//                     || player2.is_finished()
//                     || player3.is_finished()
//             );
//             player0.reset();
//             player1.reset();
//             player2.reset();
//             player3.reset();
//         }
//         println!("Player 0 winrate: {}", winrate[0] / 30.0);
//         println!("Player 1 winrate: {}", winrate[1] / 30.0);
//         println!("Player 2 winrate: {}", winrate[2] / 30.0);
//         println!("Player 3 winrate: {}", winrate[3] / 30.0);
//     }
// }
