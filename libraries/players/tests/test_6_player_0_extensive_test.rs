use board::Board;
use pieces::Color;
use dice::Dice;
use players::{Player, Act};
use std::{cell::RefCell, rc::Rc};

static PLAYER_ID: i8 = 0;


    
#[cfg(test)]
mod player_0_choice_tests {
    use super::*;

    #[test]
    fn generate_action_vector_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        let result_vec = player.generate_action_vector(0, Act::Free);

        assert_eq!(result_vec.len(), 0);

        let result_vec = player.generate_action_vector(6, Act::Free);
        assert_eq!(result_vec.len(), 4);
    }

    #[test]
    fn try_to_free_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board.clone());
        let mut other_player = Player::new(1, board);

        let result = player.try_to_free(0, 6);
        assert_eq!(result, Act::Free);
        
        for i in 0..4 {
            other_player.free_piece(i);
            other_player.update_outside(i, 13, 0);
        }

        let result = player.try_to_free(0, 6);
        assert_eq!(result, Act::Nothing);
    }
    
    #[test]
    fn try_to_move_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        let result = player.try_to_move(0, 1);
        assert_eq!(result, Act::Nothing);

        player.free_piece(0);
        let result = player.try_to_move(0, 6);
        assert_eq!(result, Act::Move);

        player.update_outside(0, 0, 7);
        let result = player.try_to_move(0, 1);
        assert_eq!(result, Act::Nothing);
        
        player.update_outside(0, 7, 12);
        let result = player.try_to_move(0, 1);
        assert_eq!(result, Act::Move);
        
        player.update_outside(0, 12, 50);
        let result = player.try_to_move(0, 1);
        assert_eq!(result, Act::Move);

        player.enter_inside(0, 50, 52);
        let result = player.try_to_move(0, 1);
        assert_eq!(result, Act::Move);

        let result = player.try_to_move(0, 5);
        assert_eq!(result, Act::Nothing);
    }

    #[test]
    fn try_to_move_test_2() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board.clone());
        let mut other_player = Player::new(1, board);

        player.free_piece(0);
        player.free_piece(1);
        let result = player.try_to_move(0, 1);
        assert_eq!(result, Act::Nothing);

        player.update_outside(0, 0, 7);
        
        other_player.free_piece(0);
        other_player.free_piece(1);
        other_player.update_outside(0, 13, 9);
        let result = player.try_to_move(0, 2);
        assert_eq!(result, Act::Nothing);

        other_player.update_outside(1
            , 13, 9);
        let result = player.try_to_move(0, 2);
        assert_eq!(result, Act::Nothing);
    }

    #[test]
    fn try_to_join_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        let result = player.try_to_join(0, 1);
        assert_eq!(result, Act::Nothing);

        player.free_piece(0);
        player.free_piece(1);
        player.free_piece(2);

        player.update_outside(0, 0, 1);
        let result = player.try_to_join(1, 1);
        assert_eq!(result, Act::Join);
        
        player.update_outside(0, 1, 5);
        let result = player.try_to_join(1, 5);
        assert_eq!(result, Act::Nothing);

        player.update_outside(0, 5, 8);
        let result = player.try_to_join(1, 3);
        assert_eq!(result, Act::Nothing);

        player.update_outside(0, 8, 11);
        let result = player.try_to_join(1, 5);
        assert_eq!(result, Act::Join);

        player.update_outside(0, 11, 13);
        let result = player.try_to_join(1, 13);
        assert_eq!(result, Act::Join);

        player.update_outside(0, 13, 50);
        player.enter_inside(0, 50, 55);
        player.update_outside(1, 0, 49);
        let result = player.try_to_join(1, 5);
        assert_eq!(result, Act::Nothing);
        
        player.enter_inside(1, 49, 52);
        let result = player.try_to_join(1, 3);
        assert_eq!(result, Act::Nothing);
    }
    
    #[test]
    fn try_to_join_test_2() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board.clone());
        let mut other_player = Player::new(1, board);

        let result = player.try_to_join(0, 1);
        assert_eq!(result, Act::Nothing);

        player.free_piece(0);
        player.free_piece(1);
        player.free_piece(2);
        other_player.free_piece(0);
        other_player.free_piece(1);

        other_player.update_outside(0, 13, 5);
        player.update_outside(0, 0, 11);

        let result = player.try_to_join(1, 5);
        assert_eq!(result, Act::Join);

        other_player.update_outside(1, 13, 5);

        let result = player.try_to_join(1, 5);
        assert_eq!(result, Act::Nothing);
    }

    #[test]
    fn try_to_kill_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board.clone());
        let mut other_player = Player::new(1, board);

        other_player.free_piece(0);
        other_player.free_piece(1);
        
        other_player.update_outside(0, 13, 1);
        let result = player.try_to_kill(0, 1);
        assert_eq!(result, Act::Nothing);

        player.free_piece(0);
        let result = player.try_to_kill(0, 1);
        assert_eq!(result, Act::Kill);

        let result = player.try_to_kill(0, 13);
        assert_eq!(result, Act::Nothing);

        other_player.update_outside(0, 1, 5);
        let result = player.try_to_kill(0, 5);
        assert_eq!(result, Act::Kill);

        other_player.enter_globe(0, 5, 8);
        let result = player.try_to_kill(0, 8);
        assert_eq!(result, Act::Nothing);

        other_player.update_outside(0, 8, 11);
        let result = player.try_to_kill(0, 5);
        assert_eq!(result, Act::Kill);

        other_player.update_outside(0, 11, 26);
        let result = player.try_to_kill(0, 26);
        assert_eq!(result, Act::Kill);
    }

    #[test]
    fn try_to_kill_test_2 () {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board.clone());
        let mut other_player = Player::new(1, board);

        other_player.free_piece(0);
        other_player.free_piece(1);
        other_player.update_outside(0, 13, 0);
        other_player.join(1, 13, 0);
        assert!(other_player.piece(0).borrow().is_not_safe());
        assert!(other_player.piece(1).borrow().is_not_safe());
        let result = player.try_to_kill(0, 6);
        assert_eq!(result, Act::Kill);

        other_player.update_outside(0, 0, 1);
        other_player.join(1, 0, 1);
        assert!(other_player.piece(0).borrow().is_dangerous());
        assert!(other_player.piece(1).borrow().is_dangerous());
        player.free_piece(0);
        let result = player.try_to_kill(0, 1);
        assert_eq!(result, Act::Nothing);

        player.update_outside(0, 0, 25);
        other_player.update_outside(0, 1, 26);
        other_player.join(1, 1, 26);
        assert!(other_player.piece(0).borrow().is_not_safe());
        assert!(other_player.piece(1).borrow().is_not_safe());
        let result = player.try_to_kill(0, 1);
        assert_eq!(result, Act::Nothing);


        other_player.enter_globe(0, 26, 34);
        other_player.enter_globe(1, 26, 34);
        assert!(other_player.piece(0).borrow().is_dangerous());
        assert!(other_player.piece(1).borrow().is_dangerous());
        let result = player.try_to_kill(0, 10);
        assert_eq!(result, Act::Nothing);

        player.update_outside(0, 25, 0);
        other_player.free_piece(2);
        other_player.update_outside(2, 13, 11);
        other_player.update_outside(0, 34, 5);
        other_player.join(1, 34, 5);
        let result = player.try_to_kill(0, 5);
        assert_eq!(result, Act::Nothing);

    }

    #[test]
    fn try_to_die_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board.clone());
        let mut other_player = Player::new(1, board.clone());

        let piece_id1 = 0;
        let piece_id2 = 1;

        player.free_piece(piece_id1);
        other_player.free_piece(piece_id1);
        other_player.free_piece(piece_id2);

        other_player.update_outside(piece_id1, 13, 1);
        other_player.update_outside(piece_id2, 13, 1);

        let other_piece_1 = other_player.piece(piece_id1);
        let other_piece_2 = other_player.piece(piece_id2);

        assert_eq!(other_piece_1.borrow().position(), 1);
        assert_eq!(other_piece_2.borrow().position(), 1);
        assert_eq!(board.borrow_mut().outside(1).pieces.len(), 2);

        let dice_number = 1;
        let result = player.try_to_die(piece_id1, dice_number);
        assert_eq!(result, Act::Die);
    }

    #[test]
    fn try_to_win_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board.clone());
        let piece_id = 0;
       
        player.free_piece(piece_id);
        let old_position = player.piece(piece_id).borrow().position();
        let new_position = 52;
        player.enter_inside(piece_id, old_position, new_position);

        let piece = player.piece(piece_id);
        assert_eq!(piece.borrow().position(), new_position);
        assert_eq!(board.borrow_mut().inside(new_position).pieces.len(), 1);

        let dice_number = 5;
        let result = player.try_to_win(piece_id, dice_number);
        assert_eq!(result, Act::Goal);
    }

    #[test]
    fn try_to_leave_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let piece_id1 = 0;
        let piece_id2 = 1;

        player.free_piece(piece_id1);
        player.free_piece(piece_id2);

        let dice_number = 1;
        player.move_piece(piece_id1, dice_number);
        player.move_piece(piece_id2, dice_number);
        
        let result = player.try_to_leave(piece_id1, dice_number);
        assert_eq!(result, Act::Leave);
        
        player.move_piece(piece_id1, dice_number);
        let result = player.try_to_leave(piece_id1, dice_number);
        assert_eq!(result, Act::Nothing);
    }

    #[test]
    fn try_to_safe_test(){
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let piece_id = 0;

        player.free_piece(piece_id);
        let dice_number = 7;
        player.move_piece(piece_id, dice_number);
        let result = player.try_to_safe(piece_id, dice_number);
        assert_eq!(result, Act::Nothing);
        let dice_number = 1;
        let result = player.try_to_safe(piece_id, dice_number);
        assert_eq!(result, Act::Safe);
    }

    #[test]
    fn try_to_starjump_test(){
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let piece_id = 0;

        player.free_piece(piece_id);
        let dice_number = 3;
        let result = player.try_to_starjump(piece_id, dice_number);
        assert_eq!(result, Act::Nothing);
        let dice_number = 5;
        let result = player.try_to_starjump(piece_id, dice_number);
        assert_eq!(result, Act::Starjump);
    }

    #[test]
    fn valid_choices_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let dice = Dice::default();
        let mut player = Player::new(0, board);

        player.take_dice(dice);

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

}

