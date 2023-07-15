use board::Board;

use players::Player;
use std::{cell::RefCell, rc::Rc};

#[cfg(test)]
mod atomic_multiplayers_tests {
    use super::*;

    const PLAYER_0: i8 = 0;
    const PLAYER_1: i8 = 1;
    const PLAYER_2: i8 = 2;
    const PLAYER_3: i8 = 3;

    #[test]
    fn add_all_player_test() {
        let board = Rc::new(RefCell::new(Board::new()));
        let board2 = Rc::new(RefCell::new(Board::new()));
        let mut player0 = Player::new(PLAYER_0);
        let mut player1 = Player::new(PLAYER_1);
        let mut player2 = Player::new(PLAYER_2);
        let mut player3 = Player::new(PLAYER_3);

        player0.setup(board.clone());
        player1.setup(board.clone());
        player2.setup(board.clone());
        player3.setup(board2.clone());

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
        let player = Player::new(4);
        assert_eq!(player.id(), 4);
    }

    #[test]
    fn two_players_free_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player1 = Player::new(PLAYER_0);
        player1.setup(board.clone());
        let mut player2 = Player::new(PLAYER_1);
        player2.setup(board.clone());

        player1.free_piece(0);
        player2.free_piece(0);

        assert_eq!(player1.piece(0).borrow().position(), 0);
        assert_eq!(player1.piece(0).borrow().color(), pieces::Color::Green);
        assert_eq!(player1.board().borrow().outside[0].pieces.len(), 1);
        assert_eq!(
            player1.board().borrow().outside[0].player_id,
            Some(board::PlayerID::Player0)
        );

        assert_eq!(player2.piece(0).borrow().position(), 13);
        assert_eq!(player2.piece(0).borrow().color(), pieces::Color::Yellow);
        assert_eq!(player2.board().borrow_mut().outside(13).pieces.len(), 1);
        assert_eq!(
            player2.board().borrow_mut().outside(13).player_id,
            Some(board::PlayerID::Player1)
        );
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
        let mut player1 = Player::new(PLAYER_0);
        player1.setup(board.clone());
        let mut player2 = Player::new(PLAYER_1);
        player2.setup(board.clone());
        let mut player3 = Player::new(PLAYER_2);
        player3.setup(board.clone());
        let mut player4 = Player::new(PLAYER_3);
        player4.setup(board.clone());

        player1.free_piece(0);
        player2.free_piece(0);
        player3.free_piece(0);
        player4.free_piece(0);

        assert_eq!(player1.piece(0).borrow().position(), 0);
        assert_eq!(player1.piece(0).borrow().color(), pieces::Color::Green);

        assert_eq!(player2.piece(0).borrow().position(), 13);
        assert_eq!(player2.piece(0).borrow().color(), pieces::Color::Yellow);

        assert_eq!(player3.piece(0).borrow().position(), 26);
        assert_eq!(player3.piece(0).borrow().color(), pieces::Color::Blue);

        assert_eq!(player4.piece(0).borrow().position(), 39);
        assert_eq!(player4.piece(0).borrow().color(), pieces::Color::Red);

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
        let mut player1 = Player::new(PLAYER_0);
        player1.setup(board.clone());
        let mut player2 = Player::new(PLAYER_1);
        player2.setup(board.clone());

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
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player1 = Player::new(PLAYER_0);
        player1.setup(board.clone());
        let mut player2 = Player::new(PLAYER_1);
        player2.setup(board.clone());
        let mut player3 = Player::new(PLAYER_2);
        player3.setup(board.clone());
        let mut player4 = Player::new(PLAYER_3);
        player4.setup(board.clone());

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
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player1 = Player::new(PLAYER_1);
        player1.setup(board.clone());
        player1.free_piece(0);
        player1.move_piece(0, 36);
        player1.move_piece(0, 2);
        assert_eq!(player1.piece(0).borrow().position(), 51);

        player1.free_piece(1);
        player1.move_piece(1, 36);
        player1.move_piece(1, 6);
        assert_eq!(player1.piece(1).borrow().position(), 3);

        let mut player2 = Player::new(PLAYER_2);
        player2.setup(board.clone());
        player2.free_piece(0);
        player2.move_piece(0, 23);
        player2.move_piece(0, 2);
        assert_eq!(player2.piece(0).borrow().position(), 51);

        player2.free_piece(1);
        player2.move_piece(1, 23);
        player2.move_piece(1, 6);
        assert_eq!(player2.piece(1).borrow().position(), 3);

        let mut player3 = Player::new(PLAYER_3);
        player3.setup(board.clone());
        player3.free_piece(0);
        player3.move_piece(0, 10);
        player3.move_piece(0, 2);
        assert_eq!(player3.piece(0).borrow().position(), 51);

