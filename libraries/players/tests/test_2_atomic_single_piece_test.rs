use board::Board;
use pieces::Color;
use players::Player;

use std::{cell::RefCell, rc::Rc};

mod atomic_single_piece_test {

    use super::*;
    const PLAYER_ID: i8 = 0;

    #[test]
    fn free_piece_test() {
        let piece_id = 0;
        let new_position = 0;

        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let piece = player.piece(piece_id);

        assert!(piece.borrow_mut().is_home());
        assert!(player
            .board()
            .borrow_mut()
            .home(PLAYER_ID)
            .piece(0)
            .borrow_mut()
            .is_home());
        assert_eq!(player.board().borrow_mut().home(PLAYER_ID).pieces.len(), 4);

        player.free_piece(piece_id);
        assert!(!piece.borrow_mut().is_home());
        assert!(piece.borrow_mut().is_free());
        assert_eq!(piece.borrow_mut().position(), 0);

        assert_eq!(player.board().borrow_mut().home(PLAYER_ID).pieces.len(), 3);
        assert_eq!(
            player
                .board()
                .borrow_mut()
                .outside(new_position)
                .pieces
                .len(),
            1
        );
        assert_eq!(
            player.board().borrow_mut().outside(new_position).player_id,
            Some(board::PlayerID::Player0)
        );
        assert_eq!(
            player
                .board()
                .borrow_mut()
                .outside(new_position)
                .piece(piece_id)
                .borrow()
                .color(),
            Color::Green
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
            .is_free());
    }

    #[test]
    fn update_piece_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let piece_id = 0;
        player.free_piece(piece_id);

