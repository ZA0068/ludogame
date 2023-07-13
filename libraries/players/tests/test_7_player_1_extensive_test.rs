use board::Board;
use pieces::Color;
use dice::Dice;
use players::{Player, Act};
use std::{cell::RefCell, rc::Rc};

static PLAYER_ID: i8 = 1;
static OTHER_PLAYER_ID: i8 = 2;

#[cfg(test)]
mod player_1_choice_tests {
    use super::*;

    #[test]
    fn try_to_free_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board.clone());
        let mut other_player = Player::new(OTHER_PLAYER_ID, board);

        let result = player.try_to_free(0, 6);
        assert_eq!(result, Act::Free);
        
        for i in 0..4 {
            other_player.free_piece(i);
            other_player.update_outside(i, 26, 13);
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

        let result = player.try_to_move(0, 6);
        assert_eq!(result, Act::Nothing);

        player.free_piece(0);
        let result = player.try_to_move(0, 6);
        assert_eq!(result, Act::Move);

        player.update_outside(0, 13, 20);
        let result = player.try_to_move(0, 1);
        assert_eq!(result, Act::Nothing);
        
        player.update_outside(0, 20, 25);
        let result = player.try_to_move(0, 1);
        assert_eq!(result, Act::Move);
        
        player.update_outside(0, 25,11);
        let result = player.try_to_move(0, 1);
        assert_eq!(result, Act::Move);

        player.enter_inside(0, 11, 57);
        let result = player.try_to_move(0, 1);
        assert_eq!(result, Act::Move);

        let result = player.try_to_move(0, 6);
        assert_eq!(result, Act::Move);

        let result = player.try_to_move(0, 5);
        assert_eq!(result, Act::Nothing);
    }

    #[test]
    fn try_to_move_test_2() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board.clone());
        let mut other_player = Player::new(OTHER_PLAYER_ID, board);

        player.free_piece(0);
        player.free_piece(1);
        let result = player.try_to_move(0, 1);
        assert_eq!(result, Act::Nothing);

        player.update_outside(0, 13, 20);
        
        other_player.free_piece(0);
        other_player.free_piece(1);
        other_player.update_outside(0, 26, 22);
        let result = player.try_to_move(0, 2);
        assert_eq!(result, Act::Nothing);

        other_player.update_outside(1, 26, 22);
        let result = player.try_to_move(0, 2);
        assert_eq!(result, Act::Nothing);
    }

    #[test]
    fn try_to_join_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        let result = player.try_to_join(0, 1);
        assert_eq!(result, Act::Nothing);

        let result = player.try_to_join(0, 6);
        assert_eq!(result, Act::Nothing);

        player.free_piece(0);
        player.free_piece(1);
        player.free_piece(2);

        player.update_outside(0, 13, 14);
        let result = player.try_to_join(1, 1);
        assert_eq!(result, Act::Join);
        
        player.update_outside(0, 14, 18);
        let result = player.try_to_join(1, 5);
        assert_eq!(result, Act::Nothing);

        player.update_outside(0, 18, 21);
        let result = player.try_to_join(1, 3);
        assert_eq!(result, Act::Nothing);

        player.update_outside(0, 21, 24);
        let result = player.try_to_join(1, 5);
        assert_eq!(result, Act::Join);

        player.update_outside(0, 24, 26);
        let result = player.try_to_join(1, 13);
        assert_eq!(result, Act::Join);

        player.update_outside(0, 26, 11);
        player.update_outside(1, 13, 10);
        let result = player.try_to_join(1, 1);
        assert_eq!(result, Act::Nothing);

    player.enter_inside(0, 11, 60);
    let result = player.try_to_join(1, 5);
    assert_eq!(result, Act::Nothing);
        
        player.enter_inside(1, 10, 52+5);
        let result = player.try_to_join(1, 3);
        assert_eq!(result, Act::Nothing);
    }
    
    #[test]
    fn try_to_join_test_2() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board.clone());
        let mut other_player = Player::new(OTHER_PLAYER_ID, board);

        let result = player.try_to_join(0, 1);
        assert_eq!(result, Act::Nothing);

        player.free_piece(0);
        player.free_piece(1);
        player.free_piece(2);
        other_player.free_piece(0);
        other_player.free_piece(1);

        // other_player.update_outside(0, 13, 5);
        // player.update_outside(0, 0, 11);

        // let result = player.try_to_join(1, 5);
        // assert_eq!(result, Act::Nothing);

        // other_player.update_outside(1, 13, 5);

        // let result = player.try_to_join(1, 5);
        // assert_eq!(result, Act::Nothing);
    }

    // #[test]
    // fn try_to_kill_test() {
    //     let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
    //     let mut player = Player::new(PLAYER_ID, board.clone());
    //     let mut other_player = Player::new(OTHER_PLAYER_ID, board);

    //     let result = player.try_to_kill(0, 6);
    //     assert_eq!(result, Act::Nothing);

    //     other_player.free_piece(0);
    //     other_player.free_piece(1);
        
    //     other_player.update_outside(0, 13, 1);
    //     let result = player.try_to_kill(0, 1);
    //     assert_eq!(result, Act::Nothing);

    //     player.free_piece(0);
    //     let result = player.try_to_kill(0, 1);
    //     assert_eq!(result, Act::Kill);

    //     let result = player.try_to_kill(0, 13);
    //     assert_eq!(result, Act::Nothing);

    //     other_player.update_outside(0, 1, 5);
    //     let result = player.try_to_kill(0, 5);
    //     assert_eq!(result, Act::Kill);

    //     other_player.enter_globe(0, 5, 8);
    //     let result = player.try_to_kill(0, 8);
    //     assert_eq!(result, Act::Nothing);

    //     other_player.update_outside(0, 8, 11);
    //     let result = player.try_to_kill(0, 5);
    //     assert_eq!(result, Act::Kill);

    //     other_player.update_outside(0, 11, 26);
    //     let result = player.try_to_kill(0, 26);
    //     assert_eq!(result, Act::Kill);
    // }

    // #[test]
    // fn try_to_kill_test_2 () {
    //     let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
    //     let mut player = Player::new(PLAYER_ID, board.clone());
    //     let mut other_player = Player::new(OTHER_PLAYER_ID, board);

    //     other_player.free_piece(0);
    //     other_player.free_piece(1);
    //     other_player.update_outside(0, 13, 0);
    //     other_player.join(1, 13, 0);
    //     let result = player.try_to_kill(0, 6);
    //     assert_eq!(result, Act::Kill);

    //     other_player.update_outside(0, 0, 1);
    //     other_player.join(1, 0, 1);
    //     player.free_piece(0);
    //     let result = player.try_to_kill(0, 1);
    //     assert_eq!(result, Act::Nothing);

    //     player.update_outside(0, 0, 25);
    //     other_player.update_outside(0, 1, 26);
    //     other_player.join(1, 1, 26);
    //     let result = player.try_to_kill(0, 1);
    //     assert_eq!(result, Act::Nothing);


    //     other_player.enter_globe(0, 26, 34);
    //     other_player.enter_globe(1, 26, 34);
    //     let result = player.try_to_kill(0, 10);
    //     assert_eq!(result, Act::Nothing);

    //     player.update_outside(0, 25, 0);
    //     other_player.free_piece(2);
    //     other_player.update_outside(2, 13, 11);
    //     other_player.update_outside(0, 34, 5);

    //     other_player.join(1, 34, 5);
    //     let result = player.try_to_kill(0, 5);
    //     assert_eq!(result, Act::Nothing);


    //     other_player.join(1, 5, 11);
    //     let result = player.try_to_kill(0, 5);
    //     assert_eq!(result, Act::Kill);

    //     other_player.leave(2, 11, 12);
    //     let result = player.try_to_kill(0, 5);
    //     assert_eq!(result, Act::Kill);

    //     other_player.update_outside(0, 5, 1);
    //     other_player.join(1, 11, 1);
    //     let result = player.try_to_kill(0, 1);
    //     assert_eq!(result, Act::Nothing);

    //     other_player.leave(0, 1, 2);
    //     let result = player.try_to_kill(0, 1);
    //     assert_eq!(result, Act::Kill);
    // }

    // #[test]
    // fn try_to_die_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let mut player = Player::new(PLAYER_ID, board.clone());
    //     let mut other_player = Player::new(OTHER_PLAYER_ID, board);

    //     let result = player.try_to_die(0, 6);
    //     assert_eq!(result, Act::Nothing);
        
    //     player.free_piece(0);
    //     other_player.free_piece(0);
    //     other_player.free_piece(1);

    //     other_player.update_outside(0, 13, 1);
    //     other_player.join(1, 13, 1);
    //     let result = player.try_to_die(0, 1);
    //     assert_eq!(result, Act::Die);

    //     other_player.update_outside(0, 1, 5);
    //     let result = player.try_to_die(0, 5);
    //     assert_eq!(result, Act::Nothing);

    //     other_player.update_outside(1, 1, 5);
    //     let result = player.try_to_die(0, 5);
    //     assert_eq!(result, Act::Die);

    //     other_player.leave(0, 5, 8);
    //     let result = player.try_to_die(0, 5);
    //     assert_eq!(result, Act::Nothing);
        
    //     let result = player.try_to_die(0, 8);
    //     assert_eq!(result, Act::Die);

    //     other_player.update_outside(1, 5, 11);
    //     other_player.join(0, 8, 11);

    //     let result = player.try_to_die(0, 5);
    //     assert_eq!(result, Act::Die);

    //     other_player.free_piece(2);
    //     other_player.update_outside(2, 13, 5);
    //     let result = player.try_to_die(0, 5);
    //     assert_eq!(result, Act::Die);

    //     other_player.join(1, 11, 5);
    //     let result = player.try_to_die(0, 5);
    //     assert_eq!(result, Act::Die);

    //     other_player.leave(1, 5, 6);

    //     let result = player.try_to_die(0, 5);
    //     assert_eq!(result, Act::Nothing);


    //     other_player.update_outside(1, 6, 26);
    //     other_player.join(0, 11, 26);
    //     let result = player.try_to_die(0, 26);
    //     assert_eq!(result, Act::Die);


    //     other_player.leave(1, 26, 13);
    //     let result = player.try_to_die(0, 26);
    //     assert_eq!(result, Act::Nothing);

    //     let result = player.try_to_die(0, 13);
    //     assert_eq!(result, Act::Die);
    // }

    // #[test]
    // fn try_to_win_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let mut player = Player::new(PLAYER_ID, board.clone());
       
    //     let result = player.try_to_win(0, 6);
    //     assert_eq!(result, Act::Nothing);

    //     player.free_piece(0);
    //     let old_position = 0;
    //     let new_position = 52;
    //     player.enter_inside(0, old_position, new_position);

    //     let piece = player.piece(0);
    //     assert_eq!(piece.borrow().position(), new_position);
    //     assert_eq!(board.borrow_mut().inside(new_position).pieces.len(), 1);

    //     let dice_number = 5;
    //     let result = player.try_to_win(0, dice_number);
    //     assert_eq!(result, Act::Goal);
    // }

    // #[test]
    // fn try_to_win_test_2() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let mut player = Player::new(PLAYER_ID, board.clone());
    //     let mut other_player = Player::new(OTHER_PLAYER_ID, board);
       
    //     other_player.free_piece(0);
    //     other_player.free_piece(1);
    //     player.free_piece(0);
    //     player.free_piece(1);

    //     other_player.update_outside(0, 13, 50);
    //     player.update_outside(0, 0, 44);
    //     let result = player.try_to_win(0, 6);
    //     assert_eq!(result, Act::Goal);

    //     other_player.join(1, 13, 50);
    //     let result = player.try_to_win(0, 6);
    //     assert_eq!(result, Act::Nothing);
    // }

    // #[test]
    // fn try_to_leave_test() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let mut player = Player::new(PLAYER_ID, board);

    //     let result = player.try_to_leave(0, 6);
    //     assert_eq!(result, Act::Nothing);

    //     player.free_piece(0);
    //     player.free_piece(1);

    //     player.update_outside(0, 0, 1);
    //     player.update_outside(1, 0, 1);
        
    //     let result = player.try_to_leave(0, 1);
    //     assert_eq!(result, Act::Leave);
        
    //     player.update_outside(1, 1,2);
    //     let result = player.try_to_leave(0, 1);
    //     assert_eq!(result, Act::Nothing);

    //     player.free_piece(2);
    //     let result = player.try_to_leave(0, 2);
    //     assert_eq!(result, Act::Nothing);

    //     player.update_outside(2, 0, 2);
    //     let result = player.try_to_leave(0, 1);
    //     assert_eq!(result, Act::Nothing);

    //     player.update_outside(2, 2, 5);
    //     player.update_outside(1, 2, 0);
    //     player.update_outside(0, 1, 0);
    //     let result = player.try_to_leave(0, 5);
    //     assert_eq!(result, Act::Nothing);

    //     player.update_outside(2, 5, 8);
    //     let result = player.try_to_leave(0, 8);
    //     assert_eq!(result, Act::Nothing);

    //     player.update_outside(2, 8, 11);
    //     let result = player.try_to_leave(0, 5);
    //     assert_eq!(result, Act::Nothing);


    //     player.update_outside(0, 0, 11);
    //     let result = player.try_to_leave(0, 2);
    //     assert_eq!(result, Act::Leave);
    // }

    // #[test]
    // fn try_to_leave_test_2() {
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let mut player = Player::new(PLAYER_ID, board.clone());
    //     let mut other_player = Player::new(OTHER_PLAYER_ID, board);

    //     player.free_piece(0);
    //     player.free_piece(1);
    //     other_player.free_piece(0);
    //     other_player.free_piece(1);

    //     other_player.update_outside(1, 13, 1);
    //     let result = player.try_to_leave(0, 1);
    //     assert_eq!(result, Act::Nothing);
        
    //     other_player.update_outside(0, 13, 1);
    //     let result = player.try_to_leave(0, 1);
    //     assert_eq!(result, Act::Nothing);
    // }

    // #[test]
    // fn try_to_safe_test(){
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let mut player = Player::new(PLAYER_ID, board);

    //     let result = player.try_to_safe(0, 6);
    //     assert_eq!(result, Act::Nothing);

    //     player.free_piece(0);
    //     player.free_piece(1);

    //     player.move_piece(0, 6);
    //     let result = player.try_to_safe(0, 1);
    //     assert_eq!(result, Act::Nothing);
    //     let result = player.try_to_safe(0, 2);
    //     assert_eq!(result, Act::Safe);

    //     player.update_outside(0, 6, 12);
    //     let result = player.try_to_safe(0, 1);
    //     assert_eq!(result, Act::Nothing);

    //     player.update_outside(1,0, 13);
    //     let result = player.try_to_safe(0, 1);
    //     assert_eq!(result, Act::Nothing);
        
    //     player.update_outside(1,13, 14);
    //     let result = player.try_to_safe(0, 2);
    //     assert_eq!(result, Act::Nothing);

    //     player.update_outside(0, 12, 49);
    //     let result = player.try_to_safe(0, 2);
    //     assert_eq!(result, Act::Safe);
    // }

    // #[test]
    // fn try_to_safe_test_2(){
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let mut player = Player::new(PLAYER_ID, board.clone());
    //     let mut other_player = Player::new(OTHER_PLAYER_ID, board);

    //     player.free_piece(0);
    //     other_player.free_piece(0);

    //     other_player.update_outside(0, 13, 8);
    //     let result = player.try_to_safe(0, 8);
    //     assert_eq!(result, Act::Nothing);
    // }

    // #[test]
    // fn try_to_starjump_test(){
    //     let board = Rc::new(RefCell::new(Board::new()));
    //     let mut player = Player::new(PLAYER_ID, board.clone());
    //     let mut other_player = Player::new(OTHER_PLAYER_ID, board);
        
    //     let result = player.try_to_starjump(0, 6);
    //     assert_eq!(result, Act::Nothing);

    //     player.free_piece(0);
    //     player.free_piece(1);
    //     other_player.free_piece(0);
    //     other_player.free_piece(1);

    //     let result = player.try_to_starjump(0, 5);
    //     assert_eq!(result, Act::Starjump);

    //     other_player.update_outside(0, 13, 5);
    //     let result = player.try_to_starjump(0, 5);
    //     assert_eq!(result, Act::Nothing);

    //     other_player.update_outside(0, 5, 11);
    //     let result = player.try_to_starjump(0, 5);
    //     assert_eq!(result, Act::Nothing);

    //     player.update_outside(0, 0, 49);
    //     let result = player.try_to_starjump(0, 1);
    //     assert_eq!(result, Act::Starjump);
    // }
}

