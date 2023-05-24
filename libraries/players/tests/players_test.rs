use players::Player;
use board::Board;
use dice::Dice;
use std::{rc::Rc, cell::RefCell};

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

    #[test]
    fn add_all_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let board2 = Rc::new(RefCell::new(Board::new()));
        let player0= Player::new(0, board.clone(), None);
        let player1= Player::new(1, board.clone(), None);
        let player2= Player::new(2, board.clone(), None);
        let player3= Player::new(3, board.clone(), None);

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
    fn get_pieces_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(0, board, None);

        let piece = player.piece(0);
        assert_eq!(piece.id(), 0);

        let piece = player.piece(1);
        assert_eq!(piece.id(), 1);

        let piece = player.piece(2);
        assert_eq!(piece.id(), 2);

        let piece = player.piece(3);
        assert_eq!(piece.id(), 3);
    }

    #[test]
    fn get_piece_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(0, board, None);
        (0..4).for_each(|i| {
            let piece = player.piece(i);
            assert_eq!(piece.id(), i);
            assert!(player.piece(i).is_home());
            assert!(player.piece(i).is_safe());
        });
    }

    #[test]
    fn player_with_dice_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(0, board, None);
        
        let result = player.roll_dice();
        assert!(result == 0);
        let dice = Rc::new(RefCell::new(Dice::new()));
        
        player.take_dice(dice);
        let result = player.roll_dice();
        assert!(result > 0 && result < 7);

        player.give_dice();
        let result = player.roll_dice();
        assert!(result == 0);
    }
}

mod move_single_piece_test {

    use super::*;

    #[test]
    fn free_piece_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(0, board, None);
        
        assert!(player.piece(0).is_home());
        assert_eq!(player.board().borrow().home(0).unwrap().number_of_pieces, 4);
        
        player.free_piece(0);
        assert!(!player.piece(0).is_home());
        assert!(player.piece(0).is_dangerous());
        assert_eq!(player.piece(0).position(), 0);
        assert_eq!(player.board().borrow().home(0).unwrap().number_of_pieces, 3);
        assert_eq!(player.board().borrow().outside(0).unwrap().number_of_pieces, 1);
    }

    #[test]
    fn update_piece_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));
        
        let next_position = 4;
        player.free_piece(0);
        player.update_piece(0, next_position);
        assert_eq!(player.piece(0).position(), next_position);
    }

    #[test]
    fn update_piece_state_test()
    {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));
        
        let next_position = 4;
        player.free_piece(0);
        player.update_piece_state(0, next_position);

        assert_eq!(player.board().borrow().outside(4).unwrap().number_of_pieces, 1);
        assert_eq!(player.board().borrow().outside(0).unwrap().number_of_pieces, 0);

        let new_pos = next_position + 2;
        player.update_piece_state(next_position, new_pos);
        assert_eq!(player.board().borrow().outside(6).unwrap().number_of_pieces, 1);
        assert_eq!(player.board().borrow().outside(4).unwrap().number_of_pieces, 0);
    }

    #[test]
    fn update_piece_test_2() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        let next_position = 4;
        player.free_piece(0);
        player.update_piece(0, next_position);

        assert_eq!(player.piece(0).position(), next_position);
        assert_eq!(player.board().borrow().outside(4).unwrap().number_of_pieces, 1);
    }

    #[test]
    fn update_piece_by_dice_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        while player.roll_dice() != 6 {}
        player.free_piece(0);
        let result = player.roll_dice();
        player.update_piece(0, result);
        assert_eq!(player.piece(0).position(), result);
        assert_eq!(player.board().borrow().outside(result as usize).unwrap().number_of_pieces, 1);
        assert_eq!(player.board().borrow().outside(result as usize).unwrap().player_id, Some(board::PlayerID::Player0));
        assert_eq!(player.board().borrow().outside(0).unwrap().number_of_pieces, 0);
    }

    #[test]
    fn enter_inside_test() {
    let board = Rc::new(RefCell::new(Board::new()));
    let dice = Rc::new(RefCell::new(Dice::new()));
    let mut player = Player::new(0, board, Some(dice));

    let piece_id = 0;
    player.free_piece(piece_id);
    player.update_piece(piece_id, 50);
    player.enter_inside(piece_id, 50, 56);

    assert_eq!(player.piece(piece_id).position(), 56);
    assert_eq!(player.board().borrow().inside(56).unwrap().number_of_pieces, 1);
    assert_eq!(player.board().borrow().outside(50).unwrap().number_of_pieces, 0);
    assert_eq!(player.board().borrow().inside(56).unwrap().player_id, Some(board::PlayerID::Player0));
    }

    #[test]
    fn go_back_when_overshoot_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        let piece_id = 0;
        let position = 50;
        let mut new_position = position + 6;
        player.free_piece(piece_id);
        player.update_piece(piece_id, position);
        player.enter_inside(piece_id, position, new_position);
        new_position += 2;

        player.go_back_when_overshoot(piece_id, new_position);

        assert_eq!(player.piece(piece_id).position(), 50);

    }
}

