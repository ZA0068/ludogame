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

    #[test]
    fn get_pieces_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(0, board, None);

        let piece = player.board().borrow_mut().home(0).piece(0);
        assert_eq!(piece.borrow().id(), 0);

        let piece = player.board().borrow_mut().home(0).piece(1);
        assert_eq!(piece.borrow_mut().id(), 1);

        let piece = player.board().borrow_mut().home(0).piece(2);
        assert_eq!(piece.borrow().id(), 2);

        let piece = player.board().borrow_mut().home(0).piece(3);
        assert_eq!(piece.borrow().id(), 3);
    }

    #[test]
    fn get_piece_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let player = Player::new(0, board, None);
        (0..4).for_each(|i| {
            let piece = player.board().borrow_mut().home(0).piece(i);
            assert_eq!(piece.borrow().id(), i);
            assert!(piece.borrow().is_home());
            assert!(piece.borrow().is_safe());
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

    #[test]
    fn player_turn_test() {
        let dice = Rc::new(RefCell::new(Dice::new()));
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(0, board, Some(dice));
        assert!(!player.is_player_turn());
        player.my_turn();
        assert!(player.is_player_turn());

        let mut result = player.roll_dice();
        while result != 6 {
            result = player.roll_dice();
        }
        player.can_continue();
        assert!(player.is_player_turn());

        while result == 6 {
            result = player.roll_dice();
        }
        player.can_continue();
        assert!(!player.is_player_turn());
    }
}

mod move_single_piece_test {

    use super::*;

    #[test]
    fn free_piece_test() {
        let player_id = 0;
        let piece_id = 0;
        let new_position = 0;

        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(player_id, board, None);
        let piece = player.piece(piece_id);

        assert!(piece.borrow_mut().is_home());
        assert!(player
            .board()
            .borrow_mut()
            .home(player_id)
            .piece(0)
            .borrow_mut()
            .is_home());
        assert_eq!(player.board().borrow_mut().home(player_id).pieces.len(), 4);
        player.free_piece(piece_id);

        assert!(!piece.borrow_mut().is_home());
        assert!(piece.borrow_mut().is_dangerous());
        assert_eq!(piece.borrow_mut().position(), 0);

        assert_eq!(player.board().borrow_mut().home(player_id).pieces.len(), 3);
        assert_eq!(
            player
                .board()
                .borrow_mut()
                .outside(new_position)
                .pieces
                .len(),
            1
        );
        assert!(!player
            .board()
            .borrow_mut()
            .outside(new_position)
            .piece(piece_id)
            .borrow_mut()
            .is_home());
        assert!(player
            .board()
            .borrow_mut()
            .outside(new_position)
            .piece(piece_id)
            .borrow_mut()
            .is_dangerous());
    }

    #[test]
    fn update_piece_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));
        let piece_id = 0;
        let next_position = 4;

        player.free_piece(0);
        player.move_piece(0, next_position);

        let piece = player.piece(piece_id);
        assert_eq!(piece.borrow_mut().position(), next_position);
        assert!(!piece.borrow_mut().is_safe());
        assert!(!player
            .board()
            .borrow_mut()
            .outside(next_position)
            .piece(piece_id)
            .borrow_mut()
            .is_safe());
        assert_eq!(
            player
                .board()
                .borrow_mut()
                .outside(next_position)
                .piece(piece_id)
                .borrow_mut()
                .position(),
            next_position
        );
    }

    #[test]
    fn update_piece_state_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        let piece_id = 0;
        let mut next_position: i8 = 4;

        player.free_piece(piece_id);
        player.move_piece(piece_id, next_position);

        let piece = player.piece(piece_id);
        assert_eq!(piece.borrow_mut().position(), next_position);
        assert!(!piece.borrow_mut().is_safe());
        assert!(!player
            .board()
            .borrow_mut()
            .outside(next_position)
            .piece(piece_id)
            .borrow_mut()
            .is_safe());
        assert_eq!(
            player
                .board()
                .borrow_mut()
                .outside(next_position)
                .piece(piece_id)
                .borrow_mut()
                .position(),
            next_position
        );
        assert_eq!(
            player
                .board()
                .borrow_mut()
                .outside(next_position)
                .pieces
                .len(),
            1
        );
        assert_eq!(player.board().borrow_mut().outside(0).pieces.len(), 0);

        next_position = 2;
        player.move_piece(0, next_position);
        assert_eq!(player.board().borrow_mut().outside(6).pieces.len(), 1);
        assert_eq!(player.board().borrow_mut().outside(4).pieces.len(), 0);
    }

    #[test]
    fn valid_move_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        let piece_id = 0;
        let piece_move = player.valid_moves(piece_id, 1).1;
        assert!(!piece_move);

        let piece_move = player.valid_moves(piece_id, 7).1;
        assert!(!piece_move);

        let piece_move = player.valid_moves(piece_id, 6).1;
        assert!(piece_move);
        player.free_piece(piece_id);

        let piece_move = player.valid_moves(piece_id, 6).1;
        assert!(piece_move);

        let piece_id = 4;
        let piece_move = player.valid_moves(piece_id, 1).1;
        assert!(!piece_move);
    }

    #[test]
    fn choice_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        let piece_id: i8 = 0;
        let dice_number: i8 = 6;
        let action = Act::Free;

        let selected_action = player.valid_choices(piece_id, dice_number, action);

        assert_eq!(selected_action, Act::Free);
        assert_ne!(selected_action, Act::Nothing);
        player.free_piece(piece_id);

        let piece_id: i8 = 0;
        let dice_number: i8 = 6;
        let action = Act::Move;

        let selected_action = player.valid_choices(piece_id, dice_number, action);

        assert_eq!(selected_action, Act::Move);
        assert_ne!(selected_action, Act::Free);
        assert_ne!(selected_action, Act::Nothing);
    }

    #[test]
    fn update_piece_by_dice_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        player.free_piece(0);
        let mut result = player.roll_dice();
        if result == 5 {
            result = 1;
        }
        player.move_piece(0, result);
        assert_eq!(player.piece(0).borrow_mut().position(), result);
        assert_eq!(player.board().borrow_mut().outside(result).pieces.len(), 1);
        assert_eq!(
            player.board().borrow_mut().outside(result).player_id,
            Some(board::PlayerID::Player0)
        );
        assert_eq!(player.board().borrow_mut().outside(0).pieces.len(), 0);
    }

    #[test]
    fn move_by_dice_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        for i in 1..7 {
            if i == 5 {
                continue;
            }

            let piece_id = 0;
            let mut dice_roll = player.roll_dice();
            let mut choice = player.valid_choices(piece_id, dice_roll, Act::Free);

            while choice != Act::Free {
                dice_roll = player.roll_dice();
                choice = player.valid_choices(piece_id, dice_roll, Act::Free);
            }

            player.make_move(piece_id, dice_roll, choice);
            assert_eq!(player.piece(0).borrow_mut().position(), 0);
            assert_eq!(player.board().borrow_mut().outside(0).pieces.len(), 1);
            assert_eq!(
                player.board().borrow_mut().outside[0].player_id,
                Some(board::PlayerID::Player0)
            );

            let choice = player.valid_choices(piece_id, i, Act::Move);
            player.make_move(piece_id, i, choice);
            assert_eq!(player.piece(0).borrow_mut().position(), i);
            assert_eq!(player.board().borrow_mut().outside(i).pieces.len(), 1);
            assert_eq!(
                player.board().borrow_mut().outside[i as usize].player_id,
                Some(board::PlayerID::Player0)
            );
            assert_eq!(player.board().borrow_mut().outside(0).pieces.len(), 0);
            player.die(piece_id);
        }
    }

    #[test]
    fn enter_inside_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        let piece_id = 0;
        let mut dice_roll = player.roll_dice();
        let action = Act::Free;
        let mut valid_choices = player.valid_choices(piece_id, dice_roll, action);

        while valid_choices != Act::Free {
            dice_roll = player.roll_dice();
            valid_choices = player.valid_choices(piece_id, dice_roll, Act::Free);
        }
        player.make_move(piece_id, dice_roll, valid_choices);

        dice_roll = 5;

        player.move_piece(piece_id, 50);
        valid_choices = player.valid_choices(piece_id, dice_roll, Act::Move);
        player.make_move(piece_id, dice_roll, valid_choices);
        assert_eq!(
            player.piece(piece_id).borrow_mut().position(),
            51 + dice_roll
        );
        assert_eq!(
            player
                .board()
                .borrow_mut()
                .inside(51 + dice_roll)
                .pieces
                .len(),
            1
        );
        assert_eq!(
            player.board().borrow_mut().inside(51 + dice_roll).player_id,
            Some(board::PlayerID::Player0)
        );
    }

    #[test]
    fn enter_goal_from_outside_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        let piece_id = 0;
        player.free_piece(piece_id);
        player.move_piece(piece_id, 44);
        let choice = player.valid_choices(piece_id, 6, Act::Goal);
        player.make_move(piece_id, 6, choice);

        assert_eq!(player.piece(piece_id).borrow().position(), 99);
        assert!(player.piece(piece_id).borrow().is_goal());
        assert_eq!(player.board().borrow_mut().goal(0).pieces.len(), 1);
        assert_eq!(
            player.board().borrow().goal[0].player_id,
            Some(board::PlayerID::Player0)
        );
    }

    #[test]
    fn enter_goal_from_inside_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        let piece_id = 0;
        player.free_piece(piece_id);
        player.move_piece(piece_id, 49);
        player.move_piece(piece_id, 4);

        assert_eq!(player.piece(piece_id).borrow().position(), 54);
        assert!(!player.piece(piece_id).borrow().is_goal());
        assert_eq!(player.board().borrow_mut().inside(54).pieces.len(), 1);
        assert_eq!(player.board().borrow().inside[2].player_id, Some(board::PlayerID::Player0));

        player.goal(piece_id);

        assert_eq!(player.piece(piece_id).borrow().position(), 99);
        assert!(player.piece(piece_id).borrow().is_goal());
        assert_eq!(player.board().borrow_mut().goal(0).pieces.len(), 1);
        assert_eq!(
            player.board().borrow().goal[0].player_id,
            Some(board::PlayerID::Player0)
        );
    }

    #[test]
    fn move_back_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        let piece_id = 0;
        player.free_piece(piece_id);
        player.move_piece(piece_id, 50);
        player.move_piece(piece_id, 4);

        assert_eq!(player.piece(piece_id).borrow_mut().position(), 55);
        assert_eq!(player.board().borrow_mut().inside(55).pieces.len(), 1);

        player.move_piece(piece_id, 6);
        assert_eq!(player.piece(piece_id).borrow_mut().position(), 53);
        assert_eq!(player.board().borrow_mut().inside(53).pieces.len(), 1);
    }

    #[test]
    fn death_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        let piece_id = 0;
        player.free_piece(piece_id);
        player.move_piece(piece_id, 50);

        player.die(piece_id);
        assert_eq!(player.piece(piece_id).borrow_mut().position(), -1);
        assert_eq!(player.board().borrow_mut().outside(50).pieces.len(), 0);
        assert_eq!(player.board().borrow_mut().outside(0).pieces.len(), 0);
        assert_eq!(player.board().borrow_mut().home(0).pieces.len(), 4);
    }

    #[test]
    fn in_globe_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        let piece_id = 0;
        player.free_piece(piece_id);
        player.make_move(piece_id, 2, Act::Move);
        assert_eq!(player.piece(piece_id).borrow().position(), 2);
        assert!(!player.piece(piece_id).borrow().is_safe());
        assert!(!player.piece(piece_id).borrow().is_dangerous());

        player.make_move(piece_id, 6, Act::Safe);
        assert_eq!(player.piece(piece_id).borrow().position(), 8);
        assert!(player.piece(piece_id).borrow().is_safe());
        assert!(player.piece(piece_id).borrow().is_dangerous());
    }

    #[test]
    fn star_jump_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        let piece_id = 0;
        player.free_piece(piece_id);
        player.make_move(piece_id, 5, Act::Skip);
        assert_eq!(player.piece(piece_id).borrow().position(), 11);
        assert!(!player.piece(piece_id).borrow().is_safe());
        assert!(!player.piece(piece_id).borrow().is_dangerous());

        player.move_piece(piece_id, 1);
        player.skip(piece_id, 6);
        assert_eq!(player.piece(piece_id).borrow().position(), 24);
        assert!(!player.piece(piece_id).borrow().is_safe());
        assert!(!player.piece(piece_id).borrow().is_dangerous());
    }

    #[test]
    fn starjump_to_goal_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        let piece_id = 0;
        player.free_piece(piece_id);
        player.move_piece(piece_id, 44);
        let choice = player.valid_choices(piece_id, 6, Act::Goal);
        player.make_move(piece_id, 6, choice);
        assert_eq!(player.piece(piece_id).borrow().position(), 99);
        assert!(player.piece(piece_id).borrow().is_goal());
        assert_eq!(player.board().borrow_mut().goal(0).pieces.len(), 1);
        assert_eq!(
            player.board().borrow().goal[0].player_id,
            Some(board::PlayerID::Player0)
        );
    }
}