// #[cfg(test)]
// mod player_1_move_tests {
// use super::*;

// #[test]
// fn add_player_test() {
// let board = Rc::new(RefCell::new(Board::new()));
// let player = Player::new(PLAYER_ID, board.clone());
// assert_eq!(player.id(), 0);
// assert_eq!(player.board().as_ptr(), board.as_ptr());
// }

// #[test]
// fn get_pieces_test() {
// let board = Rc::new(RefCell::new(Board::new()));
// let player = Player::new(PLAYER_ID, board);
// (0..4).for_each(|i| {
//     let piece = player.piece(i);
//     assert_eq!(piece.borrow().id(), i);
//     assert_eq!(piece.borrow().color(), Color::Green);
//     assert_ne!(piece.borrow().color(), Color::Yellow);
//     assert_ne!(piece.borrow().color(), Color::Blue);
//     assert_ne!(piece.borrow().color(), Color::Red);
// });
// }

// #[test]
// #[should_panic]
// fn get_pieces_test_2() {
// let board = Rc::new(RefCell::new(Board::new()));
// let player = Player::new(PLAYER_ID, board);
// let piece = player.piece(0);
// assert_eq!(piece.borrow().id(), 0);
// assert_eq!(piece.borrow().color(), Color::Yellow);
// }