mod multipiece_test {
    use super::*;

    #[test]
    fn free_all_pieces_test()
    {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        for piece_id in 0..4 {
            player.free_piece(piece_id);
            assert!(!player.piece(piece_id).is_home());
            assert!(player.piece(piece_id).is_dangerous());
            assert!(player.piece(piece_id).is_safe());
            assert_eq!(player.piece(piece_id).position(), 0);
        }
        assert_eq!(player.board().borrow().home(0).unwrap().number_of_pieces, 0);
        assert_eq!(player.board().borrow().outside(0).unwrap().number_of_pieces, 4);
    }

    // #[test]
    // fn two_piece_at_same_test() {
    //     let mut player = Player::new(0);
    //     player.free_piece(0);
    //     player.free_piece(1);

    //     player.make_move(0, 6);
    //     player.make_move(1, 6);

    //     assert_eq!(player.piece(0).position(), 6);
    //     assert_eq!(player.piece(1).position(), 6);
    //     assert!(player.piece(0).is_dangerous());
    //     assert!(player.piece(1).is_dangerous());
    //     assert!(player.piece(0).is_safe());
    //     assert!(player.piece(1).is_safe());

    //     player.make_move(0, 1);
    //     assert_eq!(player.piece(0).position(), 7);
    //     assert!(!player.piece(0).is_safe());
    //     assert!(!player.piece(1).is_safe());
    //     assert!(!player.piece(0).is_dangerous());
    //     assert!(!player.piece(1).is_dangerous());
    // }

    // #[test]
    // fn all_pieces_at_same_place_test() {
    //     let mut player = Player::new(0);
    //     player.free_piece(0);
    //     player.free_piece(1);
    //     player.free_piece(2);
    //     player.free_piece(3);

    //     player.make_move(0, 6);
    //     player.make_move(1, 6);
    //     player.make_move(2, 6);
    //     player.make_move(3, 6);

    //     assert_eq!(player.piece(0).position(), 6);
    //     assert_eq!(player.piece(1).position(), 6);
    //     assert_eq!(player.piece(2).position(), 6);
    //     assert_eq!(player.piece(3).position(), 6);

    //     assert!(player.piece(0).is_safe());
    //     assert!(player.piece(1).is_safe());
    //     assert!(player.piece(2).is_safe());
    //     assert!(player.piece(3).is_safe());

    //     assert!(player.piece(0).is_dangerous());
    //     assert!(player.piece(1).is_dangerous());
    //     assert!(player.piece(2).is_dangerous());
    //     assert!(player.piece(3).is_dangerous());

    //     player.make_move(0, 1);
    //     assert_eq!(player.piece(0).position(), 7);
    //     assert!(!player.piece(0).is_safe());
    //     assert!(!player.piece(0).is_dangerous());

    //     assert!(player.piece(1).is_safe());
    //     assert!(player.piece(2).is_safe());
    //     assert!(player.piece(3).is_safe());
    //     assert!(player.piece(1).is_dangerous());
    //     assert!(player.piece(2).is_dangerous());
    //     assert!(player.piece(3).is_dangerous());

    //     player.make_move(1, 3);
    //     assert_eq!(player.piece(1).position(), 9);
    //     assert!(!player.piece(1).is_safe());
    //     assert!(!player.piece(1).is_dangerous());

    //     assert!(player.piece(2).is_safe());
    //     assert!(player.piece(3).is_safe());
    //     assert!(player.piece(2).is_dangerous());
    //     assert!(player.piece(3).is_dangerous());

