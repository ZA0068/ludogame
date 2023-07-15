use board::Board;
use dice::Dice;
use players::Player;
use std::{cell::RefCell, rc::Rc};

#[cfg(test)]
mod default_player_tests {

    use super::*;
    #[test]
    fn add_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(0);
        player.setup(board.clone());
        assert_eq!(player.id(), 0);
        assert_eq!(player.board().as_ptr(), board.as_ptr());
    }

    #[test]
    fn get_pieces_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        for player_id in 0..4 {
            let mut player = Player::new(player_id);
            player.setup(board.clone());
            for i in 0..4 {
                let piece = player.board().borrow_mut().home(player_id).piece(i);
                assert_eq!(piece.borrow().id(), i);
                assert!(piece.borrow().is_home());
                assert_eq!(
                    piece.as_ptr(),
                    board.borrow_mut().home(player_id).piece(i).as_ptr()
                );
            }
        }
    }

    #[test]
    fn player_with_dice_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(0);
        player.setup(board);
        player.roll_dice();
        let result = player.get_dice_number();
        assert!(result == 0);

        let dice = Dice::default();

        player.get_dice(dice);
        player.roll_dice();
        let result = player.get_dice_number();
        assert!(result > 0 && result < 7);

        player.drop_dice();
        player.roll_dice();
        let result = player.get_dice_number();
        assert!(result == 0);
    }

    #[test]
    fn star_position_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(0);
        player.setup(board.clone());
        for i in 0..5 {
            let position = player.star_position(i, 5);
            assert_eq!(position, 11);
        }

        let position = player.star_position(51, 5);
        assert_eq!(position, 11);

        for i in 5..11 {
            let position = player.star_position(i, 11);
            assert_eq!(position, 18);
        }

        for i in 12..18 {
            let position = player.star_position(i, 18);
            assert_eq!(position, 24);
        }

        for i in 18..24 {
            let position = player.star_position(i, 24);
            assert_eq!(position, 31);
        }

        for i in 31..37 {
            let position = player.star_position(i, 37);
            assert_eq!(position, 44);
        }

        for i in 44..50 {
            let position = player.star_position(i, 50);
            assert_eq!(position, 50);
        }

        let mut player = Player::new(1);
        player.setup(board);
        for i in 44..50 {
            let position = player.star_position(i, 50);
            assert_eq!(position, 5);
        }
    }

    #[test]
    fn circumvent_player_0_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(0);
        player.setup(board.clone());

        let position = player.circumvent_player_0(51, 57);
        assert_eq!(position, 57);

        let mut player = Player::new(1);
        player.setup(board);
        let position = player.circumvent_player_0(51, 57);
        assert_eq!(position, 5);
    }

    #[test]
    fn send_pieces_home_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(0);
        player.setup(board);
        player.free_piece(0);

        player.update_outside(0, 0, 6);

        player.send_other_piece_home(6);

        assert!(player
            .board()
            .borrow_mut()
            .home(0)
            .piece(0)
            .borrow()
            .is_home());
        assert_eq!(
            player
                .board()
                .borrow_mut()
                .home(0)
                .piece(0)
                .borrow()
                .position(),
            -1
        );
    }
}