// #[test]
// #[should_panic]
// fn get_pieces_test_3() {
// let board = Rc::new(RefCell::new(Board::new()));
// let player = Player::new(PLAYER_ID, board);
// let piece = player.piece(0);
// assert_eq!(piece.borrow().id(), 0);
// assert_eq!(piece.borrow().color(), Color::Red);
// }

// #[test]
// #[should_panic]
// fn get_pieces_test_4() {
// let board = Rc::new(RefCell::new(Board::new()));
// let player = Player::new(PLAYER_ID, board);
// let piece = player.piece(0);
// assert_eq!(piece.borrow().id(), 0);
// assert_eq!(piece.borrow().color(), Color::Blue);
// }

// #[test]
// #[should_panic]
// fn get_pieces_test_5() {
// let board = Rc::new(RefCell::new(Board::new()));
// let player = Player::new(PLAYER_ID, board);
// let piece = player.piece(0);
// assert_eq!(piece.borrow().id(), 0);
// assert_ne!(piece.borrow().color(), Color::Green);
// }

// #[test]
// fn player_with_dice_test() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// let result = player.roll_dice();
// assert!(result == 0);

// let dice = Dice::default();

// player.take_dice(dice);
// let result = player.roll_dice();
// assert!(result > 0 && result < 7);

// player.give_dice();
// let result = player.roll_dice();
// assert!(result == 0);
// }

// #[test]
// fn move_piece_test() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);
// let piece_id = 0;
// player.free_piece(piece_id);
// for i in 0..50 {
//     player.move_piece(piece_id, 1);
//     assert_eq!(player.piece(piece_id).borrow().position(), i + 1);
//     assert_eq!(
//         player.board().borrow_mut().outside(i + 1).pieces.len(),
//         1
//     );
//     assert_eq!(
//         player.board().borrow_mut().outside(i + 1).player_id,
//         Some(board::PlayerID::Player0)
//     );
//     assert_eq!(player.board().borrow_mut().outside(i).pieces.len(), 0);
// }
// player.enter_inside(piece_id, 50, 52);
// let vec = (52..=57).chain((52..57).rev()).collect::<Vec<_>>();
// for i in 52..62 {
//     let oldpos = player.piece(piece_id).borrow().position();
//     if (i + 1) == 57 {
//         continue;
//     }

//     player.update_piece(piece_id, oldpos, i + 1);
//     assert_eq!(player.piece(piece_id).borrow().position(), vec[i as usize - 51]);
//     assert_eq!(
//         player.board().borrow_mut().inside(vec[i as usize - 51]).pieces.len(),
//         1
//     );
//     assert_eq!(
//         player.board().borrow_mut().inside(vec[i as usize - 51]).player_id,
//         Some(board::PlayerID::Player0)
//     );
//     assert_eq!(player.board().borrow_mut().inside(vec[i as usize - 52]).pieces.len(), 0);
// }
// }

// #[test]
// fn move_piece_test_2() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);
// let piece_id = 0;
// player.free_piece(piece_id);
// for i in 0..50 {
//     player.move_piece(piece_id, 1);
//     assert_eq!(player.piece(piece_id).borrow().position(), i + 1);
//     assert_eq!(
//         player.board().borrow_mut().outside(i + 1).pieces.len(),
//         1
//     );
//     assert_eq!(
//         player.board().borrow_mut().outside(i + 1).player_id,
//         Some(board::PlayerID::Player0)
//     );
//     assert_eq!(player.board().borrow_mut().outside(i).pieces.len(), 0);
// }
// player.enter_inside(piece_id, 50, 52);
// let vec = (52..=57).collect::<Vec<_>>();
// for i in 52..56 {
//     let oldpos = player.piece(piece_id).borrow().position();

//     player.update_piece(piece_id, oldpos, i + 1);
//     assert_eq!(player.piece(piece_id).borrow().position(), vec[i as usize - 51]);
//     assert_eq!(
//         player.board().borrow_mut().inside(vec[i as usize - 51]).pieces.len(),
//         1
//     );
//     assert_eq!(
//         player.board().borrow_mut().inside(vec[i as usize - 51]).player_id,
//         Some(board::PlayerID::Player0)
//     );
//     assert_eq!(player.board().borrow_mut().inside(vec[i as usize - 52]).pieces.len(), 0);
// }
// player.enter_goal(piece_id, 56);
// assert_eq!(player.piece(piece_id).borrow().position(), 99);
// }

// #[test]
// fn move_all_pieces_test() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// for piece_id in 0..4 {
//     player.free_piece(piece_id);
//     for i in 0..50 {
//         player.move_piece(piece_id, 1);
//         assert_eq!(player.piece(piece_id).borrow().position(), i + 1);
//         assert_eq!(
//             player.board().borrow_mut().outside(i + 1).pieces.len(),
//             1
//         );
//         assert_eq!(
//             player.board().borrow_mut().outside(i + 1).player_id,
//             Some(board::PlayerID::Player0)
//         );
//         assert_eq!(player.board().borrow_mut().outside(i).pieces.len(), 0);
//     }
//     player.move_piece(piece_id, 1);
//     let vec = (52..=57).chain((52..57).rev()).collect::<Vec<_>>();
//     for i in 52..62 {
//         let oldpos = player.piece(piece_id).borrow().position();
//         if (i + 1) % 57 == 0 {
//             continue;
//         }

//         player.update_piece(piece_id, oldpos, i + 1);
//         assert_eq!(player.piece(piece_id).borrow().position(), vec[i as usize - 51]);

//         assert_eq!(
//             player.board().borrow_mut().inside(vec[i as usize - 51]).player_id,
//             Some(board::PlayerID::Player0)
//         );
//     }
// }   
// assert_eq!(player.board().borrow_mut().inside(52).pieces.len(), 4);
// }

// #[test]
// fn move_all_pieces_test_2() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// for piece_id in 0..4 {
//     player.free_piece(piece_id);
//     for i in 0..50 {
//         player.move_piece(piece_id, 1);
//         assert_eq!(player.piece(piece_id).borrow().position(), i + 1);
//         assert_eq!(
//             player.board().borrow_mut().outside(i + 1).pieces.len(),
//             1
//         );
//         assert_eq!(
//             player.board().borrow_mut().outside(i + 1).player_id,
//             Some(board::PlayerID::Player0)
//         );
//         assert_eq!(player.board().borrow_mut().outside(i).pieces.len(), 0);
//     }
//     player.move_piece(piece_id, 1);
//     let vec = (52..=57).collect::<Vec<_>>();
//     for i in 52..56 {

//         player.move_piece(piece_id, 1);
//         assert_eq!(player.piece(piece_id).borrow().position(), vec[i as usize - 51]);
//         assert_eq!(
//             player.board().borrow_mut().inside(vec[i as usize - 51]).player_id,
//             Some(board::PlayerID::Player0)
//         );
//     }
//     player.enter_goal(piece_id, 56);
//     assert_eq!(player.piece(piece_id).borrow().position(), 99);
// }
// assert!(player.is_finished());
// }

// #[test]
// fn safety_test() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);
// let piece_id = 0;

// player.free_piece(piece_id);
// player.move_piece(piece_id, 7);
// player.save_piece(piece_id, 1);


// assert_eq!(player.piece(piece_id).borrow().position(), 8);