#[cfg(test)]
mod player_0_move_tests {
    use super::*;

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
            if (i + 1) == 57 {
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

    #[test]
    fn starjump_test () {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let piece_id = 0;

        player.free_piece(piece_id);
        player.starjump_piece(piece_id, 5);
        assert_eq!(player.piece(piece_id).borrow().position(), 11);
        assert_eq!(player.board().borrow_mut().outside(11).pieces.len(), 1);

        player.update_piece(piece_id, 11, 10);
        player.starjump_piece(piece_id, 1);
        assert_eq!(player.piece(piece_id).borrow().position(), 18);
        assert_eq!(player.board().borrow_mut().outside(18).pieces.len(), 1);
    }

    #[test]
    fn starjump_test_2 () {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let starvec = vec![5, 11, 18, 24, 31, 37, 44, 50];
        let piece_id = 0;

        player.free_piece(piece_id);
        player.starjump_piece(piece_id, 5);
        assert_eq!(player.piece(piece_id).borrow().position(), 11);
        assert_eq!(player.board().borrow_mut().outside(11).pieces.len(), 1);

        (1..7).for_each(|i| {
            
            player.update_piece(piece_id, starvec[i], starvec[i] - 1);
            player.starjump_piece(piece_id, 1);
            
            assert_eq!(player.piece(piece_id).borrow().position(), starvec[i + 1]);
            assert_eq!(player.board().borrow_mut().outside(starvec[i + 1]).pieces.len(), 1);
        });
    }

    #[test]
    fn starjump_test_3 () {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let starvec = vec![5, 11, 18, 24, 31, 37, 44, 50];

        for piece_id in 0..4 {
            player.free_piece(piece_id);
            player.starjump_piece(piece_id, 5);
            assert_eq!(player.piece(piece_id).borrow().position(), 11);

            (1..7).for_each(|i| {
                player.update_piece(piece_id, starvec[i], starvec[i] - 1);
                player.starjump_piece(piece_id, 1);
            
                assert_eq!(player.piece(piece_id).borrow().position(), starvec[i + 1]);
            });

            player.update_piece(piece_id, starvec[7], starvec[7] - 1);
            player.starjump_piece(piece_id, 1);
            assert_eq!(player.piece(piece_id).borrow().position(), 99);
            assert!(player.piece(piece_id).borrow().is_goal());
        }
        assert!(player.is_finished());
    }

    #[test]
    fn join_piece_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        player.free_piece(0); 
        player.free_piece(1);

        player.move_piece(0, 1);
        player.join_piece(1, 1);
        assert_eq!(player.piece(0).borrow().position(), 1);
        assert_eq!(player.piece(1).borrow().position(), 1);
        assert_eq!(player.board().borrow_mut().outside(1).pieces.len(), 2);
        assert!(player.piece(0).borrow().is_dangerous());
        assert!(player.piece(1).borrow().is_dangerous());
    }

