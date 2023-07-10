use board::Board;
use dice::Dice;
use players::{Act, Player};
use std::{cell::RefCell, rc::Rc};

#[cfg(test)]
mod atomic_choice_test {
    use super::*;
    const PLAYER_ID: i8 = 0;

    #[test]
    fn try_to_free_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        let piece_id = 0;
        for dice_number in 1..6 {
            let result = player.try_to_free(piece_id, dice_number);
            assert_eq!(result, Act::Nothing);
        }

        let dice_number = 6;
        let result = player.try_to_free(piece_id, dice_number);
        assert_eq!(result, Act::Free);

        player.free_piece(piece_id);
        for dice_number in 1..=6 {
            let result = player.try_to_free(piece_id, dice_number);
            assert_eq!(result, Act::Nothing);
        }
    }

    #[test]
    fn try_to_move_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        let piece_id = 0;
        let dice_number = 1;

        let result = player.try_to_move(piece_id, dice_number);
        assert_eq!(result, Act::Nothing);

        let dice_number = 6;
        let result = player.try_to_move(piece_id, dice_number);
        assert_eq!(result, Act::Nothing);

        player.free_piece(piece_id);
        for dice_number in 1..=6 {
            if dice_number == 5 {
                continue;
            }
            let result = player.try_to_move(piece_id, dice_number);
            assert_eq!(result, Act::Move);
        }
    }

    #[test]
    fn try_to_join_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        let piece_0 = 0;
        let piece_1 = 1;
        let dice_number = 1;

        let result = player.try_to_join(piece_0, dice_number);
        assert_eq!(result, Act::Nothing);

        player.free_piece(piece_0);
        player.free_piece(piece_1);

        player.move_piece(piece_0, dice_number);
        let result = player.try_to_join(piece_1, dice_number);
        assert_eq!(result, Act::Join);
    }
    
    #[test]
    fn try_to_kill_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board.clone());
        let mut other_player = Player::new(1, board.clone());

        let piece_id = 0;

        player.free_piece(piece_id);
        other_player.free_piece(piece_id);
        other_player.update_outside(piece_id, 13, 1);

        assert_eq!(other_player.piece(piece_id).borrow().position(), 1);
        let mut boardspace = board.borrow_mut().outside(1).clone();
        assert_eq!(boardspace.piece(piece_id).borrow().position(), 1);
        assert_eq!(boardspace.pieces.len(), 1);

        let result = player.try_to_kill(piece_id, 0);
        assert_eq!(result, Act::Nothing);

        let result = player.try_to_kill(piece_id, 1);
        assert_eq!(result, Act::Kill);
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