// player.update_piece(piece_id, 8, 50);
// player.save_piece(piece_id, 1);
// assert_eq!(player.piece(piece_id).borrow().position(), 52);
// }

// #[test]
// fn safety_test_2() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// for piece_id in 0..4 {
//     player.free_piece(piece_id);
//     player.move_piece(piece_id, 7);
//     player.save_piece(piece_id, 1);


//     assert_eq!(player.piece(piece_id).borrow().position(), 8);

//     player.update_piece(piece_id, 8, 50);
//     player.save_piece(piece_id, 1);
//     assert_eq!(player.piece(piece_id).borrow().position(), 52);
// }
// }

// #[test]
// fn safety_test_3() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);
// for piece_id in 0..4 {
//     player.free_piece(piece_id);
//     for i in 1..=56 {
//         if i == 50 {
//             continue;
//         }
//         let is_globe = player.board().borrow().is_globe(i);
//         if is_globe {
//             player.save_piece(piece_id, 1);
    
//         } else {
//             player.move_piece(piece_id, 1);
//         }
//     }
// }
// }

// #[test]
// fn starjump_to_goal_test() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board.clone());
// let mut other_player = Player::new(OTHER_PLAYER_ID, board);

// let piece_id = 0;
// player.free_piece(piece_id);
// other_player.free_piece(piece_id);

// player.move_piece(piece_id, 44);
// other_player.update_outside(piece_id, 13, 50);

// assert_eq!(player.piece(piece_id).borrow().position(), 44);

// player.win_piece(piece_id, 6);
// assert_eq!(player.piece(piece_id).borrow().position(), 99);
// assert!(player.piece(piece_id).borrow().is_goal());
// assert_eq!(player.board().borrow_mut().goal(PLAYER_ID).pieces.len(), 1);

// assert!(other_player.piece(piece_id).borrow().is_home());
// assert_eq!(other_player.piece(piece_id).borrow().position(), -1);
// }

// #[test]
// fn starjump_to_goal_test_2() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// for piece_id in 0..4 {

//     player.free_piece(piece_id);
//     player.move_piece(piece_id, 44);
    
//     assert_eq!(player.piece(piece_id).borrow().position(), 44);
    
//     player.win_piece(piece_id, 6);
//     assert_eq!(player.piece(piece_id).borrow().position(), 99);
//     assert!(player.piece(piece_id).borrow().is_goal());
//     assert_eq!(player.board().borrow_mut().goal(PLAYER_ID).pieces.len(), piece_id as usize + 1);
// }
// assert!(player.is_finished());
// }

// #[test]
// fn starjump_to_goal_test_3() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// for piece_id in 0..4 {
//     player.free_piece(piece_id);
//     for position in 1..=50 {
//         if position < 50 {
//             player.move_piece(piece_id, 1);
//             assert_eq!(player.piece(piece_id).borrow().position(), position);
//         } else {
//             player.win_piece(piece_id, 1);
//             assert_eq!(player.piece(piece_id).borrow().position(), 99);
//             assert!(player.piece(piece_id).borrow().is_goal());
//         }
//     }
//     assert_eq!(player.board().borrow_mut().goal(PLAYER_ID).pieces.len(), piece_id as usize + 1);
// }
// assert!(player.is_finished());
// }

// #[test]
// fn enter_goal_test() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// let piece_id = 0;
// player.free_piece(piece_id);
// player.move_piece(piece_id, 50);
// player.win_piece(piece_id, 6);
// assert_eq!(player.piece(piece_id).borrow().position(), 99);
// assert!(player.piece(piece_id).borrow().is_goal());
// assert_eq!(player.board().borrow_mut().goal(PLAYER_ID).pieces.len(), 1);
// }

// #[test]
// fn enter_goal_test_2() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// for piece_id in 0..4 {
//     player.free_piece(piece_id);
//     player.move_piece(piece_id, 50);
//     player.win_piece(piece_id, 6);
//     assert_eq!(player.piece(piece_id).borrow().position(), 99);
//     assert!(player.piece(piece_id).borrow().is_goal());
//     assert_eq!(player.board().borrow_mut().goal(PLAYER_ID).pieces.len(), piece_id as usize + 1);
// }
// assert!(player.is_finished());
// }

// #[test]
// fn enter_goal_test_3() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board.clone());

// for dice_number in (1..=6).rev() {
//     for piece_id in 0..4 {
//         player.free_piece(piece_id);
//         player.move_piece(piece_id, 50);
//         player.move_piece(piece_id, 6 - dice_number);
//         player.win_piece(piece_id, dice_number);
//         assert_eq!(player.piece(piece_id).borrow().position(), 99);
//         assert!(player.piece(piece_id).borrow().is_goal());
//         assert_eq!(player.board().borrow_mut().goal(PLAYER_ID).pieces.len(), piece_id as usize + 1);
//     }
//     assert!(player.is_finished());
//     board.borrow_mut().reset();
// }
// }

// #[test]
// fn starjump_test () {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);
// let piece_id = 0;

// player.free_piece(piece_id);
// player.starjump_piece(piece_id, 5);
// assert_eq!(player.piece(piece_id).borrow().position(), 11);
// assert_eq!(player.board().borrow_mut().outside(11).pieces.len(), 1);

// player.update_piece(piece_id, 11, 10);
// player.starjump_piece(piece_id, 1);
// assert_eq!(player.piece(piece_id).borrow().position(), 18);
// assert_eq!(player.board().borrow_mut().outside(18).pieces.len(), 1);
// }

// #[test]
// fn starjump_test_2 () {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);
// let starvec = vec![5, 11, 18, 24, 31, 37, 44, 50];
// let piece_id = 0;

// player.free_piece(piece_id);
// player.starjump_piece(piece_id, 5);
// assert_eq!(player.piece(piece_id).borrow().position(), 11);
// assert_eq!(player.board().borrow_mut().outside(11).pieces.len(), 1);

// (1..7).for_each(|i| {
    
//     player.update_piece(piece_id, starvec[i], starvec[i] - 1);
//     player.starjump_piece(piece_id, 1);
    
//     assert_eq!(player.piece(piece_id).borrow().position(), starvec[i + 1]);
//     assert_eq!(player.board().borrow_mut().outside(starvec[i + 1]).pieces.len(), 1);
// });
// }

// #[test]
// fn starjump_test_3 () {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);
// let starvec = vec![5, 11, 18, 24, 31, 37, 44, 50];

// for piece_id in 0..4 {
//     player.free_piece(piece_id);
//     player.starjump_piece(piece_id, 5);
//     assert_eq!(player.piece(piece_id).borrow().position(), 11);

//     (1..7).for_each(|i| {
//         player.update_piece(piece_id, starvec[i], starvec[i] - 1);
//         player.starjump_piece(piece_id, 1);
    
//         assert_eq!(player.piece(piece_id).borrow().position(), starvec[i + 1]);
//     });

//     player.update_piece(piece_id, starvec[7], starvec[7] - 1);
//     player.starjump_piece(piece_id, 1);
//     assert_eq!(player.piece(piece_id).borrow().position(), 99);
//     assert!(player.piece(piece_id).borrow().is_goal());
// }
// assert!(player.is_finished());
// }

// #[test]
// fn join_piece_test() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// player.free_piece(0); 
// player.free_piece(1);

// player.move_piece(0, 1);
// player.join_piece(1, 1);
// assert_eq!(player.piece(0).borrow().position(), 1);
// assert_eq!(player.piece(1).borrow().position(), 1);
// assert_eq!(player.board().borrow_mut().outside(1).pieces.len(), 2);
// }


// #[test]
// fn join_piece_by_starjump_test() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// player.free_piece(0); 
// player.free_piece(1);

// player.starjump_piece(0, 5);
// player.join_piece(1, 5);
// assert_eq!(player.piece(0).borrow().position(), 11);
// assert_eq!(player.piece(1).borrow().position(), 11);
// assert_eq!(player.board().borrow_mut().outside(11).pieces.len(), 2);

// player.update_outside(0, 11, 18);
// player.update_outside(1, 11, 16);

// player.join_piece(1, 2);
// }

// #[test]
// fn join_piece_by_globe_test() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// player.free_piece(0); 
// player.free_piece(1);
// player.update_outside(0, 0, 6);
// player.update_outside(1, 0, 7);

// player.move_piece(0, 2);
// player.join_piece(1, 1);

// assert_eq!(player.piece(0).borrow().position(), 8);
// assert_eq!(player.piece(1).borrow().position(), 8);
// }