        let position = 4;
        player.update_outside(piece_id, 0, position);
        let piece = player.piece(piece_id);
        assert_eq!(piece.borrow_mut().position(), position);
        assert_eq!(
            player.board().borrow_mut().outside(position).pieces.len(),
            1
        );
        assert_eq!(
            player.board().borrow_mut().outside(position).player_id,
            Some(board::PlayerID::Player0)
        );
        assert_eq!(
            player
                .board()
                .borrow_mut()
                .outside(position)
                .piece(piece_id)
                .borrow()
                .color(),
            Color::Green
        );
        assert!(piece.borrow_mut().is_free());
        assert!(player
            .board()
            .borrow_mut()
            .outside(position)
            .piece(piece_id)
            .borrow_mut()
            .is_free());
        assert_eq!(
            player
                .board()
                .borrow_mut()
                .outside(position)
                .piece(piece_id)
                .borrow_mut()
                .position(),
            position
        );
    }
    #[test]
    fn return_home_or_death_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let piece_id = 0;

        player.free_piece(piece_id);
        player.update_outside(piece_id, 0, 1);

        let piece = player.piece(piece_id);
        assert_eq!(piece.borrow_mut().position(), 1);
        assert_eq!(player.board().borrow_mut().outside(1).pieces.len(), 1);

        player.die(piece_id);

        assert_eq!(piece.borrow_mut().position(), -1);
        assert_eq!(player.board().borrow_mut().outside(1).pieces.len(), 0);
        assert_eq!(player.board().borrow_mut().home(PLAYER_ID).pieces.len(), 4);
    }
    #[test]
    fn move_piece_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let piece_id = 0;

        for position in 1..=6 {
            player.free_piece(0);
            player.move_piece(0, position);

            let piece = player.piece(piece_id);
            assert_eq!(piece.borrow_mut().position(), position);
            assert_eq!(
                player.board().borrow_mut().outside(position).pieces.len(),
                1
            );
            assert_eq!(
                player.board().borrow_mut().outside(position).player_id,
                Some(board::PlayerID::Player0)
            );
            assert_eq!(
                player
                    .board()
                    .borrow_mut()
                    .outside(position)
                    .piece(piece_id)
                    .borrow()
                    .color(),
                Color::Green
            );
            assert!(piece.borrow_mut().is_free());
            assert!(player
                .board()
                .borrow_mut()
                .outside(position)
                .piece(piece_id)
                .borrow_mut()
                .is_free());
            assert_eq!(
                player
                    .board()
                    .borrow_mut()
                    .outside(position)
                    .piece(piece_id)
                    .borrow_mut()
                    .position(),
                position
            );
            player.die(piece_id);
        }
    }

    #[test]
    fn update_piece_state_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        let piece_id: i8 = 0;
        let mut next_position: i8 = 4;
        print!("next_position: {}", next_position);

        player.free_piece(piece_id);
        player.move_piece(piece_id, next_position);

        let piece = player.piece(piece_id);
        assert_eq!(piece.borrow_mut().position(), next_position);
        assert!(piece.borrow_mut().is_free());
        assert!(player
            .board()
            .borrow_mut()
            .outside(next_position)
            .piece(piece_id)
            .borrow_mut()
            .is_free());
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
        let mut player = Player::new(PLAYER_ID, board);

        let piece_id = 0;
        let piece_move = player.valid_moves(piece_id, 1);
        assert!(!piece_move);

        let piece_move = player.valid_moves(piece_id, 7);
        assert!(!piece_move);

        let piece_move = player.valid_moves(piece_id, 6);
        assert!(piece_move);
        player.free_piece(piece_id);

        let piece_move = player.valid_moves(piece_id, 6);
        assert!(piece_move);

        let piece_id = 4;
        let piece_move = player.valid_moves(piece_id, 1);
        assert!(!piece_move);
    }

    #[test]
    fn enter_inside_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let piece_id = 0;

        player.free_piece(piece_id);
        player.update_outside(piece_id, 0, 50);
        assert_eq!(player.piece(piece_id).borrow_mut().position(), 50);
        assert!(player.piece(piece_id).borrow_mut().is_free());
        assert_eq!(player.board().borrow_mut().outside(50).pieces.len(), 1);

        player.enter_inside(piece_id, 50, 52);
        assert_eq!(player.piece(piece_id).borrow_mut().position(), 52);
        assert!(player.piece(piece_id).borrow_mut().is_free());
        assert_eq!(player.board().borrow_mut().inside(52).pieces.len(), 1);
    
    }
    
    #[test]
    fn update_inside_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let piece_id = 0;

        player.free_piece(piece_id);
        player.update_outside(piece_id, 0, 50);
        player.enter_inside(piece_id, 50, 52);
        assert_eq!(player.piece(piece_id).borrow_mut().position(), 52);
        assert!(player.piece(piece_id).borrow_mut().is_free());
        assert_eq!(player.board().borrow_mut().inside(52).pieces.len(), 1);

        player.update_inside(piece_id, 52, 54);
        assert_eq!(player.piece(piece_id).borrow_mut().position(), 54);
        assert_eq!(player.board().borrow_mut().inside(54).pieces.len(), 1);
    }

    #[test]
    fn move_back_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let piece_id = 0;

        player.free_piece(piece_id);
        player.update_outside(piece_id, 0, 50);
        player.enter_inside(piece_id, 50, 52);
        player.update_inside(piece_id, 52, 54);
        assert_eq!(player.piece(piece_id).borrow_mut().position(), 54);
        assert_eq!(player.board().borrow_mut().inside(54).pieces.len(), 1);
        
        player.update_inside(piece_id, 54, 58);
        assert_eq!(player.piece(piece_id).borrow_mut().position(), 56);
        assert_eq!(player.board().borrow_mut().inside(56).pieces.len(), 1);
    }
    
    #[test]
    fn enter_goal_from_outside_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);
        let piece_id = 0;

        player.free_piece(piece_id);
        player.update_outside(piece_id, 0, 50);
        player.enter_goal(piece_id, 50);

        let piece = player.piece(piece_id);
        assert_eq!(piece.borrow().position(), 99);
        assert_eq!(piece.borrow().color(), pieces::Color::Green);
        assert!(piece.borrow().is_goal());
        let boardstate = player.board();
        assert_eq!(boardstate.borrow_mut().goal(PLAYER_ID).pieces.len(), 1);
        assert_eq!(boardstate.borrow_mut().goal(PLAYER_ID).player_id, Some(board::PlayerID::Player0));
        assert_eq!(boardstate.borrow_mut().goal(PLAYER_ID).piece(piece_id).borrow().color(), pieces::Color::Green);
    }
    
    #[test]
    fn enter_goal_from_inside_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        let piece_id = 0;
        player.free_piece(piece_id);
        player.move_piece(piece_id, 49);
        player.enter_inside(piece_id, 49, 54);

        assert_eq!(player.piece(piece_id).borrow().position(), 54);
        assert!(!player.piece(piece_id).borrow().is_goal());
        let boardstate = player.board();
        assert_eq!(boardstate.borrow_mut().inside(54).pieces.len(), 1);
        assert_eq!(boardstate.borrow().inside[2].player_id, Some(board::PlayerID::Player0));
    
        player.enter_goal(piece_id, 54);
    
        assert_eq!(player.piece(piece_id).borrow().position(), 99);
        assert!(player.piece(piece_id).borrow().is_goal());
        
        let boardstate = player.board();
        assert_eq!(boardstate.borrow_mut().goal(0).pieces.len(), 1);
        assert_eq!(player.board().borrow().goal[0].player_id, Some(board::PlayerID::Player0));
        assert_eq!(boardstate.borrow_mut().goal(PLAYER_ID).piece(piece_id).borrow().color(), pieces::Color::Green);

    }
    
    #[test]
    fn enter_globe_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        let piece_id = 0;
        player.free_piece(piece_id);
        assert!(player.piece(piece_id).borrow().is_free());

        player.move_piece(piece_id, 8);
        assert_eq!(player.piece(piece_id).borrow().position(), 8);
        assert!(player.piece(piece_id).borrow().is_free());
        assert_eq!(player.board().borrow_mut().outside(8).pieces.len(), 1);
        
        player.enter_globe(piece_id, 8, 8);
        assert_eq!(player.piece(piece_id).borrow().position(), 8);
        assert!(player.piece(piece_id).borrow().is_free());
        assert_eq!(player.board().borrow_mut().outside(8).pieces.len(), 1);
    }
    
    #[test]
    fn starjump_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(0, board);

        let piece_id = 0;
        player.free_piece(piece_id);
        player.move_piece(piece_id, 5);
        assert_ne!(player.piece(piece_id).borrow().position(), 11);
        assert!(player.piece(piece_id).borrow().is_free());

        player.update_outside(piece_id, 5, 4);
        player.starjump(piece_id, 4, 5);
        assert_eq!(player.piece(piece_id).borrow().position(), 11);
        assert!(player.piece(piece_id).borrow().is_free());
    }

}