        player3.free_piece(1);
        player3.move_piece(1, 10);
        player3.move_piece(1, 6);
        assert_eq!(player3.piece(1).borrow().position(), 3);
    }

    #[test]
    fn two_player_kill_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player = Player::new(PLAYER_0);
        player.setup(board.clone());
        let mut opponent = Player::new(PLAYER_1);
        opponent.setup(board.clone());

        player.free_piece(0);
        player.move_piece(0, 17);
        assert_eq!(player.piece(0).borrow().position(), 17);

        opponent.free_piece(0);
        assert_eq!(opponent.piece(0).borrow().position(), 13);

        opponent.kill(0, 13, 17);

        assert_eq!(player.piece(0).borrow().position(), -1);
        assert!(player.piece(0).borrow().is_home());
        assert_eq!(player.board().borrow_mut().home(0).pieces.len(), 4);
        assert_eq!(
            player
                .board()
                .borrow_mut()
                .home(0)
                .piece(0)
                .borrow_mut()
                .color(),
            pieces::Color::Green
        );
        assert!(player
            .board()
            .borrow_mut()
            .home(0)
            .piece(0)
            .borrow()
            .is_home());

        assert_eq!(opponent.piece(0).borrow().position(), 17);
        assert_eq!(
            player.board().borrow_mut().outside(17).player_id,
            Some(board::PlayerID::Player1)
        );
    }

    #[test]
    fn suicide_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player0 = Player::new(PLAYER_0);
        player0.setup(board.clone());
        let mut player1 = Player::new(PLAYER_1);
        player1.setup(board.clone());

        player0.free_piece(0);
        player0.free_piece(1);
        player0.move_piece(0, 17);
        player0.move_piece(1, 17);

        player1.free_piece(0);
        assert_eq!(player1.piece(0).borrow().position(), 13);

        player1.die(0);

        assert_eq!(player0.piece(0).borrow().position(), 17);
        assert!(!player0.piece(0).borrow().is_home());
        assert_eq!(
            player0
                .board()
                .borrow_mut()
                .outside(17)
                .piece(0)
                .borrow_mut()
                .position(),
            17
        );
        assert!(!player0
            .board()
            .borrow_mut()
            .outside(17)
            .piece(0)
            .borrow_mut()
            .is_home());
        assert_eq!(
            player0.board().borrow_mut().outside(17).player_id,
            Some(board::PlayerID::Player0)
        );

        assert_eq!(player1.piece(0).borrow().position(), -1);
        assert!(player1.piece(0).borrow().is_home());
        assert_eq!(
            player1
                .board()
                .borrow_mut()
                .home(PLAYER_1)
                .piece(0)
                .borrow_mut()
                .color(),
            pieces::Color::Yellow
        );

        assert_eq!(
            player1
                .board()
                .borrow_mut()
                .home(PLAYER_1)
                .piece(0)
                .borrow_mut()
                .position(),
            -1
        );
        assert!(player1
            .board()
            .borrow_mut()
            .home(PLAYER_1)
            .piece(0)
            .borrow_mut()
            .is_home());
        assert_eq!(player1.board().borrow_mut().home(1).pieces.len(), 4);
        assert_eq!(
            player1.board().borrow_mut().outside(17).player_id,
            Some(board::PlayerID::Player0)
        );
    }

    #[test]
    fn two_player_star_kill_tests() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player0 = Player::new(PLAYER_0);
        player0.setup(board.clone());
        let mut player1 = Player::new(PLAYER_1);
        player1.setup(board.clone());
        let piece_0 = 0;
        let piece_1 = 1;

        player0.free_piece(piece_0);
        player0.free_piece(piece_1);

        player0.move_piece(piece_0, 18);
        assert_eq!(player0.piece(piece_0).borrow().position(), 18);

        player0.move_piece(piece_1, 24);
        assert_eq!(player0.piece(piece_1).borrow().position(), 24);

        player1.free_piece(piece_0);
        player1.kill_piece(0, 5);

        assert_eq!(player0.piece(piece_0).borrow().position(), -1);
        assert_eq!(player0.piece(piece_1).borrow().position(), -1);
        assert_eq!(player0.board().borrow_mut().outside(18).pieces.len(), 0);
        assert_eq!(player1.board().borrow_mut().outside(18).player_id, None);
        assert!(player0.piece(piece_0).borrow().is_home());
        assert!(player0.piece(piece_1).borrow().is_home());

        assert_eq!(player1.piece(0).borrow().position(), 24);
        assert_eq!(player1.board().borrow_mut().outside(24).pieces.len(), 1);
        assert_eq!(
            player1.board().borrow_mut().outside(24).player_id,
            Some(board::PlayerID::Player1)
        );
    }

    #[test]
    fn star_sucide_test() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player0 = Player::new(PLAYER_0);
        player0.setup(board.clone());
        let mut player1 = Player::new(PLAYER_1);
        player1.setup(board.clone());
        let piece_0 = 0;
        let piece_1 = 1;

        player1.free_piece(piece_0);
        player1.free_piece(piece_1);
        player1.starjump(piece_0, 13, 18);
        player1.join(piece_1, 13, 24);
        assert_eq!(player1.piece(piece_0).borrow().position(), 24);
        assert_eq!(player1.piece(piece_1).borrow().position(), 24);

        player0.free_piece(piece_0);

        player0.update_outside(piece_0, 0, 17);

        player0.die(piece_0);

        assert_eq!(player1.piece(piece_0).borrow().position(), 24);
        assert_eq!(player1.piece(piece_1).borrow().position(), 24);
        assert_eq!(player1.board().borrow_mut().outside(24).pieces.len(), 2);
        assert_eq!(
            player1.board().borrow_mut().outside(24).player_id,
            Some(board::PlayerID::Player1)
        );

        assert!(player0.piece(piece_0).borrow().is_home());
        assert_eq!(player0.piece(piece_0).borrow().position(), -1);
        assert_eq!(player0.board().borrow_mut().home(PLAYER_0).pieces.len(), 4);
    }

    #[test]
    fn star_sucide_test_2() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player0 = Player::new(PLAYER_0);
        player0.setup(board.clone());
        let mut player1 = Player::new(PLAYER_1);
        player1.setup(board.clone());
        let piece_0 = 0;
        let piece_1 = 1;

        player0.free_piece(piece_0);
        player0.free_piece(piece_1);
        player0.update_outside(piece_0, 0, 18);
        player0.update_outside(piece_1, 0, 18);

        assert_eq!(player0.piece(piece_0).borrow().position(), 18);
        assert_eq!(player0.piece(piece_1).borrow().position(), 18);
        assert_eq!(player0.board().borrow_mut().outside(18).pieces.len(), 2);
        assert_eq!(
            player0.board().borrow_mut().outside(18).player_id,
            Some(board::PlayerID::Player0)
        );

        player1.free_piece(piece_0);
        player1.die(piece_0);

        assert!(player1.piece(piece_0).borrow().is_home());
        assert_eq!(player1.piece(piece_0).borrow().position(), -1);
        assert_eq!(player1.board().borrow_mut().home(PLAYER_1).pieces.len(), 4);
    }

    #[test]
    fn star_sucide_test_3() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player0 = Player::new(PLAYER_0);
        player0.setup(board.clone());
        let mut player1 = Player::new(PLAYER_1);
        player1.setup(board.clone());
        let piece_0 = 0;

        player0.free_piece(piece_0);
        player0.enter_globe(piece_0, 0, 21);

        assert_eq!(player0.piece(piece_0).borrow().position(), 21);
        assert_eq!(player0.board().borrow_mut().outside(21).pieces.len(), 1);
        assert_eq!(
            player0.board().borrow_mut().outside(21).player_id,
            Some(board::PlayerID::Player0)
        );

        player1.free_piece(piece_0);
        player1.die(piece_0);

        assert!(player1.piece(piece_0).borrow().is_home());
        assert_eq!(player1.piece(piece_0).borrow().position(), -1);
        assert_eq!(player1.board().borrow_mut().home(PLAYER_1).pieces.len(), 4);
    }

    #[test]
    fn star_sucide_test_4() {
        let board: Rc<RefCell<Board>> = Rc::new(RefCell::new(Board::new()));
        let mut player0 = Player::new(PLAYER_0);
        player0.setup(board.clone());
        let mut player1 = Player::new(PLAYER_1);
        player1.setup(board.clone());
        let piece_0 = 0;

        player0.free_piece(piece_0);
        player0.update_outside(piece_0, 0, 12);

        assert_eq!(player0.piece(piece_0).borrow().position(), 12);
        assert_eq!(player0.board().borrow_mut().outside(12).pieces.len(), 1);
        assert_eq!(
            player0.board().borrow_mut().outside(12).player_id,
            Some(board::PlayerID::Player0)
        );

        player1.free_piece(piece_0);
        player0.die(piece_0);

        assert!(player0.piece(piece_0).borrow().is_home());
        assert_eq!(player0.piece(piece_0).borrow().position(), -1);
        assert_eq!(player0.board().borrow_mut().home(PLAYER_0).pieces.len(), 4);
    }
}