// #[test]
// fn join_piece_by_invincible_test() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// player.free_piece(0); 
// player.free_piece(1);
// player.update_outside(0, 0, 11);
// player.update_outside(1, 0, 12);

// player.move_piece(0, 2);
// player.join_piece(1, 1);

// assert_eq!(player.piece(0).borrow().position(), 13);
// assert_eq!(player.piece(1).borrow().position(), 13);
// }

// #[test]
// fn leave_piece_test() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// player.free_piece(0); 
// player.free_piece(1);

// player.move_piece(0, 1);
// player.join_piece(1, 1);

// player.leave_piece(0, 1);
// }

// #[test]
// fn leave_piece_from_globe_test() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// player.free_piece(0); 
// player.free_piece(1);

// player.move_piece(0, 8);
// player.join_piece(1, 8);

// player.leave_piece(0, 1);
// }

// #[test]
// fn leave_piece_from_invincible_test() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// player.free_piece(0); 
// player.free_piece(1);

// player.move_piece(0, 13);
// player.join_piece(1, 13);

// player.leave_piece(0, 1);
// }

// #[test]
// fn kill_piece_test() {
// let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board.clone());
// let mut other_player = Player::new(OTHER_PLAYER_ID, board);

// other_player.free_piece(0);
// other_player.free_piece(1);
// other_player.free_piece(2);
// other_player.free_piece(3);

// other_player.update_outside(0, 13, 0);
// other_player.update_outside(1, 13, 0);
// other_player.update_outside(2, 13, 0);
// other_player.update_outside(3, 13, 1);

// player.kill_piece(0, 6);
// assert_eq!(other_player.piece(0).borrow().position(), -1);
// assert_eq!(other_player.piece(1).borrow().position(), -1);
// assert_eq!(other_player.piece(2).borrow().position(), -1);
// assert!(other_player.piece(0).borrow().is_home());
// assert!(other_player.piece(1).borrow().is_home());
// assert!(other_player.piece(2).borrow().is_home());

// player.kill_piece(0, 1);
// assert_eq!(other_player.piece(3).borrow().position(), -1);
// assert!(other_player.piece(3).borrow().is_home());
// }

// #[test]
// fn kill_piece_by_starjump_test() {
// let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board.clone());
// let mut other_player = Player::new(OTHER_PLAYER_ID, board);

// other_player.free_piece(0);
// other_player.free_piece(1);
// player.free_piece(0);

// other_player.update_outside(0, 13, 5);
// other_player.update_outside(1, 13, 11);

// player.kill_piece(0, 5);
// assert_eq!(other_player.piece(0).borrow().position(), -1);
// assert!(other_player.piece(0).borrow().is_home());
// assert_eq!(other_player.piece(1).borrow().position(), -1);
// assert!(other_player.piece(1).borrow().is_home());
// assert_eq!(player.piece(0).borrow().position(), 11);
// }


// #[test]
// fn kill_piece_and_join() {
//     let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
//     let mut player = Player::new(PLAYER_ID, board.clone());
//     let mut other_player = Player::new(OTHER_PLAYER_ID, board);

//     other_player.free_piece(0);
//     player.free_piece(0);
//     player.free_piece(1);

//     player.starjump_piece(1, 5);
//     other_player.update_outside(0, 13, 5);

//     player.kill_piece(0, 5);
//     assert_eq!(other_player.piece(0).borrow().position(), -1);
//     assert!(other_player.piece(0).borrow().is_home());

//     assert_eq!(player.piece(0).borrow().position(), 11);
//     assert_eq!(player.piece(1).borrow().position(), 11);
    

// }

// #[test]
// fn get_playerpiece_heuristics() {
// let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

//     for piece_id in 0..4 {
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 57);

//         player.free_piece(piece_id);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 56);

//         player.move_piece(piece_id, 1);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 55);

//         player.starjump_piece(piece_id, 4);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 45);

//         player.move_piece(piece_id, 6);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 39);

//         player.save_piece(piece_id, 4);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 35);

//         player.starjump_piece(piece_id, 3);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 25);

//         player.starjump_piece(piece_id, 6);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 12);

//         player.win_piece(piece_id, 6);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 0);
//     }
// }

// #[test]
// fn get_playerpiece_heuristics_2() {
// let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

//     for piece_id in 0..4 {
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 57);

//         player.free_piece(piece_id);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 56);

//         player.move_piece(piece_id, 1);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 55);

//         player.starjump_piece(piece_id, 4);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 45);

//         player.move_piece(piece_id, 6);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 39);

//         player.save_piece(piece_id, 4);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 35);

//         player.move_piece(piece_id, 5);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 30);

//         player.starjump_piece(piece_id, 5);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 19);

//         player.move_piece(piece_id, 6);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 13);

//         player.starjump_piece(piece_id, 1);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 6);

//         player.win_piece(piece_id, 6);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 0);
//     }
// }

// #[test]
// fn get_playerpiece_heuristics_3() {
// let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

//     for piece_id in 0..4 {
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 57);

//         player.free_piece(piece_id);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 56);

//         player.move_piece(piece_id, 6);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 50);

//         player.starjump_piece(piece_id, 5);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 38);

//         player.starjump_piece(piece_id, 6);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 25);

//         player.starjump_piece(piece_id, 6);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 12);

//         player.save_piece(piece_id, 3);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 9);

//         player.save_piece(piece_id, 4);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 5);

//         player.win_piece(piece_id, 5);
//         let result = player.get_heuristics(piece_id);
//         assert_eq!(result, 0);
//     }
// }
// }

// #[cfg(test)]
// mod player_1_valid_choices_tests {
// use super::*;

// #[test]
// fn valid_choice_nothing_test() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);
// let mut result: Act;

// result = player.valid_choices(0, 1, Act::Nothing);
// assert_eq!(result, Act::Nothing);

// player.free_piece(0);
// result = player.valid_choices(0, 7, Act::Move);
// assert_eq!(result, Act::Nothing);

// player.enter_goal(0, 0);
// result = player.valid_choices(0, 7, Act::Move);
// assert_eq!(result, Act::Nothing);
// }

// #[test]
// fn valid_choice_test_1() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// let mut result = player.valid_choices(0, 0, Act::Free);
// assert_eq!(result, Act::Nothing);
// assert!(player.piece(0).borrow().is_home());

// result = player.valid_choices(0, 5, Act::Free);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 6, Act::Move);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 6, Act::Free);
// assert_eq!(result, Act::Free);

// player.free_piece(0);
// assert!(player.piece(0).borrow().is_free());

// result = player.valid_choices(0, 6, Act::Free);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 1, Act::Move);
// assert_eq!(result, Act::Move);

// player.move_piece(0, 1);
// result = player.valid_choices(0, 4, Act::Move);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 4, Act::Starjump);
// assert_eq!(result, Act::Starjump);

// player.starjump_piece(0, 4);

// result = player.valid_choices(0, 2, Act::Move);
// assert_eq!(result, Act::Move);

// player.move_piece(0, 2);
// result = player.valid_choices(0, 6, Act::Move);
// assert_eq!(result, Act::Move);

// player.move_piece(0, 6);
// result = player.valid_choices(0, 2, Act::Move);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 2, Act::Starjump);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 2, Act::Move);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 2, Act::Safe);
// assert_eq!(result, Act::Safe);
// player.save_piece(0, 2);

// result = player.valid_choices(0, 2, Act::Kill);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 2, Act::Die);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 2, Act::Leave);
// assert_eq!(result, Act::Nothing);

// player.starjump_piece(0, 3);
// player.starjump_piece(0, 6);

// result = player.valid_choices(0, 6, Act::Move);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 6, Act::Starjump);
// assert_eq!(result, Act::Starjump);

// result = player.valid_choices(0, 6, Act::Goal);
// assert_eq!(result, Act::Goal);

// result = player.valid_choices(0, 6, Act::Safe);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 3, Act::Safe);
// assert_eq!(result, Act::Safe);

// player.save_piece(0, 3);
// result = player.valid_choices(0, 6, Act::Move);
// assert_eq!(result, Act::Move);

// result = player.valid_choices(0, 6, Act::Safe);
// assert_eq!(result, Act::Safe);

// player.move_piece(0, 6);

// result = player.valid_choices(0, 1, Act::Move);
// assert_eq!(result, Act::Move);

// result = player.valid_choices(0, 1, Act::Safe);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 6, Act::Move);
// assert_eq!(result, Act::Move);