    //     player.make_move(2, 4);
    //     assert_eq!(player.piece(2).position(), 10);
    //     assert!(!player.piece(2).is_safe());
    //     assert!(!player.piece(2).is_dangerous());
    //     assert!(!player.piece(3).is_safe());
    //     assert!(!player.piece(3).is_dangerous());
    // }

    // #[test]
    // fn all_pieces_in_goal_test() {
    //     let mut player = Player::new(0);
    //     player.free_piece(0);
    //     player.free_piece(1);
    //     player.free_piece(2);
    //     player.free_piece(3);

    //     player.piece(0).set_position(99);
    //     player.piece(1).set_position(99);
    //     player.piece(2).set_position(99);
    //     player.piece(3).set_position(99);

    //     player.update_piece_state(0);
    //     player.update_piece_state(1);
    //     player.update_piece_state(2);
    //     player.update_piece_state(3);

    //     assert!(player.piece(0).is_goal());
    //     assert!(player.piece(1).is_goal());
    //     assert!(player.piece(2).is_goal());
    //     assert!(player.piece(3).is_goal());

    //     assert!(player.is_finished());
    // }

    // #[test]
    // fn all_pieces_in_goal_test_part_2() {
    //     let mut player = Player::new(0);
    //     player.free_piece(0);
    //     player.free_piece(1);
    //     player.free_piece(2);
    //     player.free_piece(3);

    //     player.piece(0).set_position(52);
    //     player.piece(1).set_position(52);
    //     player.piece(2).set_position(52);
    //     player.piece(3).set_position(52);

    //     player.make_move(0, 5);
    //     player.make_move(1, 5);
    //     player.make_move(2, 5);
    //     player.make_move(3, 5);

    //     assert!(player.piece(0).is_goal());
    //     assert!(player.piece(1).is_goal());
    //     assert!(player.piece(2).is_goal());
    //     assert!(player.piece(3).is_goal());

    //     assert!(player.is_finished());
    // }

    // #[test]
    // fn all_pieces_in_goal_test_part_3() {
    //     let mut player = Player::new(0);
    //     player.free_piece(0);
    //     player.free_piece(1);
    //     player.free_piece(2);
    //     player.free_piece(3);

    //     player.piece(0).set_position(49);
    //     player.piece(1).set_position(49);
    //     player.piece(2).set_position(49);
    //     player.piece(3).set_position(49);

    //     player.make_move(0, 1);
    //     player.make_move(1, 1);
    //     player.make_move(2, 1);
    //     player.make_move(3, 1);

    //     assert!(player.piece(0).is_goal());
    //     assert!(player.piece(1).is_goal());
    //     assert!(player.piece(2).is_goal());
    //     assert!(player.piece(3).is_goal());

    //     assert!(player.is_finished());
    // }

    // #[test]
    // fn single_player_test() {
    //     let mut player = Player::new(0);
    //     while !player.is_finished() {
    //         let dice = player.roll_dice();
    //         println!("Dice hit : {}", dice);
    //         let piece_id = player.choose_piece();
    //         println!("Chosen piece: {}\n", piece_id);
    //         player.make_move(piece_id, dice);
    //         println!(
    //             "Piece 0: {:?}\nPiece 1: {:?}\nPiece 2: {:?}\nPiece 3: {:?}\n\n",
    //             player.piece(0).position(),
    //             player.piece(1).position(),
    //             player.piece(2).position(),
    //             player.piece(3).position()
    //         );
    //     }
    // }
}

mod multiplayer_test {
    use super::*;

    #[test]
    fn two_players_free_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
        let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
        let mut player2 = Player::new(1, board, Some(dice));

        player1.free_piece(0);
        player2.free_piece(0);