mod multipiece_test {
    use super::*;

    #[test]
    fn free_all_pieces_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        for piece_id in 0..4 {
            player.free_piece(piece_id);
            assert!(!player.piece(piece_id).borrow_mut().is_home());
            assert!(player.piece(piece_id).borrow_mut().is_dangerous());
            assert!(player.piece(piece_id).borrow_mut().is_safe());
            assert_eq!(player.piece(piece_id).borrow_mut().position(), 0);
        }
        assert_eq!(player.board().borrow_mut().home(0).pieces.len(), 0);
        assert_eq!(player.board().borrow_mut().outside(0).pieces.len(), 4);
    }

    #[test]
    fn joining_other_pieces_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));
        player.free_piece(0);
        player.free_piece(1);

        player.make_move(0, 6, Act::Move);
        player.make_move(1, 6, Act::Join);

        assert_eq!(player.piece(0).borrow_mut().position(), 6);
        assert_eq!(player.piece(1).borrow_mut().position(), 6);
        assert_eq!(player.board().borrow_mut().outside(6).pieces.len(), 2);
        assert!(player.piece(0).borrow_mut().is_dangerous());
        assert!(player.piece(1).borrow_mut().is_dangerous());
        assert!(player.piece(0).borrow_mut().is_safe());
        assert!(player.piece(1).borrow_mut().is_safe());
    }

    #[test]
    fn leaving_other_pieces_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));
        player.free_piece(0);
        player.free_piece(1);

        player.make_move(0, 6, Act::Move);
        player.make_move(1, 6, Act::Join);
        assert_eq!(player.piece(0).borrow_mut().position(), 6);
        assert_eq!(player.piece(1).borrow_mut().position(), 6);
        assert_eq!(player.board().borrow_mut().outside(6).pieces.len(), 2);
        assert!(player.piece(0).borrow_mut().is_dangerous());
        assert!(player.piece(1).borrow_mut().is_dangerous());

        player.make_move(0, 6, Act::Leave);
        assert_eq!(player.piece(0).borrow_mut().position(), 12);
        assert_eq!(player.piece(1).borrow_mut().position(), 6);
        assert_eq!(player.board().borrow_mut().outside(6).pieces.len(), 1);
        assert_eq!(player.board().borrow_mut().outside(12).pieces.len(), 1);
        assert!(!player.piece(0).borrow_mut().is_dangerous());
        assert!(!player.piece(1).borrow_mut().is_dangerous());
    }

    #[test]
    fn all_pieces_at_same_place_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        player.free_piece(0);
        player.free_piece(1);
        player.free_piece(2);
        player.free_piece(3);

        player.make_move(0, 6, Act::Move);
        player.make_move(1, 6, Act::Join);
        player.make_move(2, 6, Act::Join);
        player.make_move(3, 6, Act::Join);

        assert_eq!(player.piece(0).borrow().position(), 6);
        assert_eq!(player.piece(1).borrow_mut().position(), 6);
        assert_eq!(player.piece(2).borrow_mut().position(), 6);
        assert_eq!(player.piece(3).borrow_mut().position(), 6);

        assert!(player.piece(0).borrow().is_safe());
        assert!(player.piece(1).borrow_mut().is_safe());
        assert!(player.piece(2).borrow_mut().is_safe());
        assert!(player.piece(3).borrow_mut().is_safe());

        assert!(player.piece(0).borrow().is_dangerous());
        assert!(player.piece(1).borrow_mut().is_dangerous());
        assert!(player.piece(2).borrow_mut().is_dangerous());
        assert!(player.piece(3).borrow_mut().is_dangerous());

        player.make_move(0, 1, Act::Leave);
        assert_eq!(player.piece(0).borrow().position(), 7);
        assert!(!player.piece(0).borrow().is_safe());
        assert!(!player.piece(0).borrow().is_dangerous());

        assert!(player.piece(1).borrow_mut().is_safe());
        assert!(player.piece(2).borrow_mut().is_safe());
        assert!(player.piece(3).borrow_mut().is_safe());
        assert!(player.piece(1).borrow_mut().is_dangerous());
        assert!(player.piece(2).borrow_mut().is_dangerous());
        assert!(player.piece(3).borrow_mut().is_dangerous());

        player.make_move(1, 3, Act::Leave);
        assert_eq!(player.piece(1).borrow_mut().position(), 9);
        assert!(!player.piece(1).borrow_mut().is_safe());
        assert!(!player.piece(1).borrow_mut().is_dangerous());

        assert!(player.piece(2).borrow_mut().is_safe());
        assert!(player.piece(3).borrow_mut().is_safe());
        assert!(player.piece(2).borrow_mut().is_dangerous());
        assert!(player.piece(3).borrow_mut().is_dangerous());

        player.make_move(2, 4, Act::Leave);
        assert_eq!(player.piece(2).borrow_mut().position(), 10);
        assert!(!player.piece(2).borrow_mut().is_safe());
        assert!(!player.piece(2).borrow_mut().is_dangerous());
        assert!(!player.piece(3).borrow_mut().is_safe());
        assert!(!player.piece(3).borrow_mut().is_dangerous());
    }

    #[test]
    fn joining_choice_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        player.free_piece(0);
        player.free_piece(1);

        player.make_move(0, 6, Act::Move);
        let joining_choice = player.valid_choices(1, 6, Act::Join);
        assert_eq!(joining_choice, Act::Join);
    }

    #[test]
    fn all_pieces_in_goal_test_0() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        player.free_piece(0);
        player.free_piece(1);
        player.free_piece(2);
        player.free_piece(3);

        player.move_piece(0, 99);
        player.move_piece(1, 99);
        player.move_piece(2, 99);
        player.move_piece(3, 99);

        assert!(player.piece(0).borrow().is_goal());
        assert!(player.piece(1).borrow_mut().is_goal());
        assert!(player.piece(2).borrow_mut().is_goal());
        assert!(player.piece(3).borrow_mut().is_goal());

        assert!(player.is_finished());
    }

    #[test]
    fn all_pieces_in_goal_test_1() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        player.free_piece(0);
        player.free_piece(1);
        player.free_piece(2);
        player.free_piece(3);

        player.move_piece(0, 50);
        player.move_piece(1, 50);
        player.move_piece(2, 50);
        player.move_piece(3, 50);

        player.move_piece(0, 6);
        player.move_piece(1, 6);
        player.move_piece(2, 6);
        player.move_piece(3, 6);

        assert!(player.piece(0).borrow().is_goal());
        assert!(player.piece(1).borrow_mut().is_goal());
        assert!(player.piece(2).borrow_mut().is_goal());
        assert!(player.piece(3).borrow_mut().is_goal());

        assert!(player.is_finished());
    }

    #[test]
    fn all_pieces_in_goal_test_part_2() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        player.free_piece(0);
        player.free_piece(1);
        player.free_piece(2);
        player.free_piece(3);

        player.move_piece(0, 49);
        player.move_piece(1, 49);
        player.move_piece(2, 49);
        player.move_piece(3, 49);

        player.move_piece(0, 4);
        player.move_piece(1, 4);
        player.move_piece(2, 4);
        player.move_piece(3, 4);

        player.move_piece(0, 3);
        player.move_piece(1, 3);
        player.move_piece(2, 3);
        player.move_piece(3, 3);

        assert!(player.piece(0).borrow().is_goal());
        assert!(player.piece(1).borrow_mut().is_goal());
        assert!(player.piece(2).borrow_mut().is_goal());
        assert!(player.piece(3).borrow_mut().is_goal());

        assert!(player.is_finished());
    }

    #[test]
    fn all_pieces_in_goal_test_part_3() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        player.free_piece(0);
        player.free_piece(1);
        player.free_piece(2);
        player.free_piece(3);

        player.move_piece(0, 49);
        player.move_piece(1, 49);
        player.move_piece(2, 49);
        player.move_piece(3, 49);

        player.move_piece(0, 1);
        player.move_piece(1, 1);
        player.move_piece(2, 1);
        player.move_piece(3, 1);

        assert!(player.piece(0).borrow().is_goal());
        assert!(player.piece(1).borrow_mut().is_goal());
        assert!(player.piece(2).borrow_mut().is_goal());
        assert!(player.piece(3).borrow_mut().is_goal());

        assert!(player.is_finished());
    }

    #[test]
    fn star_join_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));
        player.free_piece(0);
        player.free_piece(1);

        player.make_move(0, 5, Act::Move);
        assert_eq!(player.piece(0).borrow_mut().position(), 11);
        assert!(!player.piece(0).borrow_mut().is_dangerous());
        assert!(!player.piece(0).borrow_mut().is_safe());

        let choice = player.valid_choices(1, 5, Act::Join);
        assert_eq!(choice, Act::Join);

        player.make_move(1, 5, Act::Join);
        assert_eq!(player.piece(1).borrow_mut().position(), 11);
        assert_eq!(player.board().borrow_mut().outside(11).pieces.len(), 2);
        assert_eq!(player.board().borrow_mut().outside(5).pieces.len(), 0);
        assert_eq!(player.board().borrow_mut().outside(5).player_id, None);
        assert_eq!(
            player
                .board()
                .borrow_mut()
                .outside(11)
                .piece(0)
                .borrow_mut()
                .position(),
            11
        );
        assert_eq!(
            player
                .board()
                .borrow_mut()
                .outside(11)
                .piece(1)
                .borrow_mut()
                .position(),
            11
        );
        assert!(player
            .board()
            .borrow_mut()
            .outside(11)
            .piece(0)
            .borrow_mut()
            .is_dangerous());
        assert!(player
            .board()
            .borrow_mut()
            .outside(11)
            .piece(1)
            .borrow_mut()
            .is_dangerous());
        assert!(player.piece(0).borrow_mut().is_dangerous());
        assert!(player.piece(1).borrow_mut().is_dangerous());
        assert!(player.piece(0).borrow_mut().is_safe());
        assert!(player.piece(1).borrow_mut().is_safe());
    }

    #[test]
    fn winning_choice_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        player.free_piece(0);
        player.move_piece(0, 50);
        let mut diceroll = 6;
        let choice = player.valid_choices(0, diceroll, Act::Goal);
        assert_eq!(choice, Act::Goal);

        player.free_piece(1);
        player.move_piece(1, 49);
        diceroll = 1;
        let choice = player.valid_choices(1, diceroll, Act::Goal);
        assert_eq!(choice, Act::Goal);

        player.free_piece(2);
        player.move_piece(2, 49);
        player.move_piece(2, 4);
        diceroll = 3;
        let choice = player.valid_choices(2, diceroll, Act::Goal);
        assert_eq!(choice, Act::Goal);
    }

    #[test]
    #[ignore = "Remake, will fail"]
    fn try_to_move_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        for i in 0..4 {
            player.free_piece(i);

            let action = player.try_to_move(i, 50);
            assert_eq!(action, Act::Move);
            player.make_move(i, 50, Act::Move);

            let action = player.try_to_move(i, 4);
            assert_eq!(action, Act::Move);
            player.make_move(i, 4, Act::Move);

            let action = player.try_to_move(i, 1);
            assert_eq!(action, Act::Move);
            player.make_move(i, 1, Act::Move);

            let action = player.try_to_move(i, 3);
            assert_eq!(action, Act::Move);
            player.make_move(i, 3, Act::Move);

            let action = player.try_to_move(i, 2);
            assert_eq!(action, Act::Goal);
            player.make_move(i, 2, Act::Goal);
        }
        assert!(player.is_finished());
    }

    #[test]
    fn try_to_join_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        for i in 0..4 {
            player.free_piece(i);
            let action = player.try_to_join(i, 50);
            if i > 0 {
                assert_eq!(action, Act::Join);
            } else {
                assert_eq!(action, Act::Nothing);
            }
            player.move_piece(i, 50);
        }
        let action = player.try_to_join(0, 5);
        assert_eq!(action, Act::Nothing);
        player.move_piece(0, 5);

        let action = player.try_to_join(1, 5);
        assert_eq!(action, Act::Nothing);
        player.move_piece(1, 5);
    }

    #[test]
    fn try_to_leave_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        for i in 0..4 {
            player.free_piece(i);
            let action = player.try_to_leave(i, 50 + i);
            assert_eq!(action, Act::Nothing);
            player.die(i);
        }
        for i in 0..4 {
            player.free_piece(i);
        }
        for i in 0..4 {
            let action = player.try_to_leave(i, 40 + i + 1);
            if i > 2 {
                assert_eq!(action, Act::Nothing);
            } else {
                assert_eq!(action, Act::Leave);
            }
            player.make_move(i, 40 + i + 1, Act::Leave);
            let action = player.try_to_leave(i, i * 2);
            assert_eq!(action, Act::Nothing);
            player.make_move(i, i * 2, Act::Move);
        }
    }

    #[test]
    fn try_to_kill_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board.clone(), Some(dice.clone()));
        let mut opponent = Player::new(1, board, Some(dice));

        opponent.free_piece(0);
        player.free_piece(0);
        let action = player.try_to_kill(0, 13);
        assert_eq!(action, Act::Nothing);
        opponent.make_move(0, 1, Act::Move);
        let action = player.try_to_kill(0, 14);
        assert_eq!(action, Act::Kill);
    }

    #[test]
    fn try_to_kill_2_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board.clone(), Some(dice.clone()));
        let mut opponent = Player::new(1, board, Some(dice));

        opponent.free_piece(0);
        opponent.make_move(0, 5, Act::Skip);
        assert_eq!(opponent.piece(0).borrow_mut().position(), 24);

        player.free_piece(0);
        player.move_piece(0, 17);
        let action = player.try_to_kill(0, 1);
        assert_eq!(action, Act::Kill);
        player.make_move(0, 1, Act::Kill);
        assert_eq!(player.piece(0).borrow_mut().position(), 24);
        assert_eq!(opponent.piece(0).borrow_mut().position(), -1);
    }

    #[test]
    fn try_to_kill_3_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board.clone(), Some(dice.clone()));
        let mut opponent = Player::new(1, board, Some(dice));

        opponent.free_piece(0);
        opponent.free_piece(1);
        opponent.make_move(0, 5, Act::Skip);
        opponent.make_move(1, 6, Act::Move);
        opponent.make_move(1, 5, Act::Skip);
        assert_eq!(opponent.piece(0).borrow_mut().position(), 24);
        assert_eq!(opponent.piece(1).borrow_mut().position(), 31);

        player.free_piece(0);
        player.move_piece(0, 23);
        let action = player.try_to_kill(0, 1);
        assert_eq!(action, Act::Kill);
        player.make_move(0, 1, Act::Kill);

        assert_eq!(player.piece(0).borrow_mut().position(), 31);
        assert_eq!(opponent.piece(0).borrow_mut().position(), -1);
        assert_eq!(opponent.piece(1).borrow_mut().position(), -1);
    }

    #[test]
    fn try_to_win_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        for i in 0..4 {
            player.free_piece(i);
            let action = player.try_to_win(i, 50);
            assert_eq!(action, Act::Nothing);
            player.make_move(i, 50, Act::Move);

            let action = player.try_to_win(i, 6);
            assert_eq!(action, Act::Goal);
            player.make_move(i, 6, Act::Goal);
        }
        assert!(player.is_finished());

        let action = player.try_to_win(0, 1);
        assert_eq!(action, Act::Nothing);
        player.make_move(0, 1, Act::Move);
    }

    #[test]
    fn try_to_skip_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        for i in 0..4 {
            player.free_piece(i);
            let action = player.try_to_skip(i, 2);
            assert_eq!(action, Act::Nothing);
            player.make_move(i, 2, action);

            let action = player.try_to_skip(i, 5);
            assert_eq!(action, Act::Skip);
            player.make_move(i, 6, action);
        }
    }

    #[test]
    #[ignore = "long test"]
    fn single_player_move_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));
        player.free_piece(0);
        player.free_piece(1);
        player.free_piece(2);
        player.free_piece(3);
        while !player.is_finished() {
            player.my_turn();
            let dice_number = player.roll_dice();
            println!("Dice: {}", dice_number);
            let (action, piece_id) = player.make_random_choice(dice_number, Act::Move);
            println!("Piece ID: {:?}, Action: {:?}", piece_id, action);
            player.make_move(piece_id, dice_number, action);
            println!(
                "Piece 0: {:?}\nPiece 1: {:?}\nPiece 2: {:?}\nPiece 3: {:?}\n\n",
                player.piece(0).borrow().position(),
                player.piece(1).borrow_mut().position(),
                player.piece(2).borrow_mut().position(),
                player.piece(3).borrow_mut().position()
            );
        }
    }

    #[test]
    #[ignore = "long test"]
    fn single_player_safe_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));
        player.free_piece(0);
        player.free_piece(1);
        player.free_piece(2);
        player.free_piece(3);
        while !player.is_finished() {
            player.my_turn();
            let dice_number = player.roll_dice();
            println!("Dice: {}", dice_number);
            let (mut action, mut piece_id) = player.make_random_choice(dice_number, Act::Safe);
            if action == Act::Nothing {
                (action, piece_id) = player.make_random_choice(dice_number, Act::Move);
            }
            println!("Piece ID: {:?}, Action: {:?}", piece_id, action);
            player.make_move(piece_id, dice_number, action);
            println!(
                "Piece 0: {:?}\nPiece 1: {:?}\nPiece 2: {:?}\nPiece 3: {:?}\n\n",
                player.piece(0).borrow().position(),
                player.piece(1).borrow_mut().position(),
                player.piece(2).borrow_mut().position(),
                player.piece(3).borrow_mut().position()
            );
        }
    }

    #[test]
    #[ignore = "long test"]
    fn single_player_join_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));
        player.free_piece(0);
        player.free_piece(1);
        player.free_piece(2);
        player.free_piece(3);
        while !player.is_finished() {
            player.my_turn();
            let dice_number = player.roll_dice();
            println!("Dice: {}", dice_number);
            let (mut action, mut piece_id) = player.make_random_choice(dice_number, Act::Join);
            if action == Act::Nothing {
                (action, piece_id) = player.make_random_choice(dice_number, Act::Move);
            }
            println!("Piece ID: {:?}, Action: {:?}", piece_id, action);
            player.make_move(piece_id, dice_number, action);
            println!(
                "Piece 0: {:?}\nPiece 1: {:?}\nPiece 2: {:?}\nPiece 3: {:?}\n\n",
                player.piece(0).borrow().position(),
                player.piece(1).borrow_mut().position(),
                player.piece(2).borrow_mut().position(),
                player.piece(3).borrow_mut().position()
            );
        }
    }

    #[test]
    #[ignore = "long test"]
    fn single_player_leave_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));
        player.free_piece(0);
        player.free_piece(1);
        player.free_piece(2);
        player.free_piece(3);
        while !player.is_finished() {
            player.my_turn();
            let dice_number = player.roll_dice();
            println!("Dice: {}", dice_number);
            let (mut action, mut piece_id) = player.make_random_choice(dice_number, Act::Leave);
            if action == Act::Nothing {
                (action, piece_id) = player.make_random_choice(dice_number, Act::Move);
            }
            println!("Piece ID: {:?}, Action: {:?}", piece_id, action);
            player.make_move(piece_id, dice_number, action);
            println!(
                "Piece 0: {:?}\nPiece 1: {:?}\nPiece 2: {:?}\nPiece 3: {:?}\n\n",
                player.piece(0).borrow().position(),
                player.piece(1).borrow_mut().position(),
                player.piece(2).borrow_mut().position(),
                player.piece(3).borrow_mut().position()
            );
        }
    }

    #[test]
    #[ignore = "long test"]
    fn single_player_skip_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));
        player.free_piece(0);
        player.free_piece(1);
        player.free_piece(2);
        player.free_piece(3);
        while !player.is_finished() {
            player.my_turn();
            let dice_number = player.roll_dice();
            println!("Dice: {}", dice_number);
            let (mut action, mut piece_id) = player.make_random_choice(dice_number, Act::Skip);
            if action == Act::Nothing {
                (action, piece_id) = player.make_random_choice(dice_number, Act::Move);
            }
            println!("Piece ID: {:?}, Action: {:?}", piece_id, action);
            player.make_move(piece_id, dice_number, action);
            println!(
                "Piece 0: {:?}\nPiece 1: {:?}\nPiece 2: {:?}\nPiece 3: {:?}\n\n",
                player.piece(0).borrow().position(),
                player.piece(1).borrow_mut().position(),
                player.piece(2).borrow_mut().position(),
                player.piece(3).borrow_mut().position()
            );
        }
    }

    #[test]
    #[ignore]
    fn single_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));

        
        let actions =    vec![Act::Move,
                Act::Free,
                Act::Kill,
                Act::Join,
                Act::Leave,
                Act::Die,
                Act::Goal,
                Act::Safe,
                Act::Skip,
                Act::Nothing];

        while !player.is_finished() {
            player.my_turn();
            player.random_play(actions.clone());
            println!(
                "Piece 0: {:?}\nPiece 1: {:?}\nPiece 2: {:?}\nPiece 3: {:?}\n\n",
                player.piece(0).borrow().position(),
                player.piece(1).borrow_mut().position(),
                player.piece(2).borrow_mut().position(),
                player.piece(3).borrow_mut().position()
            );
        }
        assert!(player.is_finished());
    }
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

        assert_eq!(player1.piece(0).borrow().position(), 0);
        assert_eq!(player2.piece(0).borrow().position(), 13);
        assert_eq!(player1.board().borrow().outside[0].pieces.len(), 1);
        assert_eq!(
            player1.board().borrow().outside[0].player_id,
            Some(board::PlayerID::Player0)
        );
        assert_eq!(player2.board().borrow_mut().outside(13).pieces.len(), 1);
        assert_eq!(
            player2
                .board()
                .borrow_mut()
                .outside(13)
                .piece(0)
                .borrow()
                .position(),
            13
        );
        assert_eq!(
            player1.board().borrow().outside[13].player_id,
            Some(board::PlayerID::Player1)
        );
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

        assert_eq!(player1.piece(0).borrow().position(), 0);
        assert_eq!(player2.piece(0).borrow().position(), 13);
        assert_eq!(player3.piece(0).borrow().position(), 26);
        assert_eq!(player4.piece(0).borrow().position(), 39);

        assert_eq!(player1.board().borrow().outside[0].pieces.len(), 1);
        assert_eq!(
            player1.board().borrow().outside[0].player_id,
            Some(board::PlayerID::Player0)
        );
        assert_eq!(player1.board().borrow().outside[13].pieces.len(), 1);
        assert_eq!(
            player1.board().borrow().outside[13].player_id,
            Some(board::PlayerID::Player1)
        );
        assert_eq!(player1.board().borrow().outside[26].pieces.len(), 1);
        assert_eq!(
            player1.board().borrow().outside[26].player_id,
            Some(board::PlayerID::Player2)
        );
        assert_eq!(player1.board().borrow().outside[39].pieces.len(), 1);
        assert_eq!(
            player1.board().borrow().outside[39].player_id,
            Some(board::PlayerID::Player3)
        );
    }

    #[test]
    fn two_players_move_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
        let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
        let mut player2 = Player::new(1, board, Some(dice));

        player1.free_piece(0);
        player2.free_piece(0);

        player1.move_piece(0, 6);
        player2.move_piece(0, 6);

        assert_eq!(player1.piece(0).borrow().position(), 6);
        assert_eq!(player2.piece(0).borrow().position(), 19);

        assert_eq!(player1.board().borrow().outside[0].pieces.len(), 0);
        assert_eq!(player1.board().borrow().outside[6].pieces.len(), 1);
        assert_eq!(
            player1.board().borrow().outside[6].player_id,
            Some(board::PlayerID::Player0)
        );
        assert_eq!(player2.board().borrow().outside[13].pieces.len(), 0);
        assert_eq!(player2.board().borrow().outside[19].pieces.len(), 1);
        assert_eq!(
            player2.board().borrow().outside[19].player_id,
            Some(board::PlayerID::Player1)
        );
    }

    #[test]
    fn all_players_move_test() {
        let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
        let mut player2 = Player::new(1, board.clone(), Some(dice.clone()));
        let mut player3 = Player::new(2, board.clone(), Some(dice.clone()));
        let mut player4 = Player::new(3, board, Some(dice));

        player1.free_piece(0);
        player2.free_piece(0);
        player3.free_piece(0);
        player4.free_piece(0);

        player1.move_piece(0, 6);
        player2.move_piece(0, 6);
        player3.move_piece(0, 6);
        player4.move_piece(0, 6);

        assert_eq!(player1.piece(0).borrow().position(), 6);
        assert_eq!(player2.piece(0).borrow().position(), 19);
        assert_eq!(player3.piece(0).borrow().position(), 32);
        assert_eq!(player4.piece(0).borrow().position(), 45);

        assert_eq!(player1.board().borrow_mut().outside[0].pieces.len(), 0);
        assert_eq!(player1.board().borrow_mut().outside[6].pieces.len(), 1);
        assert_eq!(
            player1.board().borrow_mut().outside[6].player_id,
            Some(board::PlayerID::Player0)
        );
        assert_eq!(player2.board().borrow_mut().outside[13].pieces.len(), 0);
        assert_eq!(player2.board().borrow_mut().outside[19].pieces.len(), 1);
        assert_eq!(
            player2.board().borrow_mut().outside[19].player_id,
            Some(board::PlayerID::Player1)
        );
        assert_eq!(player3.board().borrow_mut().outside[26].pieces.len(), 0);
        assert_eq!(player3.board().borrow_mut().outside[32].pieces.len(), 1);
        assert_eq!(
            player3.board().borrow_mut().outside[32].player_id,
            Some(board::PlayerID::Player2)
        );
        assert_eq!(player4.board().borrow_mut().outside[39].pieces.len(), 0);
        assert_eq!(player4.board().borrow_mut().outside[45].pieces.len(), 1);
        assert_eq!(
            player4.board().borrow().outside[45].player_id,
            Some(board::PlayerID::Player3)
        );
    }

    #[test]
    fn other_player_circumvent_player_1() {
        let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));

        let mut player2 = Player::new(1, board.clone(), Some(dice.clone()));
        player2.free_piece(0);
        player2.move_piece(0, 36);
        player2.move_piece(0, 2);
        assert_eq!(player2.piece(0).borrow().position(), 51);

        player2.free_piece(1);
        player2.move_piece(1, 36);
        player2.move_piece(1, 6);
        assert_eq!(player2.piece(1).borrow().position(), 3);

        let mut player3 = Player::new(2, board.clone(), Some(dice.clone()));
        player3.free_piece(0);
        player3.move_piece(0, 23);
        player3.move_piece(0, 2);
        assert_eq!(player3.piece(0).borrow().position(), 51);

        player3.free_piece(1);
        player3.move_piece(1, 23);
        player3.move_piece(1, 6);
        assert_eq!(player3.piece(1).borrow().position(), 3);

        let mut player4 = Player::new(3, board, Some(dice));
        player4.free_piece(0);
        player4.move_piece(0, 10);
        player4.move_piece(0, 2);
        assert_eq!(player4.piece(0).borrow().position(), 51);

        player4.free_piece(1);
        player4.move_piece(1, 10);
        player4.move_piece(1, 6);
        assert_eq!(player4.piece(1).borrow().position(), 3);
    }

    #[test]
    fn two_player_kill_test() {
        let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
        let mut player2 = Player::new(1, board, Some(dice));

        player1.free_piece(0);
        let diceroll1 = 3;
        let choice1 = player1.valid_choices(0, diceroll1, Act::Move);
        assert_eq!(choice1, Act::Move);
        player1.make_move(0, 17, choice1);
        assert_eq!(player1.piece(0).borrow().position(), 17);

        player2.free_piece(0);
        assert_eq!(player2.piece(0).borrow().position(), 13);

        let diceroll2 = 4;
        let choice2 = player2.valid_choices(0, diceroll2, Act::Kill);
        assert_eq!(choice2, Act::Kill);
        player2.make_move(0, diceroll2, choice2);

        assert_eq!(player1.piece(0).borrow().position(), -1);
        assert!(player1.piece(0).borrow().is_home());
        assert_eq!(
            player1.board().borrow_mut().outside(17).player_id,
            Some(board::PlayerID::Player1)
        );

        assert_eq!(player2.piece(0).borrow().position(), 17);
        assert_eq!(
            player1.board().borrow_mut().outside(17).player_id,
            Some(board::PlayerID::Player1)
        );
    }

    #[test]
    fn suicide_test() {
        let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
        let mut player2 = Player::new(1, board, Some(dice));

        player1.free_piece(0);
        player1.free_piece(1);
        player1.move_piece(0, 17);
        player1.move_piece(1, 17);

        player2.free_piece(0);
        assert_eq!(player2.piece(0).borrow().position(), 13);

        let diceroll2 = 4;
        let choice2 = player2.valid_choices(0, diceroll2, Act::Kill);
        assert_eq!(choice2, Act::Nothing);
        let choice2 = player2.valid_choices(0, diceroll2, Act::Die);
        assert_eq!(choice2, Act::Die);
        player2.make_move(0, diceroll2, choice2);

        assert_eq!(player1.piece(0).borrow().position(), 17);
        assert!(!player1.piece(0).borrow().is_home());
        assert_eq!(
            player1
                .board()
                .borrow_mut()
                .outside(17)
                .piece(0)
                .borrow_mut()
                .position(),
            17
        );
        assert!(!player1
            .board()
            .borrow_mut()
            .outside(17)
            .piece(0)
            .borrow_mut()
            .is_home());
        assert_eq!(
            player1.board().borrow_mut().outside(17).player_id,
            Some(board::PlayerID::Player0)
        );

        assert_eq!(player2.piece(0).borrow().position(), -1);
        assert!(player2.piece(0).borrow().is_home());

        assert_eq!(
            player2
                .board()
                .borrow_mut()
                .home(1)
                .piece(0)
                .borrow_mut()
                .position(),
            -1
        );
        assert!(player2
            .board()
            .borrow_mut()
            .home(1)
            .piece(0)
            .borrow_mut()
            .is_home());
        assert_eq!(player2.board().borrow_mut().home(1).pieces.len(), 4);
        assert_eq!(
            player2.board().borrow_mut().outside(17).player_id,
            Some(board::PlayerID::Player0)
        );
    }

    #[test]
    fn two_player_star_kill_tests() {
        let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
        let mut player2 = Player::new(1, board, Some(dice));
        let piece_0 = 0;
        let piece_1 = 1;

        player1.free_piece(piece_0);
        player1.free_piece(piece_1);
        let mut dice_number = 18;
        player1.move_piece(piece_0, dice_number);
        assert_eq!(player1.piece(piece_0).borrow().position(), 18);

        dice_number = 24;
        player1.move_piece(piece_1, dice_number);
        assert_eq!(player1.piece(piece_1).borrow().position(), 24);

        player2.free_piece(piece_0);
        dice_number = 5;
        let choice2 = player2.valid_choices(piece_0, dice_number, Act::Kill);
        assert_eq!(choice2, Act::Kill);
        player2.make_move(piece_0, dice_number, choice2);

        assert_eq!(player1.piece(piece_0).borrow().position(), -1);
        assert_eq!(player1.piece(piece_1).borrow().position(), -1);
        assert_eq!(player1.board().borrow_mut().outside(18).pieces.len(), 0);
        assert_eq!(player1.board().borrow_mut().outside(18).player_id, None);
        assert!(player1.piece(piece_0).borrow().is_home());
        assert!(player1.piece(piece_1).borrow().is_home());

        assert_eq!(player2.piece(0).borrow().position(), 24);
        assert_eq!(player2.board().borrow_mut().outside(24).pieces.len(), 1);
        assert_eq!(
            player2.board().borrow_mut().outside(24).player_id,
            Some(board::PlayerID::Player1)
        );
    }

    #[test]
    fn star_sucide_test() {
        let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
        let mut player2 = Player::new(1, board, Some(dice));
        let piece_0 = 0;
        let piece_1 = 1;

        player1.free_piece(piece_0);
        player1.free_piece(piece_1);
        let mut dice_number = 18;
        player1.move_piece(piece_0, dice_number);
        player1.move_piece(piece_1, dice_number);
        assert_eq!(player1.piece(piece_0).borrow().position(), 18);
        assert_eq!(player1.piece(piece_1).borrow().position(), 18);

        player2.free_piece(piece_0);
        dice_number = 5;

        let choice2 = player2.valid_choices(piece_0, dice_number, Act::Kill);
        assert_eq!(choice2, Act::Nothing);

        let choice2 = player2.valid_choices(piece_0, dice_number, Act::Die);
        assert_eq!(choice2, Act::Die);
        player2.make_move(piece_0, dice_number, choice2);

        assert_eq!(player1.piece(piece_0).borrow().position(), 18);
        assert_eq!(player1.piece(piece_1).borrow().position(), 18);
        assert_eq!(player1.board().borrow_mut().outside(18).pieces.len(), 2);
        assert_eq!(
            player1.board().borrow_mut().outside(18).player_id,
            Some(board::PlayerID::Player0)
        );

        assert!(player2.piece(piece_0).borrow().is_home());
        assert_eq!(player2.piece(piece_0).borrow().position(), -1);
        assert_eq!(player2.board().borrow_mut().home(1).pieces.len(), 4);
    }

    #[test]
    fn star_sucide_test_2() {
        let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
        let mut player2 = Player::new(1, board, Some(dice));
        let piece_0 = 0;
        let piece_1 = 1;

        player1.free_piece(piece_0);
        player1.free_piece(piece_1);
        let mut dice_number = 24;
        player1.move_piece(piece_0, dice_number);
        player1.move_piece(piece_1, dice_number);
        assert_eq!(player1.piece(piece_0).borrow().position(), 24);
        assert_eq!(player1.piece(piece_1).borrow().position(), 24);

        player2.free_piece(piece_0);
        dice_number = 5;

        let choice2 = player2.valid_choices(piece_0, dice_number, Act::Kill);
        assert_eq!(choice2, Act::Nothing);

        let choice2 = player2.valid_choices(piece_0, dice_number, Act::Die);
        assert_eq!(choice2, Act::Die);
        player2.make_move(piece_0, dice_number, choice2);

        assert_eq!(player1.piece(piece_0).borrow().position(), 24);
        assert_eq!(player1.piece(piece_1).borrow().position(), 24);
        assert_eq!(player1.board().borrow_mut().outside(18).pieces.len(), 0);
        assert_eq!(player1.board().borrow_mut().outside(24).pieces.len(), 2);
        assert_eq!(
            player1.board().borrow_mut().outside(24).player_id,
            Some(board::PlayerID::Player0)
        );
        assert!(player2.piece(piece_0).borrow().is_home());
        assert_eq!(player2.piece(piece_0).borrow().position(), -1);
        assert_eq!(player2.board().borrow_mut().home(1).pieces.len(), 4);
    }

    #[test]
    fn try_to_die_test() {
        let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
        let mut player2 = Player::new(1, board, Some(dice));

        let piece_0 = 0;
        let piece_1 = 1;
        let dice_number = 18;
        player1.free_piece(piece_0);
        player1.free_piece(piece_1);
        player1.move_piece(piece_0, dice_number);
        player1.move_piece(piece_1, dice_number);
        player2.free_piece(piece_0);

        let action = player2.try_to_die(piece_0, 5);
        assert_eq!(action, Act::Die);

        player1.die(piece_0);
        player1.die(piece_1);
        player1.free_piece(piece_0);
        player1.free_piece(piece_1);

        let dice_number = 17;
        player1.move_piece(piece_0, dice_number);
        player1.move_piece(piece_1, dice_number);
        let dice_number = 1;
        player1.move_piece(piece_0, dice_number);
        player1.move_piece(piece_1, dice_number);

        let action = player2.try_to_die(piece_0, 5);
        assert_eq!(action, Act::Die);

        player1.free_piece(2);
        player1.move_piece(2, 21);

        let action = player2.try_to_die(piece_0, 8);
        assert_eq!(action, Act::Die);
    }

    #[test]
    fn try_to_kill_test() {
        let dice: Rc<RefCell<Dice>> = Rc::new(RefCell::new(Dice::new()));
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player1 = Player::new(0, board.clone(), Some(dice.clone()));
        let mut player2 = Player::new(1, board, Some(dice));

        let piece_0 = 0;
        let piece_1 = 1;
        let dice_number = 18;
        player1.free_piece(piece_0);
        player1.free_piece(piece_1);
        player1.move_piece(piece_0, dice_number);
        player1.move_piece(piece_1, 15);
        player2.free_piece(piece_0);

        let action = player2.try_to_kill(piece_0, 5);
        assert_eq!(action, Act::Kill);

        let action = player2.try_to_kill(piece_0, 2);
        assert_eq!(action, Act::Kill);

        player1.die(piece_0);
        player1.free_piece(piece_0);

        let dice_number = 17;
        player1.move_piece(piece_0, dice_number);
        let dice_number = 1;
        player1.move_piece(piece_0, dice_number);

        let action = player2.try_to_kill(piece_0, 5);
        assert_eq!(action, Act::Kill);

        player1.free_piece(2);
        player1.move_piece(2, 21);

        let action = player2.try_to_kill(piece_0, 8);
        assert_eq!(action, Act::Nothing);
    }

    #[test]
    #[ignore = "long test"]
    fn first_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(0, board, Some(dice));
        let actions =    vec![Act::Move,
        Act::Free,
        Act::Kill,
        Act::Join,
        Act::Leave,
        Act::Die,
        Act::Goal,
        Act::Safe,
        Act::Skip,
        Act::Nothing];
        while !player.is_finished() {
            player.my_turn();
            player.random_play(actions.clone());
            player.print_pieces();
        }
        assert!(player.is_finished());
    }

    #[test]
    #[ignore = "long test"]
    fn second_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(1, board, Some(dice));

        let actions =    vec![Act::Move,
        Act::Free,
        Act::Kill,
        Act::Join,
        Act::Leave,
        Act::Die,
        Act::Goal,
        Act::Safe,
        Act::Skip,
        Act::Nothing];

        while !player.is_finished() {
            player.my_turn();
            player.random_play(actions.clone());
            player.print_pieces();
        }
        assert!(player.is_finished());
    }

    #[test]
    #[ignore = "long test"]
    fn third_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(2, board, Some(dice));
        let actions =    vec![Act::Move,
        Act::Free,
        Act::Kill,
        Act::Join,
        Act::Leave,
        Act::Die,
        Act::Goal,
        Act::Safe,
        Act::Skip,
        Act::Nothing];

        while !player.is_finished() {
            player.my_turn();
            player.random_play(actions.clone());
            player.print_pieces();
        }
        assert!(player.is_finished());
    }

    #[test]
    #[ignore = "long test"]
    fn fourth_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player = Player::new(3, board, Some(dice));
        let actions =    vec![Act::Move,
        Act::Free,
        Act::Kill,
        Act::Join,
        Act::Leave,
        Act::Die,
        Act::Goal,
        Act::Safe,
        Act::Skip,
        Act::Nothing];
        while !player.is_finished() {
            player.my_turn();
            player.random_play(actions.clone());
            player.print_pieces();
        }
        assert!(player.is_finished());
    }
    #[test]
    #[ignore = "long test"]
    fn all_single_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));

        let actions =    vec![Act::Move,
        Act::Free,
        Act::Kill,
        Act::Join,
        Act::Leave,
        Act::Die,
        Act::Goal,
        Act::Safe,
        Act::Skip,
        Act::Nothing];

        for i in 0..4 {
            let mut player = Player::new(i, board.clone(), Some(dice.clone()));

            while !player.is_finished() {
                player.my_turn();
                player.random_play(actions.clone());
                player.print_pieces();
            }
            assert!(player.is_finished());
        }
    }

    #[test]
    #[ignore = "long test"]
    fn two_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player0 = Player::new(0, board.clone(), Some(dice.clone()));
        let mut player1 = Player::new(1, board, Some(dice));
        let actions =    vec![Act::Move,
        Act::Free,
        Act::Kill,
        Act::Join,
        Act::Leave,
        Act::Die,
        Act::Goal,
        Act::Safe,
        Act::Skip,
        Act::Nothing];
        loop {
            player0.my_turn();
            player0.random_play(actions.clone());
            player0.print_pieces();
            if player0.is_finished() {
                println!("Player 0 wins");
                break;
            }
            player1.my_turn();
            player1.random_play(actions.clone());
            player1.print_pieces();
            if player1.is_finished() {
                println!("Player 1 wins");
                break;
            }
        }
        assert!(player0.is_finished() || player1.is_finished());
    }

    #[test]
    #[ignore = "super long test"]
    fn all_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let mut player0 = Player::new(0, board.clone(), Some(dice.clone()));
        let mut player1 = Player::new(1, board.clone(), Some(dice.clone()));
        let mut player2 = Player::new(2, board.clone(), Some(dice.clone()));
        let mut player3 = Player::new(3, board, Some(dice));
        let actions =    vec![Act::Move,
        Act::Free,
        Act::Kill,
        Act::Join,
        Act::Leave,
        Act::Die,
        Act::Goal,
        Act::Safe,
        Act::Skip,
        Act::Nothing];

        for _ in 0..10 {
            loop {
                player0.my_turn();
                player0.random_play(actions.clone());
                if player0.is_finished() {
                    println!("Player 0 wins");
                    break;
                }
                player1.my_turn();
                player1.random_play(actions.clone());
                if player1.is_finished() {
                    println!("Player 1 wins");
                    break;
                }

                player2.my_turn();
                player2.random_play(actions.clone());
                if player2.is_finished() {
                    println!("Player 2 wins");
                    break;
                }

                player3.my_turn();
                player3.random_play(actions.clone());
                if player3.is_finished() {
                    println!("Player 3 wins");
                    break;
                }
            }
            assert!(
                player0.is_finished()
                    || player1.is_finished()
                    || player2.is_finished()
                    || player3.is_finished()
            );
            player0.reset();
            player1.reset();
            player2.reset();
            player3.reset();
        }
    }
}

#[cfg(test)]
mod playstyle_tests {
    use super::*;

    #[test]
    fn aggressive_player_test()
    {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Rc::new(RefCell::new(Dice::new()));
        let aggressive_player = Player::new(0, board.clone(), Some(dice.clone()));
        let random_player = Player::new(2, board.clone(), Some(dice.clone()));


    }
}