// result = player.valid_choices(0, 3, Act::Move);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 3, Act::Starjump);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 3, Act::Safe);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 3, Act::Goal);
// assert_eq!(result, Act::Goal);
// player.win_piece(0, 3);
// assert!(player.piece(0).borrow().is_goal());
// }

// #[test]
// fn valid_choice_test_2() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board);

// let mut result = player.valid_choices(0, 0, Act::Free);
// assert_eq!(result, Act::Nothing);

// assert!(player.piece(0).borrow().is_home());
// assert!(player.piece(1).borrow().is_home());
// assert!(player.piece(2).borrow().is_home());
// assert!(player.piece(3).borrow().is_home());

// (0..4).for_each(|i| {
//     result = player.valid_choices(i, 6, Act::Free);
//     assert_eq!(result, Act::Free);
//     player.free_piece(i);
// });

// assert!(player.piece(0).borrow().is_free());
// assert!(player.piece(1).borrow().is_free());
// assert!(player.piece(2).borrow().is_free());
// assert!(player.piece(3).borrow().is_free());

// result = player.valid_choices(4, 6, Act::Free);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 1, Act::Move);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 1, Act::Leave);
// assert_eq!(result, Act::Leave);
// player.leave_piece(0, 1);

// result = player.valid_choices(1, 5, Act::Leave);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(1, 5, Act::Starjump);
// assert_eq!(result, Act::Starjump);
// player.starjump_piece(1, 5);

// result = player.valid_choices(2, 5, Act::Leave);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(2, 5, Act::Move);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(2, 5, Act::Starjump);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(2, 5, Act::Join);
// assert_eq!(result, Act::Join);
// player.join_piece(2, 5);

// result = player.valid_choices(3, 5, Act::Leave);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(3, 6, Act::Move);
// assert_eq!(result, Act::Move);
// player.move_piece(3, 6);

// result = player.valid_choices(0, 5, Act::Join);
// assert_eq!(result, Act::Join);

// player.join_piece(0, 5);
// result = player.valid_choices(0, 2, Act::Move);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 2, Act::Leave);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 2, Act::Safe);
// assert_eq!(result, Act::Safe);

// player.save_piece(0, 2);

// result = player.valid_choices(3, 2, Act::Join);
// assert_eq!(result, Act::Join);

// result = player.valid_choices(3, 2, Act::Safe);
// assert_eq!(result, Act::Safe);

// player.join_piece(3, 2);

// result = player.valid_choices(0, 3, Act::Leave);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 3, Act::Join);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 3, Act::Starjump);
// assert_eq!(result, Act::Starjump);
// player.starjump_piece(0, 3);

// result = player.valid_choices(3, 5, Act::Move);
// assert_eq!(result, Act::Move);
// player.move_piece(3, 5);

// result = player.valid_choices(1, 2, Act::Move);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(1, 2, Act::Safe);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(1, 2, Act::Leave);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(1, 2, Act::Join);
// assert_eq!(result, Act::Join);
// player.join_piece(1, 2);

// player.update_outside(0, 18, 24);
// player.update_outside(2, 11, 18);

// result = player.valid_choices(1, 5, Act::Move);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(1, 5, Act::Leave);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(1, 5, Act::Starjump);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(1, 5, Act::Safe);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(1, 5, Act::Join);
// assert_eq!(result, Act::Join);
// player.join_piece(1, 5);

// result = player.valid_choices(3, 5, Act::Join);
// assert_eq!(result, Act::Join);
// player.join_piece(3, 5);

// result = player.valid_choices(2, 6, Act::Join);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(2, 6, Act::Starjump);
// assert_eq!(result, Act::Starjump);
// player.join_piece(2, 6);

// player.update_outside(0, 24, 50);
// player.update_outside(1, 24, 44);
// player.update_outside(2, 31, 39);
// player.update_outside(3, 24, 39);

// result = player.valid_choices(3, 5, Act::Move);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(3, 5, Act::Starjump);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(3, 5, Act::Safe);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(3, 5, Act::Leave);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(3, 5, Act::Join);
// assert_eq!(result, Act::Join);

// result = player.valid_choices(1, 6, Act::Join);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(1, 6, Act::Starjump);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(1, 6, Act::Goal);
// assert_eq!(result, Act::Goal);
// player.win_piece(1, 6);

// player.move_piece(3, 6);
// result = player.valid_choices(3, 2, Act::Safe);
// assert_eq!(result, Act::Safe);
// player.save_piece(3, 2);
// player.move_piece(2, 6);

// result = player.valid_choices(2, 2, Act::Join);
// assert_eq!(result, Act::Join);

// result = player.valid_choices(2, 2, Act::Safe);
// assert_eq!(result, Act::Safe);
// player.save_piece(2, 2);

// player.move_piece(0, 1);

// result = player.valid_choices(2, 4, Act::Join);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(3, 4, Act::Safe);
// assert_eq!(result, Act::Safe);

// result = player.valid_choices(2, 4, Act::Move);
// assert_eq!(result, Act::Move);
// player.save_piece(3, 4);
// player.move_piece(2, 4);

// result = player.valid_choices(2, 4, Act::Safe);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(2, 4, Act::Leave);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(2, 4, Act::Move);
// assert_eq!(result, Act::Move);
// player.move_piece(2, 4);

// result = player.valid_choices(0, 6, Act::Join);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 6, Act::Leave);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 6, Act::Move);
// assert_eq!(result, Act::Move);
// player.move_piece(0, 6);

// result = player.valid_choices(3, 5, Act::Move);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(3, 5, Act::Goal);
// assert_eq!(result, Act::Goal);
// player.win_piece(3, 5);

// result = player.valid_choices(0, 1, Act::Leave);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 1, Act::Goal);
// assert_eq!(result, Act::Goal);
// player.win_piece(2, 1);
// player.win_piece(0, 1);

// assert!(player.is_finished());
// }

// #[test]
// fn valid_choice_test_3() {
// let board = Rc::new(RefCell::new(Board::new()));
// let mut player = Player::new(PLAYER_ID, board.clone());
// let mut other_player = Player::new(OTHER_PLAYER_ID, board);

// let mut result: Act;

// other_player.free_piece(0);
// other_player.free_piece(1);
// other_player.free_piece(2);
// other_player.free_piece(3);

// result = player.valid_choices(0, 6, Act::Free);
// assert_eq!(result, Act::Free);

// other_player.update_piece(0, 13, 0);

// result = player.valid_choices(0, 6, Act::Free);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 6, Act::Kill);
// assert_eq!(result, Act::Kill);

// other_player.update_piece(1, 13, 0);

// result = player.valid_choices(0, 6, Act::Die);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 6, Act::Kill);
// assert_eq!(result, Act::Kill);

// other_player.move_piece(0, 1);
// other_player.move_piece(1, 1);

// player.free_piece(0);
// player.free_piece(1);
// player.free_piece(2);

// result = player.valid_choices(0, 1, Act::Move);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 1, Act::Kill);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 1, Act::Leave);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 1, Act::Die);
// assert_eq!(result, Act::Die);

// other_player.move_piece(0, 1);

// result = player.valid_choices(0, 1, Act::Move);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 1, Act::Kill);
// assert_eq!(result, Act::Kill);

// result = player.valid_choices(0, 1, Act::Leave);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 1, Act::Die);
// assert_eq!(result, Act::Nothing);

// other_player.move_piece(0, 3);

// result = player.valid_choices(0, 5, Act::Starjump);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 5, Act::Die);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 5, Act::Kill);
// assert_eq!(result, Act::Kill);

// player.update_outside(1, 0, 11);

// result = player.valid_choices(0, 5, Act::Die);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 5, Act::Join);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 5, Act::Kill);
// assert_eq!(result, Act::Kill);

// other_player.update_piece(1, 1, 5);

// result = player.valid_choices(0, 5, Act::Join);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 5, Act::Kill);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 5, Act::Die);
// assert_eq!(result, Act::Die);

// player.move_piece(1, 1);
// other_player.update_outside(2, 13, 11);

// result = player.valid_choices(0, 5, Act::Starjump);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 5, Act::Kill);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 5, Act::Die);
// assert_eq!(result, Act::Die);

// other_player.update_outside(1, 5, 11);

// result = player.valid_choices(0, 5, Act::Starjump);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 5, Act::Kill);
// assert_eq!(result, Act::Kill);