        assert_eq!(player1.piece(0).position(), 0);
        assert_eq!(player2.piece(0).position(), 13);
        assert_eq!(player1.board().borrow().outside(0).unwrap().number_of_pieces, 1);
        assert_eq!(player1.board().borrow().outside(0).unwrap().player_id, Some(board::PlayerID::Player0));
        assert_eq!(player2.board().borrow().outside(13).unwrap().number_of_pieces, 1);
        assert_eq!(player1.board().borrow().outside(13).unwrap().player_id, Some(board::PlayerID::Player1));
    }

    #[test]
    fn all_players_free_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
        let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
        let mut player2 = Player::new(1, board.clone(), Some(dice.clone()));
        let mut player3 = Player::new(2, board.clone(), Some(dice.clone()));
        let mut player4 = Player::new(3, board, Some(dice));

        player1.free_piece(0);
        player2.free_piece(0);
        player3.free_piece(0);
        player4.free_piece(0);

        assert_eq!(player1.piece(0).position(), 0);
        assert_eq!(player2.piece(0).position(), 13);
        assert_eq!(player3.piece(0).position(), 26);
        assert_eq!(player4.piece(0).position(), 39);
        assert_eq!(player1.board().borrow().outside(0).unwrap().number_of_pieces, 1);
        assert_eq!(player1.board().borrow().outside(0).unwrap().player_id, Some(board::PlayerID::Player0));
        assert_eq!(player2.board().borrow().outside(13).unwrap().number_of_pieces, 1);
        assert_eq!(player2.board().borrow().outside(13).unwrap().player_id, Some(board::PlayerID::Player1));
        assert_eq!(player2.board().borrow().outside(26).unwrap().number_of_pieces, 1);
        assert_eq!(player2.board().borrow().outside(26).unwrap().player_id, Some(board::PlayerID::Player2));
        assert_eq!(player2.board().borrow().outside(39).unwrap().number_of_pieces, 1);
        assert_eq!(player2.board().borrow().outside(39).unwrap().player_id, Some(board::PlayerID::Player3));
    }

    // #[test]
    // fn two_players_move_test() {
    //     let mut player1 = Player::new(0);
    //     let mut player2 = Player::new(1);

    //     player1.free_piece(0);
    //     player2.free_piece(0);

    //     player1.make_move(0, 6);
    //     player2.make_move(0, 6);

    //     assert_eq!(player1.piece(0).position(), 6);
    //     assert_eq!(player2.piece(0).position(), 19);
    // }

    // #[test]
    // fn all_players_move_test() {
    //     let mut player1 = Player::new(0);
    //     let mut player2 = Player::new(1);
    //     let mut player3 = Player::new(2);
    //     let mut player4 = Player::new(3);

    //     player1.free_piece(0);
    //     player2.free_piece(0);
    //     player3.free_piece(0);
    //     player4.free_piece(0);

    //     player1.make_move(0, 6);
    //     player2.make_move(0, 6);
    //     player3.make_move(0, 6);
    //     player4.make_move(0, 6);

    //     assert_eq!(player1.piece(0).position(), 6);
    //     assert_eq!(player2.piece(0).position(), 19);
    //     assert_eq!(player3.piece(0).position(), 32);
    //     assert_eq!(player4.piece(0).position(), 45);
    // }

    // #[test]
    // fn other_player_circumvent_player_1() {
    //     let mut player2 = Player::new(1);

    //     player2.free_piece(0);
    //     player2.piece(0).set_position(50);
    //     player2.make_move(0, 1);
    //     assert_eq!(player2.piece(0).position(), 51);

    //     player2.piece(0).set_position(50);
    //     player2.make_move(0, 6);
    //     assert_eq!(player2.piece(0).position(), 4);

    //     let mut player3 = Player::new(2);

    //     player3.free_piece(0);
    //     player3.piece(0).set_position(50);
    //     player3.make_move(0, 1);
    //     assert_eq!(player3.piece(0).position(), 51);

    //     player3.piece(0).set_position(50);
    //     player3.make_move(0, 6);
    //     assert_eq!(player3.piece(0).position(), 4);

    //     let mut player4 = Player::new(3);

    //     player4.free_piece(0);
    //     player4.piece(0).set_position(50);
    //     player4.make_move(0, 1);
    //     assert_eq!(player4.piece(0).position(), 51);

    //     player4.piece(0).set_position(50);
    //     player4.make_move(0, 6);
    //     assert_eq!(player4.piece(0).position(), 4);
    // }

    // #[test]
    // #[ignore]
    // fn two_player_kill_test() {
    //     let mut player1 = Player::new(0);
    //     let mut player2 = Player::new(1);

    //     player1.free_piece(0);
    //     player2.free_piece(0);

    //     player1.piece(0).set_position(6);
    //     player2.piece(0).set_position(5);

    //     player1.make_move(0, 3);
    //     player2.make_move(0, 4);

    //     assert_eq!(player1.piece(0).position(), -1);
    //     assert!(player1.piece(0).is_home());

    //     assert_eq!(player2.piece(0).position(), 9);
    // }
}
