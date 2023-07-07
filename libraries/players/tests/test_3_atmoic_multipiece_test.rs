use board::Board;
use dice::Dice;
use players::{Act, Player};
use std::{cell::RefCell, rc::Rc};
use rand::seq::SliceRandom;

#[cfg(test)]
mod atomic_multipiece_test {
    use super::*;

    const PLAYER_ID: i8 = 0;

    #[test]
    fn free_all_pieces_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

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
        let mut player = Player::new(PLAYER_ID, board);

        player.free_piece(0);
        player.free_piece(1);

        player.update_outside(0, 0, 1);
        player.update_outside(1, 0, 1);

        assert_eq!(player.piece(0).borrow_mut().position(), 1);
        assert_eq!(player.piece(1).borrow_mut().position(), 1);
        assert_eq!(player.board().borrow_mut().outside(1).pieces.len(), 2);
        
        assert!(!player.piece(0).borrow_mut().is_dangerous());
        assert!(!player.piece(0).borrow_mut().is_safe());
        assert!(!player.piece(1).borrow_mut().is_dangerous());
        assert!(!player.piece(1).borrow_mut().is_safe());

        player.update_outside(1, 1, 0);
        player.join_piece(1, 1, 0);

    }

    // #[test]
    // fn leaving_other_pieces_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));
    //     player.free_piece(0);
    //     player.free_piece(1);

    //     player.make_move(0, 6, Act::Move);
    //     player.make_move(1, 6, Act::Join);
    //     assert_eq!(player.piece(0).borrow_mut().position(), 6);
    //     assert_eq!(player.piece(1).borrow_mut().position(), 6);
    //     assert_eq!(player.board().borrow_mut().outside(6).pieces.len(), 2);
    //     assert!(player.piece(0).borrow_mut().is_dangerous());
    //     assert!(player.piece(1).borrow_mut().is_dangerous());

    //     player.make_move(0, 6, Act::Leave);
    //     assert_eq!(player.piece(0).borrow_mut().position(), 12);
    //     assert_eq!(player.piece(1).borrow_mut().position(), 6);
    //     assert_eq!(player.board().borrow_mut().outside(6).pieces.len(), 1);
    //     assert_eq!(player.board().borrow_mut().outside(12).pieces.len(), 1);
    //     assert!(!player.piece(0).borrow_mut().is_dangerous());
    //     assert!(!player.piece(1).borrow_mut().is_dangerous());
    // }

    // #[test]
    // fn all_pieces_at_same_place_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));

    //     player.free_piece(0);
    //     player.free_piece(1);
    //     player.free_piece(2);
    //     player.free_piece(3);

    //     player.make_move(0, 6, Act::Move);
    //     player.make_move(1, 6, Act::Join);
    //     player.make_move(2, 6, Act::Join);
    //     player.make_move(3, 6, Act::Join);

    //     assert_eq!(player.piece(0).borrow().position(), 6);
    //     assert_eq!(player.piece(1).borrow_mut().position(), 6);
    //     assert_eq!(player.piece(2).borrow_mut().position(), 6);
    //     assert_eq!(player.piece(3).borrow_mut().position(), 6);

    //     assert!(player.piece(0).borrow().is_safe());
    //     assert!(player.piece(1).borrow_mut().is_safe());
    //     assert!(player.piece(2).borrow_mut().is_safe());
    //     assert!(player.piece(3).borrow_mut().is_safe());

    //     assert!(player.piece(0).borrow().is_dangerous());
    //     assert!(player.piece(1).borrow_mut().is_dangerous());
    //     assert!(player.piece(2).borrow_mut().is_dangerous());
    //     assert!(player.piece(3).borrow_mut().is_dangerous());

    //     player.make_move(0, 1, Act::Leave);
    //     assert_eq!(player.piece(0).borrow().position(), 7);
    //     assert!(!player.piece(0).borrow().is_safe());
    //     assert!(!player.piece(0).borrow().is_dangerous());

    //     assert!(player.piece(1).borrow_mut().is_safe());
    //     assert!(player.piece(2).borrow_mut().is_safe());
    //     assert!(player.piece(3).borrow_mut().is_safe());
    //     assert!(player.piece(1).borrow_mut().is_dangerous());
    //     assert!(player.piece(2).borrow_mut().is_dangerous());
    //     assert!(player.piece(3).borrow_mut().is_dangerous());

    //     player.make_move(1, 3, Act::Leave);
    //     assert_eq!(player.piece(1).borrow_mut().position(), 9);
    //     assert!(!player.piece(1).borrow_mut().is_safe());
    //     assert!(!player.piece(1).borrow_mut().is_dangerous());

    //     assert!(player.piece(2).borrow_mut().is_safe());
    //     assert!(player.piece(3).borrow_mut().is_safe());
    //     assert!(player.piece(2).borrow_mut().is_dangerous());
    //     assert!(player.piece(3).borrow_mut().is_dangerous());

    //     player.make_move(2, 4, Act::Leave);
    //     assert_eq!(player.piece(2).borrow_mut().position(), 10);
    //     assert!(!player.piece(2).borrow_mut().is_safe());
    //     assert!(!player.piece(2).borrow_mut().is_dangerous());
    //     assert!(!player.piece(3).borrow_mut().is_safe());
    //     assert!(!player.piece(3).borrow_mut().is_dangerous());
    // }

    // #[test]
    // fn joining_choice_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));

    //     player.free_piece(0);
    //     player.free_piece(1);

    //     player.make_move(0, 6, Act::Move);
    //     let joining_choice = player.valid_choices(1, 6, Act::Join);
    //     assert_eq!(joining_choice, Act::Join);
    // }

    // #[test]
    // fn all_pieces_in_goal_test_0() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));

    //     player.free_piece(0);
    //     player.free_piece(1);
    //     player.free_piece(2);
    //     player.free_piece(3);

    //     player.goal(0);
    //     player.goal(1);
    //     player.goal(2);
    //     player.goal(3);

    //     assert!(player.piece(0).borrow_mut().is_goal());
    //     assert!(player.piece(1).borrow_mut().is_goal());
    //     assert!(player.piece(2).borrow_mut().is_goal());
    //     assert!(player.piece(3).borrow_mut().is_goal());
    //     assert!(player
    //         .board()
    //         .borrow_mut()
    //         .goal(0)
    //         .piece(0)
    //         .borrow_mut()
    //         .is_goal());
    //     assert!(player
    //         .board()
    //         .borrow_mut()
    //         .goal(0)
    //         .piece(1)
    //         .borrow_mut()
    //         .is_goal());
    //     assert!(player
    //         .board()
    //         .borrow_mut()
    //         .goal(0)
    //         .piece(2)
    //         .borrow_mut()
    //         .is_goal());
    //     assert!(player
    //         .board()
    //         .borrow_mut()
    //         .goal(0)
    //         .piece(3)
    //         .borrow_mut()
    //         .is_goal());
    //     assert_eq!(player.board().borrow_mut().goal(0).pieces.len(), 4);
    //     assert!(player.is_finished());
    // }

    // #[test]
    // fn all_pieces_in_goal_test_1() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));

    //     player.free_piece(0);
    //     player.free_piece(1);
    //     player.free_piece(2);
    //     player.free_piece(3);

    //     player.move_piece(0, 50);
    //     player.move_piece(1, 50);
    //     player.move_piece(2, 50);
    //     player.move_piece(3, 50);

    //     let win_choice = player.valid_choices(0, 6, Act::Goal);

    //     player.make_move(0, 6, win_choice);
    //     player.make_move(1, 6, win_choice);
    //     player.make_move(2, 6, win_choice);
    //     player.make_move(3, 6, win_choice);

    //     assert!(player.piece(0).borrow().is_goal());
    //     assert!(player.piece(1).borrow_mut().is_goal());
    //     assert!(player.piece(2).borrow_mut().is_goal());
    //     assert!(player.piece(3).borrow_mut().is_goal());

    //     assert!(player.is_finished());
    // }

    // #[test]
    // fn all_pieces_in_goal_test_part_2() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));

    //     player.free_piece(0);
    //     player.free_piece(1);
    //     player.free_piece(2);
    //     player.free_piece(3);

    //     player.move_piece(0, 49);
    //     player.move_piece(1, 49);
    //     player.move_piece(2, 49);
    //     player.move_piece(3, 49);

    //     player.move_piece(0, 4);
    //     player.move_piece(1, 4);
    //     player.move_piece(2, 4);
    //     player.move_piece(3, 4);

    //     let win_choice = player.valid_choices(0, 3, Act::Goal);

    //     player.make_move(0, 3, win_choice);
    //     player.make_move(1, 3, win_choice);
    //     player.make_move(2, 3, win_choice);
    //     player.make_move(3, 3, win_choice);

    //     assert!(player.piece(0).borrow().is_goal());
    //     assert!(player.piece(1).borrow_mut().is_goal());
    //     assert!(player.piece(2).borrow_mut().is_goal());
    //     assert!(player.piece(3).borrow_mut().is_goal());

    //     assert!(player.is_finished());
    // }

    // #[test]
    // fn all_pieces_in_goal_test_part_3() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));

    //     player.free_piece(0);
    //     player.free_piece(1);
    //     player.free_piece(2);
    //     player.free_piece(3);

    //     player.move_piece(0, 49);
    //     player.move_piece(1, 49);
    //     player.move_piece(2, 49);
    //     player.move_piece(3, 49);

    //     let dice_number = 1;

    //     let win_choice = player.valid_choices(0, dice_number, Act::Goal);

    //     player.make_move(0, dice_number, win_choice);
    //     player.make_move(1, dice_number, win_choice);
    //     player.make_move(2, dice_number, win_choice);
    //     player.make_move(3, dice_number, win_choice);

    //     assert!(player.piece(0).borrow().is_goal());
    //     assert!(player.piece(1).borrow_mut().is_goal());
    //     assert!(player.piece(2).borrow_mut().is_goal());
    //     assert!(player.piece(3).borrow_mut().is_goal());

    //     assert!(player.is_finished());
    // }

    // #[test]
    // fn star_join_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));
    //     player.free_piece(0);
    //     player.free_piece(1);

    //     player.make_move(0, 5, Act::Skip);
    //     assert_eq!(player.piece(0).borrow_mut().position(), 11);
    //     assert!(!player.piece(0).borrow_mut().is_dangerous());
    //     assert!(!player.piece(0).borrow_mut().is_safe());

    //     let choice = player.valid_choices(1, 5, Act::Join);
    //     assert_eq!(choice, Act::Join);

    //     player.make_move(1, 5, Act::Join);
    //     assert_eq!(player.piece(1).borrow_mut().position(), 11);
    //     assert_eq!(player.board().borrow_mut().outside(11).pieces.len(), 2);
    //     assert_eq!(player.board().borrow_mut().outside(5).pieces.len(), 0);
    //     assert_eq!(player.board().borrow_mut().outside(5).player_id, None);
    //     assert_eq!(
    //         player
    //             .board()
    //             .borrow_mut()
    //             .outside(11)
    //             .piece(0)
    //             .borrow_mut()
    //             .position(),
    //         11
    //     );
    //     assert_eq!(
    //         player
    //             .board()
    //             .borrow_mut()
    //             .outside(11)
    //             .piece(1)
    //             .borrow_mut()
    //             .position(),
    //         11
    //     );
    //     assert!(player
    //         .board()
    //         .borrow_mut()
    //         .outside(11)
    //         .piece(0)
    //         .borrow_mut()
    //         .is_dangerous());
    //     assert!(player
    //         .board()
    //         .borrow_mut()
    //         .outside(11)
    //         .piece(1)
    //         .borrow_mut()
    //         .is_dangerous());
    //     assert!(player.piece(0).borrow_mut().is_dangerous());
    //     assert!(player.piece(1).borrow_mut().is_dangerous());
    //     assert!(player.piece(0).borrow_mut().is_safe());
    //     assert!(player.piece(1).borrow_mut().is_safe());
    // }

    // #[test]
    // fn winning_choice_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));

    //     player.free_piece(0);
    //     player.move_piece(0, 50);
    //     let mut diceroll = 6;
    //     let choice = player.valid_choices(0, diceroll, Act::Goal);
    //     assert_eq!(choice, Act::Goal);

    //     player.free_piece(1);
    //     player.move_piece(1, 49);
    //     diceroll = 1;
    //     let choice = player.valid_choices(1, diceroll, Act::Goal);
    //     assert_eq!(choice, Act::Goal);

    //     player.free_piece(2);
    //     player.move_piece(2, 49);
    //     player.move_piece(2, 4);
    //     diceroll = 3;
    //     let choice = player.valid_choices(2, diceroll, Act::Goal);
    //     assert_eq!(choice, Act::Goal);
    // }

    // #[test]
    // fn try_to_free_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board.clone(), Some(dice.clone()));
    //     let mut opponent = Player::new(1, board, Some(dice));

    //     let action = opponent.try_to_free();
    //     assert_eq!(action, Act::Free);
    //     opponent.free_piece(0);
    //     opponent.move_piece(0, 39);
    //     assert_eq!(opponent.piece(0).borrow().position(), 0);
    //     assert_eq!(opponent.board().borrow_mut().invincible(0).pieces.len(), 1);

    //     let action = player.try_to_free();
    //     assert_eq!(action, Act::Nothing);
    // }

    // #[test]
    // fn try_to_enter_goal_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));

    //     for i in 0..4 {
    //         player.free_piece(i);

    //         let action = player.try_to_move(i, 49);
    //         assert_eq!(action, Act::Move);
    //         player.make_move(i, 49, Act::Move);

    //         let action = player.valid_choices(i, 4, Act::Move);
    //         assert_eq!(action, Act::Move);
    //         player.make_move(i, 4, action);

    //         let action = player.valid_choices(i, 1, Act::Move);
    //         assert_eq!(action, Act::Move);
    //         player.make_move(i, 1, action);

    //         let action = player.valid_choices(i, 3, Act::Move);
    //         assert_eq!(action, Act::Move);
    //         player.make_move(i, 3, action);

    //         let action = player.valid_choices(i, 1, Act::Goal);
    //         assert_eq!(action, Act::Goal);
    //         player.make_move(i, 1, action);
    //     }
    //     assert!(player.is_finished());
    // }

    // #[test]
    // fn try_to_join_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));

    //     for i in 0..4 {
    //         player.free_piece(i);
    //         let action = player.try_to_join(i, 50);
    //         if i > 0 {
    //             assert_eq!(action, Act::Join);
    //         } else {
    //             assert_eq!(action, Act::Nothing);
    //         }
    //         player.move_piece(i, 50);
    //     }
    //     let action = player.try_to_join(0, 5);
    //     assert_eq!(action, Act::Nothing);
    //     player.move_piece(0, 5);

    //     let action = player.try_to_join(1, 5);
    //     assert_eq!(action, Act::Nothing);
    //     player.move_piece(1, 5);
    // }

    // #[test]
    // fn try_to_leave_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));

    //     for i in 0..4 {
    //         player.free_piece(i);
    //         let action = player.try_to_leave(i, 50 + i);
    //         assert_eq!(action, Act::Nothing);
    //         player.die(i);
    //     }
    //     for i in 0..4 {
    //         player.free_piece(i);
    //     }
    //     for i in 0..4 {
    //         let action = player.try_to_leave(i, 19 + i);
    //         if i > 2 || player.board().borrow_mut().is_globe(19 + i) {
    //             assert_eq!(action, Act::Nothing);
    //         } else {
    //             assert_eq!(action, Act::Leave);
    //         }
    //         player.make_move(i, i + 5, Act::Leave);
    //         let action = player.try_to_leave(i, i + 5);
    //         assert_eq!(action, Act::Nothing);
    //         player.make_move(i, i + 5, Act::Move);
    //     }
    // }

    // #[test]
    // fn try_to_kill_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board.clone(), Some(dice.clone()));
    //     let mut opponent = Player::new(1, board, Some(dice));

    //     opponent.free_piece(0);
    //     player.free_piece(0);
    //     let action = player.try_to_kill(0, 13);
    //     assert_eq!(action, Act::Nothing);
    //     opponent.make_move(0, 1, Act::Move);
    //     let action = player.try_to_kill(0, 14);
    //     assert_eq!(action, Act::Kill);
    // }

    // #[test]
    // fn try_to_kill_2_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board.clone(), Some(dice.clone()));
    //     let mut opponent = Player::new(1, board, Some(dice));

    //     opponent.free_piece(0);
    //     opponent.make_move(0, 5, Act::Skip);
    //     assert_eq!(opponent.piece(0).borrow_mut().position(), 24);

    //     player.free_piece(0);
    //     player.move_piece(0, 17);
    //     let action = player.try_to_kill(0, 1);
    //     assert_eq!(action, Act::Kill);
    //     player.make_move(0, 1, Act::Kill);
    //     assert_eq!(player.piece(0).borrow_mut().position(), 24);
    //     assert_eq!(opponent.piece(0).borrow_mut().position(), -1);
    // }

    // #[test]
    // fn try_to_kill_3_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board.clone(), Some(dice.clone()));
    //     let mut opponent = Player::new(1, board, Some(dice));

    //     opponent.free_piece(0);
    //     opponent.free_piece(1);
    //     opponent.make_move(0, 5, Act::Skip);
    //     opponent.make_move(1, 6, Act::Move);
    //     opponent.make_move(1, 5, Act::Skip);
    //     assert_eq!(opponent.piece(0).borrow_mut().position(), 24);
    //     assert_eq!(opponent.piece(1).borrow_mut().position(), 31);

    //     player.free_piece(0);
    //     player.move_piece(0, 23);
    //     let action = player.try_to_kill(0, 1);
    //     assert_eq!(action, Act::Kill);
    //     player.make_move(0, 1, Act::Kill);

    //     assert_eq!(player.piece(0).borrow_mut().position(), 31);
    //     assert_eq!(opponent.piece(0).borrow_mut().position(), -1);
    //     assert_eq!(opponent.piece(1).borrow_mut().position(), -1);
    // }

    // #[test]
    // fn try_to_kill_4_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board.clone(), Some(dice.clone()));
    //     let mut opponent = Player::new(1, board, Some(dice));

    //     opponent.free_piece(0);
    //     opponent.move_piece(0, 39);

    //     let action = player.valid_choices(0, 6, Act::Kill);

    //     assert_eq!(action, Act::Kill);
    //     player.make_move(0, 6, action);

    //     assert!(opponent.piece(0).borrow_mut().is_home());
    //     assert_eq!(opponent.piece(0).borrow_mut().position(), -1);
    //     let action = player.try_to_kill(0, 55);
    //     assert_eq!(action, Act::Nothing);
    // }

    // #[test]
    // fn try_to_win_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));

    //     for i in 0..4 {
    //         player.free_piece(i);
    //         let action = player.try_to_win(i, 50);
    //         assert_eq!(action, Act::Nothing);
    //         player.make_move(i, 50, Act::Move);

    //         let action = player.try_to_win(i, 6);
    //         assert_eq!(action, Act::Goal);
    //         player.make_move(i, 6, Act::Goal);
    //     }
    //     assert!(player.is_finished());

    //     let action = player.try_to_win(0, 1);
    //     assert_eq!(action, Act::Nothing);
    // }

    // #[test]
    // fn try_to_skip_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));

    //     for i in 0..4 {
    //         player.free_piece(i);
    //         let action = player.try_to_skip(i, 2);
    //         assert_eq!(action, Act::Nothing);
    //         player.make_move(i, 2, action);

    //         let action = player.try_to_skip(i, 5);
    //         assert_eq!(action, Act::Skip);
    //         player.make_move(i, 6, action);

    //         let action = player.try_to_skip(i, 50);
    //         assert_eq!(action, Act::Nothing);
    //     }
    // }

    // #[test]
    // #[ignore = "long test"]
    // fn single_player_move_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));
    //     player.free_piece(0);
    //     player.free_piece(1);
    //     player.free_piece(2);
    //     player.free_piece(3);
    //     let actions = vec![
    //         Act::Move,
    //         Act::Free,
    //         Act::Kill,
    //         Act::Join,
    //         Act::Leave,
    //         Act::Die,
    //         Act::Goal,
    //         Act::Safe,
    //         Act::Skip,
    //         Act::Nothing,
    //     ];
    //     while !player.is_finished() {
    //         player.my_turn();
    //         let dice_number = player.roll_dice();
    //         println!("Dice: {}", dice_number);

    //         let (mut action, mut piece_id) = player.make_random_choice(dice_number, Act::Move);

    //         while action == Act::Nothing {
    //             (action, piece_id) = player.make_random_choice(
    //                 dice_number,
    //                 *actions.choose(&mut rand::thread_rng()).unwrap(),
    //             );
    //         }

    //         player.make_move(piece_id, dice_number, action);
    //         println!("Piece ID: {:?}, Action: {:?}", piece_id, action);
    //         player.print_status();
    //     }
    // }

    // #[test]
    // #[ignore]

    // fn single_player_safe_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));
    //     player.free_piece(0);
    //     player.free_piece(1);
    //     player.free_piece(2);
    //     player.free_piece(3);

    //     let actions = vec![
    //         Act::Move,
    //         Act::Free,
    //         Act::Kill,
    //         Act::Join,
    //         Act::Leave,
    //         Act::Die,
    //         Act::Goal,
    //         Act::Safe,
    //         Act::Skip,
    //         Act::Nothing,
    //     ];

    //     while !player.is_finished() {
    //         player.my_turn();
    //         let dice_number = player.roll_dice();
    //         println!("Dice: {}", dice_number);
    //         let (mut action, mut piece_id) = player.make_random_choice(dice_number, Act::Safe);
    //         while action == Act::Nothing {
    //             (action, piece_id) = player.make_random_choice(
    //                 dice_number,
    //                 *actions.choose(&mut rand::thread_rng()).unwrap(),
    //             );
    //         }
    //         player.make_move(piece_id, dice_number, action);
    //         player.print_status();
    //     }
    // }

    // #[test]
    // #[ignore]

    // fn single_player_join_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));
    //     player.free_piece(0);
    //     player.free_piece(1);
    //     player.free_piece(2);
    //     player.free_piece(3);

    //     let actions = vec![
    //         Act::Move,
    //         Act::Free,
    //         Act::Kill,
    //         Act::Join,
    //         Act::Leave,
    //         Act::Die,
    //         Act::Goal,
    //         Act::Safe,
    //         Act::Skip,
    //         Act::Nothing,
    //     ];

    //     while !player.is_finished() {
    //         player.my_turn();
    //         let dice_number = player.roll_dice();
    //         println!("Dice: {}", dice_number);
    //         let (mut action, mut piece_id) = player.make_random_choice(dice_number, Act::Join);

    //         while action == Act::Nothing {
    //             (action, piece_id) = player.make_random_choice(
    //                 dice_number,
    //                 *actions.choose(&mut rand::thread_rng()).unwrap(),
    //             );
    //         }
    //         println!("Piece ID: {:?}, Action: {:?}", piece_id, action);
    //         player.make_move(piece_id, dice_number, action);
    //         println!(
    //             "Piece 0: {:?}\nPiece 1: {:?}\nPiece 2: {:?}\nPiece 3: {:?}\n\n",
    //             player.piece(0).borrow().position(),
    //             player.piece(1).borrow_mut().position(),
    //             player.piece(2).borrow_mut().position(),
    //             player.piece(3).borrow_mut().position()
    //         );
    //     }
    // }

    // #[test]
    // #[ignore]

    // fn single_player_leave_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));
    //     player.free_piece(0);
    //     player.free_piece(1);
    //     player.free_piece(2);
    //     player.free_piece(3);

    //     let actions = vec![
    //         Act::Move,
    //         Act::Free,
    //         Act::Kill,
    //         Act::Join,
    //         Act::Leave,
    //         Act::Die,
    //         Act::Goal,
    //         Act::Safe,
    //         Act::Skip,
    //         Act::Nothing,
    //     ];

    //     while !player.is_finished() {
    //         player.my_turn();
    //         let dice_number = player.roll_dice();
    //         println!("Dice: {}", dice_number);
    //         let (mut action, mut piece_id) = player.make_random_choice(dice_number, Act::Leave);
    //         while action == Act::Nothing {
    //             (action, piece_id) = player.make_random_choice(
    //                 dice_number,
    //                 *actions.choose(&mut rand::thread_rng()).unwrap(),
    //             );
    //         }
    //         println!("Piece ID: {:?}, Action: {:?}", piece_id, action);
    //         player.make_move(piece_id, dice_number, action);
    //         println!(
    //             "Piece 0: {:?}\nPiece 1: {:?}\nPiece 2: {:?}\nPiece 3: {:?}\n\n",
    //             player.piece(0).borrow().position(),
    //             player.piece(1).borrow_mut().position(),
    //             player.piece(2).borrow_mut().position(),
    //             player.piece(3).borrow_mut().position()
    //         );
    //     }
    // }

    // #[test]
    // #[ignore]
    // fn single_player_skip_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));
    //     player.free_piece(0);
    //     player.free_piece(1);
    //     player.free_piece(2);
    //     player.free_piece(3);

    //     let actions = vec![
    //         Act::Move,
    //         Act::Free,
    //         Act::Kill,
    //         Act::Join,
    //         Act::Leave,
    //         Act::Die,
    //         Act::Goal,
    //         Act::Safe,
    //         Act::Skip,
    //         Act::Nothing,
    //     ];

    //     while !player.is_finished() {
    //         player.my_turn();
    //         let dice_number = player.roll_dice();
    //         println!("Dice: {}", dice_number);
    //         let (mut action, mut piece_id) = player.make_random_choice(dice_number, Act::Skip);
    //         while action == Act::Nothing {
    //             (action, piece_id) = player.make_random_choice(
    //                 dice_number,
    //                 *actions.choose(&mut rand::thread_rng()).unwrap(),
    //             );
    //         }
    //         println!("Piece ID: {:?}, Action: {:?}", piece_id, action);
    //         player.make_move(piece_id, dice_number, action);
    //         println!(
    //             "Piece 0: {:?}\nPiece 1: {:?}\nPiece 2: {:?}\nPiece 3: {:?}\n\n",
    //             player.piece(0).borrow().position(),
    //             player.piece(1).borrow_mut().position(),
    //             player.piece(2).borrow_mut().position(),
    //             player.piece(3).borrow_mut().position()
    //         );
    //     }
    //     assert!(player.is_finished());
    // }

    // #[test]
    // #[ignore = "Long Test"]
    // fn single_player_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let dice = Rc::new(RefCell::new(Dice::new()));
    //     let mut player = Player::new(0, board, Some(dice));

    //     let actions = vec![
    //         Act::Move,
    //         Act::Free,
    //         Act::Kill,
    //         Act::Join,
    //         Act::Leave,
    //         Act::Die,
    //         Act::Goal,
    //         Act::Safe,
    //         Act::Skip,
    //         Act::Nothing,
    //     ];

    //     while !player.is_finished() {
    //         player.my_turn();
    //         player.random_play(actions.clone());
    //         player.print_status();
    //     }
    //     assert!(player.is_finished());
    // }
}