// player.kill_piece(0, 5);
// assert!(player.piece(0).borrow().is_home());
// assert_eq!(player.piece(0).borrow().position(), -1);
// assert!(other_player.piece(0).borrow().is_home());
// assert_eq!(other_player.piece(0).borrow().position(), -1);

// player.free_piece(0);
// other_player.free_piece(0);
// other_player.update_outside(0, 13, 5);

// result = player.valid_choices(0, 5, Act::Die);
// assert_eq!(result, Act::Die);

// player.die_piece(0, 5);
// assert!(player.piece(0).borrow().is_home());
// assert_eq!(player.piece(0).borrow().position(), -1);
// assert!(other_player.piece(0).borrow().is_home());
// assert_eq!(other_player.piece(0).borrow().position(), -1);

// player.free_piece(0);
// other_player.free_piece(0);
// other_player.update_outside(0, 13, 5);

// player.leave_piece(0, 6);
// other_player.enter_globe(0, 5, 8);

// result = player.valid_choices(0, 2, Act::Safe);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 2, Act::Kill);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 2, Act::Die);
// assert_eq!(result, Act::Die);

// other_player.move_piece(0, 1);

// player.save_piece(0, 2);
// player.enter_globe(2, 0, 8);
// player.move_piece(1, 2);

// other_player.update_outside(1, 11, 12);
// other_player.update_outside(2, 11, 12);

// result = player.valid_choices(0, 4, Act::Die);
// assert_eq!(result, Act::Die);

// result = player.valid_choices(0, 5, Act::Die);
// assert_eq!(result, Act::Die);

// other_player.update_outside(1, 12, 26);
// other_player.update_outside(2, 12, 25);

// player.update_outside(0, 8, 24);

// result = player.valid_choices(0, 2, Act::Kill);
// assert_eq!(result, Act::Kill);

// result = player.valid_choices(0, 2, Act::Die);
// assert_eq!(result, Act::Nothing);

// other_player.move_piece(2, 1);

// result = player.valid_choices(0, 2, Act::Kill);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 2, Act::Die);
// assert_eq!(result, Act::Die);

// other_player.update_outside(1, 26, 50);
// player.update_outside(0, 24, 43);

// result = player.valid_choices(0, 1, Act::Kill);
// assert_eq!(result, Act::Kill);

// result = player.valid_choices(0, 1, Act::Starjump);
// assert_eq!(result, Act::Nothing);

// player.update_outside(1, 14, 45);

// result = player.valid_choices(1, 5, Act::Starjump);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(1, 5, Act::Goal);
// assert_eq!(result, Act::Goal);

// player.win_piece(1, 5);
// assert_eq!(player.piece(1).borrow().position(), 99);
// assert!(player.piece(1).borrow().is_goal());
// assert_eq!(other_player.piece(1).borrow().position(), -1);
// assert!(other_player.piece(1).borrow().is_home());

// other_player.update_outside(2, 26, 50);
// player.move_piece(0, 6);

// result = player.valid_choices(0, 1, Act::Starjump);
// assert_eq!(result, Act::Nothing);

// result = player.valid_choices(0, 1, Act::Goal);
// assert_eq!(result, Act::Goal);

// result = player.valid_choices(0, 1, Act::Kill);
// assert_eq!(result, Act::Kill);
// player.kill_piece(0, 1);

// assert_eq!(player.piece(0).borrow().position(), 99);
// assert!(player.piece(0).borrow().is_goal());

// assert_eq!(other_player.piece(1).borrow().position(), -1);
// assert!(other_player.piece(1).borrow().is_home());

// }

// }

// #[cfg(test)]
// mod player_1_play_test {
//     use super::*;

//         static ACTIONS: [Act; 10] = [
//         Act::Move,
//         Act::Free,
//         Act::Kill,
//         Act::Join,
//         Act::Leave,
//         Act::Die,
//         Act::Goal,
//         Act::Safe,
//         Act::Starjump,
//         Act::Nothing,
//     ];

//     #[test]
//     fn make_move_test() {
//         let board = Rc::new(RefCell::new(Board::new()));

//         let mut player = Player::new(PLAYER_ID, board.clone());
//         let mut other_player = Player::new(1, board);

//         other_player.free_piece(0);

//         let mut dice_number: i8;
//         let mut choice: Act;

//         dice_number = 6;
//         choice = player.valid_choices(0, dice_number, Act::Free);
//         player.make_move(0, dice_number, choice);
//         assert!(player.piece(0).borrow().is_free());
    
//         dice_number = 5;
//         choice = player.valid_choices(1, dice_number, Act::Free);
//         player.make_move(1, dice_number, choice);
//         assert!(!player.piece(1).borrow().is_free());

//         dice_number = 1;
//         choice = player.valid_choices(0, dice_number, Act::Move);
//         player.make_move(0, dice_number, choice);
//         assert_eq!(player.piece(0).borrow().position(), 1);

//         dice_number = 4;
//         choice = player.valid_choices(0, dice_number, Act::Move);
//         player.make_move(0, dice_number, choice);
//         assert_eq!(player.piece(0).borrow().position(), 1);
//         choice = player.valid_choices(0, dice_number, Act::Starjump);
//         player.make_move(0, dice_number, choice);
//         assert_eq!(player.piece(0).borrow().position(), 11);

//         dice_number = 2;
//         choice = player.valid_choices(0, dice_number, Act::Move);
//         player.make_move(0, dice_number, choice);
//         assert_eq!(player.piece(0).borrow().position(), 11);
//         choice = player.valid_choices(0, dice_number, Act::Die);
//         player.make_move(0, dice_number, choice);
//         assert!(player.piece(0).borrow().is_home());
//         assert_eq!(player.piece(0).borrow().position(), -1);

//         dice_number = 6;
//         choice = player.valid_choices(0, dice_number, Act::Free);
//         player.make_move(0, dice_number, choice);
//         assert!(player.piece(0).borrow().is_free());

//         dice_number = 6;
//         choice = player.valid_choices(0, dice_number, Act::Move);
//         player.make_move(0, dice_number, choice);
//         assert_eq!(player.piece(0).borrow().position(), 6);

//         dice_number = 2;
//         choice = player.valid_choices(0, dice_number, Act::Safe);
//         player.make_move(0, dice_number, choice);
//         assert_eq!(player.piece(0).borrow().position(), 8);

//         other_player.move_piece(0, 1);

//         dice_number = 6;
//         choice = player.valid_choices(0, dice_number, Act::Kill);
//         player.make_move(0, dice_number, choice);
//         assert_eq!(player.piece(0).borrow().position(), 14);

//         choice = player.valid_choices(1, dice_number, Act::Free);
//         player.make_move(1, dice_number, choice);
//         assert!(player.piece(1).borrow().is_free());
//         choice = player.valid_choices(2, dice_number, Act::Free);
//         player.make_move(2, dice_number, choice);
//         assert!(player.piece(2).borrow().is_free());

//         choice = player.valid_choices(1, dice_number, Act::Move);
//         player.make_move(1, dice_number, choice);
//         assert_eq!(player.piece(1).borrow().position(), 0);

//         choice = player.valid_choices(1, dice_number, Act::Leave);
//         player.make_move(1, dice_number, choice);
//         assert_eq!(player.piece(1).borrow().position(), 6);

//         choice = player.valid_choices(2, dice_number, Act::Move);
//         player.make_move(2, dice_number, choice);
//         assert_eq!(player.piece(2).borrow().position(), 0);

//         choice = player.valid_choices(2, dice_number, Act::Join);
//         player.make_move(2, dice_number, choice);
//         assert_eq!(player.piece(2).borrow().position(), 6);

//         player.move_piece(0, 35);

//         choice = player.valid_choices(0, dice_number, Act::Goal);
//         player.make_move(0, dice_number, choice);
//         assert_eq!(player.piece(0).borrow().position(), 49);

//         dice_number = 1;
//         choice = player.valid_choices(0, dice_number, Act::Goal);
//         player.make_move(0, dice_number, choice);
//         assert_eq!(player.piece(0).borrow().position(), 99);
//         assert!(player.piece(0).borrow().is_goal());
//     }

//     #[test]
//     fn generate_action_vector_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let mut player = Player::new(PLAYER_ID, board);

//         let mut result: Vec<(Act, i8, i8)>;
//         let mut dice_number: i8;

//         dice_number = 6;
//         result = player.generate_action_vector(dice_number, Act::Move);
//         assert_eq!(result.len(), 0);
//         result = player.generate_action_vector(dice_number, Act::Free);
//         assert_eq!(result.len(), 4);
//         assert_eq!(result.first().unwrap().2, 57);