    #[test]
    fn join_piece_by_starjump_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        player.free_piece(0); 
        player.free_piece(1);

        player.starjump_piece(0, 5);
        player.join_piece(1, 5);
        assert_eq!(player.piece(0).borrow().position(), 11);
        assert_eq!(player.piece(1).borrow().position(), 11);
        assert_eq!(player.board().borrow_mut().outside(11).pieces.len(), 2);
        assert!(player.piece(0).borrow().is_dangerous());
        assert!(player.piece(1).borrow().is_dangerous());

        player.update_outside(0, 11, 18);
        player.update_outside(1, 11, 16);
        assert!(!player.piece(0).borrow().is_dangerous());
        assert!(!player.piece(1).borrow().is_dangerous());

        player.join_piece(1, 2);

        assert!(!player.piece(0).borrow().is_dangerous());
        assert!(!player.piece(1).borrow().is_dangerous());
    }

    #[test]
    fn join_piece_by_globe_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        player.free_piece(0); 
        player.free_piece(1);
        player.update_outside(0, 0, 6);
        player.update_outside(1, 0, 7);

        player.move_piece(0, 2);
        player.join_piece(1, 1);
        
        assert!(player.piece(0).borrow().is_dangerous());
        assert!(player.piece(1).borrow().is_dangerous());
        assert_eq!(player.piece(0).borrow().position(), 8);
        assert_eq!(player.piece(1).borrow().position(), 8);
    }

    #[test]
    fn join_piece_by_invincible_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        player.free_piece(0); 
        player.free_piece(1);
        player.update_outside(0, 0, 11);
        player.update_outside(1, 0, 12);

        player.move_piece(0, 2);
        player.join_piece(1, 1);
        
        assert!(player.piece(0).borrow().is_not_safe());
        assert!(player.piece(1).borrow().is_not_safe());
        assert_eq!(player.piece(0).borrow().position(), 13);
        assert_eq!(player.piece(1).borrow().position(), 13);
    }

    #[test]
    fn leave_piece_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        player.free_piece(0); 
        player.free_piece(1);

        player.move_piece(0, 1);
        player.join_piece(1, 1);
        assert!(player.piece(0).borrow().is_dangerous());
        assert!(player.piece(1).borrow().is_dangerous());

        player.leave_piece(0, 1);
        assert!(player.piece(0).borrow().is_vulnerable());
        assert!(player.piece(1).borrow().is_vulnerable());
        assert!(!player.piece(0).borrow().is_dangerous());
        assert!(!player.piece(1).borrow().is_dangerous());

    }

    #[test]
    fn leave_piece_from_globe_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        player.free_piece(0); 
        player.free_piece(1);

        player.move_piece(0, 8);
        player.join_piece(1, 8);
        assert!(player.piece(0).borrow().is_dangerous());
        assert!(player.piece(1).borrow().is_dangerous());

        player.leave_piece(0, 1);
        assert!(!player.piece(0).borrow().is_dangerous());
        assert!(player.piece(0).borrow().is_vulnerable());
        assert!(player.piece(1).borrow().is_dangerous());
    }

    #[test]
    fn leave_piece_from_invincible_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        player.free_piece(0); 
        player.free_piece(1);

        player.move_piece(0, 13);
        player.join_piece(1, 13);
        assert!(player.piece(0).borrow().is_not_safe());
        assert!(player.piece(1).borrow().is_not_safe());

        player.leave_piece(0, 1);
        assert!(!player.piece(0).borrow().is_not_safe());
        assert!(!player.piece(1).borrow().is_not_safe());
        assert!(player.piece(0).borrow().is_vulnerable());
        assert!(player.piece(1).borrow().is_vulnerable());
    }

    #[test]
    fn leave_piece_to_globe_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        player.free_piece(0); 
        player.free_piece(1);

        player.move_piece(0, 6);
        player.join_piece(1, 6);
        assert!(player.piece(0).borrow().is_dangerous());
        assert!(player.piece(1).borrow().is_dangerous());

        player.leave_piece(0, 2);
        assert!(!player.piece(0).borrow().is_not_safe());
        assert!(!player.piece(1).borrow().is_not_safe());
        assert!(player.piece(0).borrow().is_vulnerable());
        assert!(player.piece(1).borrow().is_vulnerable());
    }

    #[test]
    fn leave_piece_to_starjump_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        player.free_piece(0); 
        player.free_piece(1);

        player.move_piece(0, 4);
        player.join_piece(1, 4);
        assert!(player.piece(0).borrow().is_dangerous());
        assert!(player.piece(1).borrow().is_dangerous());

        player.leave_piece(0, 1);
        assert!(player.piece(0).borrow().is_vulnerable());
        assert!(player.piece(1).borrow().is_vulnerable());
        assert_eq!(player.piece(0).borrow().position(), 11);
    }

    #[test]
    fn get_playerpiece_heuristics() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

            for piece_id in 0..4 {
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 57);

                player.free_piece(piece_id);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 56);

                player.move_piece(piece_id, 1);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 55);

                player.starjump_piece(piece_id, 4);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 45);

                player.move_piece(piece_id, 6);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 39);

                player.save_piece(piece_id, 4);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 35);

                player.starjump_piece(piece_id, 3);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 25);

                player.starjump_piece(piece_id, 6);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 12);

                player.win_piece(piece_id, 6);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 0);
            }
    }

    #[test]
    fn get_playerpiece_heuristics_2() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

            for piece_id in 0..4 {
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 57);

                player.free_piece(piece_id);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 56);

                player.move_piece(piece_id, 1);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 55);

                player.starjump_piece(piece_id, 4);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 45);

                player.move_piece(piece_id, 6);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 39);

                player.save_piece(piece_id, 4);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 35);

                player.move_piece(piece_id, 5);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 30);

                player.starjump_piece(piece_id, 5);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 19);

                player.move_piece(piece_id, 6);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 13);

                player.starjump_piece(piece_id, 1);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 6);

                player.win_piece(piece_id, 6);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 0);
            }
    }

    #[test]
    fn get_playerpiece_heuristics_3() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

            for piece_id in 0..4 {
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 57);

                player.free_piece(piece_id);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 56);

                player.move_piece(piece_id, 6);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 50);

                player.starjump_piece(piece_id, 5);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 38);

                player.starjump_piece(piece_id, 6);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 25);

                player.starjump_piece(piece_id, 6);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 12);

                player.save_piece(piece_id, 3);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 9);

                player.save_piece(piece_id, 4);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 5);

                player.win_piece(piece_id, 5);
                let result = player.get_heuristics(piece_id);
                assert_eq!(result, 0);
            }
    }
}