use board::Board;
use players::Player;
use std::{cell::RefCell, rc::Rc};

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
            assert!(!player.get_piece(piece_id).borrow_mut().is_home());
            assert!(player.get_piece(piece_id).borrow_mut().is_free());
            assert_eq!(player.get_piece(piece_id).borrow_mut().position(), 0);
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

        assert_eq!(player.get_piece(0).borrow_mut().position(), 1);
        assert_eq!(player.get_piece(1).borrow_mut().position(), 1);
        assert_eq!(player.board().borrow_mut().outside(1).pieces.len(), 2);
        
        assert!(player.get_piece(0).borrow_mut().is_free());
        
        assert_eq!(player.get_piece(0).borrow_mut().color(), pieces::Color::Green);
        assert_eq!(player.get_piece(1).borrow_mut().color(), pieces::Color::Green);

        player.update_outside(1, 1, 0);
        player.join(1, 0, 1);

        assert_eq!(player.get_piece(0).borrow_mut().position(), 1);
        assert_eq!(player.get_piece(1).borrow_mut().position(), 1);
        assert_eq!(player.board().borrow_mut().outside(1).pieces.len(), 2);
        
        assert!(player.get_piece(0).borrow_mut().is_free());
        assert!(player.get_piece(1).borrow_mut().is_free());
        
        assert_eq!(player.get_piece(0).borrow_mut().color(), pieces::Color::Green);
        assert_eq!(player.get_piece(1).borrow_mut().color(), pieces::Color::Green);
    }

    #[test]
    fn leaving_other_pieces_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(0, board);

        player.free_piece(0);
        player.free_piece(1);

        player.update_outside(0, 0, 1);
        player.join(1, 0, 1);

        assert_eq!(player.get_piece(0).borrow_mut().position(), 1);
        assert_eq!(player.get_piece(1).borrow_mut().position(), 1);
        assert_eq!(player.board().borrow_mut().outside(1).pieces.len(), 2);
        assert!(player.get_piece(0).borrow_mut().is_free());
        assert!(player.get_piece(1).borrow_mut().is_free());

        player.leave(0, 1, 2);
        assert_eq!(player.get_piece(0).borrow_mut().position(), 2);
        assert_eq!(player.get_piece(1).borrow_mut().position(), 1);
        assert_eq!(player.board().borrow_mut().outside(2).pieces.len(), 1);
        assert_eq!(player.board().borrow_mut().outside(1).pieces.len(), 1);
        assert!(player.get_piece(0).borrow_mut().is_free());
        assert!(player.get_piece(1).borrow_mut().is_free());
    }

    #[test]
    fn all_pieces_at_same_place_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_ID, board);

        player.free_piece(0);
        player.free_piece(1);
        player.free_piece(2);
        player.free_piece(3);

        player.update_outside(0, 0, 6);
        player.join(1, 0, 6);
        player.join(2, 0, 6);
        player.join(3, 0, 6);

        assert_eq!(player.get_piece(0).borrow().position(), 6);
        assert_eq!(player.get_piece(1).borrow_mut().position(), 6);
        assert_eq!(player.get_piece(2).borrow_mut().position(), 6);
        assert_eq!(player.get_piece(3).borrow_mut().position(), 6);

        assert!(player.get_piece(0).borrow_mut().is_free());
        assert!(player.get_piece(1).borrow_mut().is_free());
        assert!(player.get_piece(2).borrow_mut().is_free());
        assert!(player.get_piece(3).borrow_mut().is_free());

        player.leave(0, 6, 7);
        assert_eq!(player.get_piece(0).borrow().position(), 7);
        assert!(player.get_piece(3).borrow_mut().is_free());

        player.leave(1, 6, 9);
        assert_eq!(player.get_piece(1).borrow_mut().position(), 9);
        assert!(player.get_piece(1).borrow_mut().is_free());

        player.leave(2, 6, 10);
        assert_eq!(player.get_piece(2).borrow_mut().position(), 10);
        assert!(player.get_piece(2).borrow_mut().is_free());
    }

    #[test]
    fn all_pieces_in_goal_test_0() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(0, board);

        player.free_piece(0);
        player.free_piece(1);
        player.free_piece(2);
        player.free_piece(3);

        player.enter_goal(0, 0);
        player.enter_goal(1, 0);
        player.enter_goal(2, 0);
        player.enter_goal(3, 0);

        assert!(player.get_piece(0).borrow_mut().is_goal());
        assert!(player.get_piece(1).borrow_mut().is_goal());
        assert!(player.get_piece(2).borrow_mut().is_goal());
        assert!(player.get_piece(3).borrow_mut().is_goal());
        assert!(player
            .board()
            .borrow_mut()
            .goal(0)
            .piece(0)
            .borrow_mut()
            .is_goal());
        assert!(player
            .board()
            .borrow_mut()
            .goal(0)
            .piece(1)
            .borrow_mut()
            .is_goal());
        assert!(player
            .board()
            .borrow_mut()
            .goal(0)
            .piece(2)
            .borrow_mut()
            .is_goal());
        assert!(player
            .board()
            .borrow_mut()
            .goal(0)
            .piece(3)
            .borrow_mut()
            .is_goal());
        assert_eq!(player.board().borrow_mut().goal(0).pieces.len(), 4);
        assert!(player.is_finished());
    }

    #[test]
    fn star_join_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(0, board);
        player.free_piece(0);
        player.free_piece(1);

        player.starjump(0, 0, 5);
        assert_eq!(player.get_piece(0).borrow_mut().position(), 11);
        assert!(player.get_piece(0).borrow_mut().is_free());

        player.join_piece(1, 5);
        assert_eq!(player.get_piece(1).borrow_mut().position(), 11);
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
            .is_free());
        assert!(player
            .board()
            .borrow_mut()
            .outside(11)
            .piece(1)
            .borrow_mut()
            .is_free());
        assert!(player.get_piece(0).borrow_mut().is_free());
        assert!(player.get_piece(1).borrow_mut().is_free());
    }
}