//         player.free_piece(0);

//         result = player.generate_action_vector(dice_number, Act::Free);
//         assert_eq!(result.len(), 3);

//         result = player.generate_action_vector(dice_number, Act::Move);
//         assert_eq!(result.len(), 1);
//         assert_eq!(result.first().unwrap().2, 56);


//         player.free_piece(1);
//         player.free_piece(2);
//         player.free_piece(3);

//         result = player.generate_action_vector(dice_number, Act::Free);
//         assert_eq!(result.len(), 0);

//         result = player.generate_action_vector(dice_number, Act::Move);
//         assert_eq!(result.len(), 0);

//         result = player.generate_action_vector(dice_number, Act::Leave);
//         assert_eq!(result.len(), 4);
//         assert_eq!(result[0].2, 56);
//         assert_eq!(result[1].2, 56);
//         assert_eq!(result[2].2, 56);
//         assert_eq!(result[3].2, 56);


//         player.leave_piece(0, dice_number);

//         result = player.generate_action_vector(dice_number, Act::Leave);
//         assert_eq!(result.len(), 0);

//         result = player.generate_action_vector(dice_number, Act::Join);
//         assert_eq!(result.len(), 3);

//         player.join_piece(1, dice_number);

//         dice_number = 5;
//         result = player.generate_action_vector(dice_number, Act::Starjump);
//         assert_eq!(result.len(), 4);
//         assert_eq!(result[0].2, 50);
//         assert_eq!(result[1].2, 50);
//         assert_eq!(result[2].2, 56);
//         assert_eq!(result[3].2, 56);
//     }

//     #[test]
//     fn select_ordered_piece_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let mut player = Player::new(PLAYER_ID, board);

//         let mut action_vector: Vec<(Act, i8, i8)>;
//         let mut dice_number: i8;
//         let mut result: (Act, i8, i8);

//         dice_number = 6;
//         action_vector = player.generate_action_vector(dice_number, Act::Move);
//         assert_eq!(action_vector.len(), 0);
        
//         result = player.select_ordered_piece(action_vector, true);
//         assert_eq!(result.0, Act::Nothing);
//         assert_eq!(result.2, 57);


//         action_vector = player.generate_action_vector(dice_number, Act::Free);
//         assert_eq!(action_vector.len(), 4);
//         result = player.select_ordered_piece(action_vector, true);
//         assert_eq!(result.0, Act::Free);
//         assert_eq!(result.2, 57);

//         player.free_piece(0);

//         action_vector = player.generate_action_vector(dice_number, Act::Free);
//         assert_eq!(action_vector.len(), 3);

//         action_vector = player.generate_action_vector(dice_number, Act::Move);
//         assert_eq!(action_vector.len(), 1);
//         assert_eq!(action_vector.first().unwrap().2, 56);
//         result = player.select_ordered_piece(action_vector, true);
//         assert_eq!(result.0, Act::Move);
//         assert_eq!(result.2, 56);


//         player.free_piece(1);
//         player.free_piece(2);
//         player.free_piece(3);

//         action_vector = player.generate_action_vector(dice_number, Act::Free);
//         assert_eq!(action_vector.len(), 0);

//         action_vector = player.generate_action_vector(dice_number, Act::Move);
//         assert_eq!(action_vector.len(), 0);

//         action_vector = player.generate_action_vector(dice_number, Act::Leave);
//         assert_eq!(action_vector.len(), 4);
//         assert_eq!(action_vector[0].2, 56);
//         assert_eq!(action_vector[1].2, 56);
//         assert_eq!(action_vector[2].2, 56);
//         assert_eq!(action_vector[3].2, 56);

//         result = player.select_ordered_piece(action_vector, true);
//         assert_eq!(result.0, Act::Leave);
//         assert_eq!(result.2, 56);
        

//         player.leave_piece(0, dice_number);

//         action_vector = player.generate_action_vector(dice_number, Act::Leave);
//         assert_eq!(action_vector.len(), 0);

//         action_vector = player.generate_action_vector(dice_number, Act::Join);
//         assert_eq!(action_vector.len(), 3);

//         player.join_piece(1, dice_number);

//         dice_number = 5;
//         action_vector = player.generate_action_vector(dice_number, Act::Starjump);
//         assert_eq!(action_vector.len(), 4);
//         assert_eq!(action_vector[0].2, 50);
//         assert_eq!(action_vector[1].2, 50);
//         assert_eq!(action_vector[2].2, 56);
//         assert_eq!(action_vector[3].2, 56);

//         result = player.select_ordered_piece(action_vector.clone(), true);
//         assert_eq!(result.0, Act::Starjump);
//         assert_eq!(result.2, 50);

//         result = player.select_ordered_piece(action_vector, false);
//         assert_eq!(result.0, Act::Starjump);
//         assert_eq!(result.2, 56);

//         player.leave_piece(0, 5);
//         player.leave_piece(0, 2);

//         action_vector = player.generate_action_vector(dice_number, Act::Starjump);
//         assert_eq!(action_vector.len(), 4);
//         assert_eq!(action_vector[0].2, 43);
//         assert_eq!(action_vector[1].2, 50);
//         assert_eq!(action_vector[2].2, 56);
//         assert_eq!(action_vector[3].2, 56);

//         result = player.select_ordered_piece(action_vector, true);
//         assert_eq!(result.0, Act::Starjump);
//         assert_eq!(result.1, 0);
//         assert_eq!(result.2, 43);
//     }

//     #[test]
//     fn make_ordered_choice() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let mut player = Player::new(PLAYER_ID, board);

//         let mut dice_number: i8;
//         let mut result: (Act, i8, i8);

//         dice_number = 6;

//         result = player.make_ordered_choice(dice_number, Act::Move, true);
        
//         assert_eq!(result.0, Act::Nothing);
//         assert_eq!(result.2, 57);


//         result = player.make_ordered_choice(dice_number, Act::Free, true);
//         assert_eq!(result.0, Act::Free);
//         assert_eq!(result.2, 57);

//         player.free_piece(0);

//         result = player.make_ordered_choice(dice_number, Act::Move, true);
//         assert_eq!(result.0, Act::Move);
//         assert_eq!(result.2, 56);


//         player.free_piece(1);
//         player.free_piece(2);
//         player.free_piece(3);

//         result = player.make_ordered_choice(dice_number, Act::Leave, true);

//         assert_eq!(result.0, Act::Leave);
//         assert_eq!(result.2, 56);
        

//         player.leave_piece(0, dice_number);
//         player.join_piece(1, dice_number);

//         dice_number = 5;
//         result = player.make_ordered_choice(dice_number, Act::Starjump, true);
//         assert_eq!(result.0, Act::Starjump);
//         assert_eq!(result.2, 50);

//         result = player.make_ordered_choice(dice_number,Act::Starjump, false);
//         assert_eq!(result.0, Act::Starjump);
//         assert_eq!(result.2, 56);

//         player.leave_piece(0, 5);
//         player.leave_piece(0, 2);

//         result = player.make_ordered_choice(dice_number,Act::Starjump, true);
//         assert_eq!(result.0, Act::Starjump);
//         assert_eq!(result.1, 0);
//         assert_eq!(result.2, 43);

//         result = player.make_ordered_choice(dice_number,Act::Starjump, false);
//         assert_eq!(result.0, Act::Starjump);
//         assert_eq!(result.1, 2);
//         assert_eq!(result.2, 56);
//     }

//     #[test]
//     #[ignore]
//     fn random_play_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let mut player = Player::new(PLAYER_ID, board.clone());
//         let dice = Dice::default();
//         let actions = ACTIONS.to_vec();
//         player.my_turn();
//         player.take_dice(dice);
        
//             for _ in 0..100 {
//             while !player.is_finished() {
//                 player.play_random(actions.clone());
//                 player.print_status();
//             }
//             board.borrow_mut().reset();
//         }
//     }

//     #[test]
//     #[ignore]
//     fn ordered_play_test() {
//         let board = Rc::new(RefCell::new(Board::new()));
//         let mut player = Player::new(PLAYER_ID, board.clone());
//         let dice = Dice::default();
//         let actions = ACTIONS.to_vec();
//         player.my_turn();
//         player.take_dice(dice);
        
//             for _ in 0..100 {
//             while !player.is_finished() {
//                 player.play_ordered(actions.clone(), true);
//                 player.print_status();
//             }
//             board.borrow_mut().reset();
//         }
//     }
// }