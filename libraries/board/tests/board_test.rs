use board::{Board, BoardState, PlayerID, State};
use std::any::{Any, TypeId};

#[cfg(test)]
mod board_tests {

    use super::*;

    #[test]
    fn create_a_board_test() {
        let board = Board::new();
        assert_eq!(TypeId::of::<Board>(), board.type_id());
    }

    #[test]
    fn create_default_board_test()
    {
        let board = Board::default();
        assert_eq!(TypeId::of::<Board>(), board.type_id());
    }

    #[test]
    fn create_a_board_state_test() {
        let board_state = BoardState::new();
        assert_eq!(board_state.position, -1);
        assert_eq!(board_state.number_of_pieces, 0);
        assert_eq!(board_state.player_id, None);
        assert_eq!(board_state.state, State::Home);
    }
    #[test]
    fn create_a_board_state_test_2() {
        let board_state = BoardState::create(-1, 1, Some(PlayerID::Player1), State::Home);
        assert_eq!(TypeId::of::<BoardState>(), board_state.type_id());
        assert_eq!(board_state.position, -1);
        assert_eq!(board_state.number_of_pieces, 1);
        assert_eq!(board_state.player_id, Some(PlayerID::Player1));
        assert_eq!(board_state.state, State::Home);
    }

    #[test]
    fn board_home_spaces_test() {
        let board = Board::new();

        let player0 = board.home[0];
        let player1 = board.home[1];
        let player2 = board.home[2];
        let player3 = board.home[3];

        assert_eq!(player0.player_id, Some(PlayerID::Player0));
        assert_eq!(player0.position, -1);
        assert_eq!(player0.number_of_pieces, 4);
        assert_eq!(player0.state, State::Home);

        assert_eq!(player1.player_id, Some(PlayerID::Player1));
        assert_eq!(player1.position, -1);
        assert_eq!(player1.number_of_pieces, 4);
        assert_eq!(player1.state, State::Home);

        assert_eq!(player2.player_id, Some(PlayerID::Player2));
        assert_eq!(player2.position, -1);
        assert_eq!(player2.number_of_pieces, 4);
        assert_eq!(player2.state, State::Home);

        assert_eq!(player3.player_id, Some(PlayerID::Player3));
        assert_eq!(player3.position, -1);
        assert_eq!(player3.number_of_pieces, 4);
        assert_eq!(player3.state, State::Home);
    }

    #[test]
    fn board_goal_spaces_test() {
        let board = Board::new();
        for i in 0..4 {
            let state = board.goal[i];
            assert_eq!(state.player_id, None);
            assert_eq!(state.number_of_pieces, 0);
            assert_eq!(state.position, 99);
            assert_eq!(state.state, State::Goal);
        }
    }

    #[test]
    fn board_outside_spaces_test() {
        let board = Board::new();
        for cnt in 0..52 {
            let state = board.outside[cnt];
            if state.state != State::Outside {
                continue;
            }
            assert_eq!(state.player_id, None);
            assert_eq!(state.number_of_pieces, 0);
            assert_eq!(state.position, cnt as i8);
            assert_eq!(state.state, State::Outside);
        }
    }

    #[test]
    fn board_inside_spaces_test() {
        let board = Board::new();
        for cnt in 0..20 {
            let pos = board.inside[cnt].position;
            assert!(board.inside(pos as usize).unwrap().position == pos);
        }
    }

    #[test]
    fn board_globe_spaces_test() {
        let board = Board::new();
        let vec = [8, 21, 34, 47];
        (0..4).for_each(|cnt| {
        assert!(board.is_globe(vec[cnt]));
        });
    }

    #[test]
    fn invincible_test() {
        let board = Board::new();
        (0..4).for_each(|cnt| {
            let pos = board.invincible[cnt];
            assert!(board.is_invincible(pos));
        });
    }

    #[test]
    fn board_star_spaces_test() {
        let board = Board::new();
        let stars = [5, 11, 18, 24, 31, 37, 44, 50];
        (0..8).for_each(|cnt| {
            assert!(board.is_star(stars[cnt]));
        });
    }
}
#[cfg(test)]
mod board_update_test {
    use super::*;

    #[test]
    fn move_from_home_test() {
        let mut board = Board::new();
        board.move_from_home(0, 0);
        assert_eq!(board.home[0].number_of_pieces, 3);
        assert_eq!(board.outside[0].number_of_pieces, 1);
        assert_eq!(board.outside[0].player_id, Some(PlayerID::Player0));
    }

    #[test]
    fn move_all_from_home_test() {
        let mut board = Board::new();
        for _ in 0..4 {
            board.move_from_home(0, 0);
        }
        assert_eq!(board.home[0].number_of_pieces, 0);
        assert_eq!(board.home[0].player_id, None);
        assert_eq!(board.outside[0].number_of_pieces, 4);
        assert_eq!(board.outside[0].player_id, Some(PlayerID::Player0));
    }

    #[test]
    fn move_into_home_test() {
        let mut board = Board::new();
        board.move_from_home(0, 0);
        assert_eq!(board.home[0].number_of_pieces, 3);
        assert_eq!(board.home[0].player_id, Some(PlayerID::Player0));
        assert_eq!(board.outside[0].number_of_pieces, 1);
        assert_eq!(board.outside[0].player_id, Some(PlayerID::Player0));

        board.move_into_home(0, 0);
        assert_eq!(board.home[0].number_of_pieces, 4);
        assert_eq!(board.home[0].player_id, Some(PlayerID::Player0));
        assert_eq!(board.outside[0].number_of_pieces, 0);
        assert_eq!(board.outside[0].player_id, None);
    }

    #[test]
    fn move_pieces_test() {
        let mut board = Board::new();
        board.move_from_home(0, 0);
        assert_eq!(board.outside[0].number_of_pieces, 1);
        assert_ne!(board.outside[1].number_of_pieces, 1);

        board.update_outside(0, 0, 1);
        assert_eq!(board.outside[1].number_of_pieces, 1);
        assert_ne!(board.outside[0].number_of_pieces, 1);

        board.update_outside(0, 1, 9);
        assert_eq!(board.outside[9].number_of_pieces, 1);
        assert_ne!(board.outside[1].number_of_pieces, 1);

        board.update_outside(0, 9, 10);
        assert_eq!(board.outside[10].number_of_pieces, 1);
    }

    #[test]
    fn move_pieces_test_2() {
        let mut board = Board::new();
        board.move_from_home(0, 0);
        assert_eq!(board.outside[0].number_of_pieces, 1);
        assert_ne!(board.outside[1].number_of_pieces, 1);

        board.update_outside(0, 0, 1);
        assert_eq!(board.outside[1].number_of_pieces, 1);
        assert_ne!(board.outside[0].number_of_pieces, 1);

        board.update_outside(0, 1, 9);
        assert_eq!(board.outside[9].number_of_pieces, 1);
        assert_ne!(board.outside[1].number_of_pieces, 1);

        board.update_outside(0, 9, 10);
        assert_eq!(board.outside[10].number_of_pieces, 1);
    }

    #[test]
    fn move_pieces_test_3() {
        let mut board = Board::new();
        board.move_from_home(0, 0);
        board.update_outside(0, 0, 51);
        board.move_inside(0, 51, 52);
            assert_eq!(board.inside[0].number_of_pieces, 1);
            assert_eq!(board.inside[0].position, 52);
            assert_eq!(board.inside[0].state, State::Inside);
        assert_eq!(board.outside[51].number_of_pieces, 0);
    }

    #[test]
    fn move_pieces_test_4() {
        let mut board = Board::new();
        board.move_from_home(0, 0);
        board.update_outside(0, 0, 51);
        board.move_inside(0, 51, 52);
            assert_eq!(board.inside[0].number_of_pieces, 1);
            assert_eq!(board.inside[0].player_id, Some(PlayerID::Player0));
        assert_eq!(board.outside[51].number_of_pieces, 0);
        assert_eq!(board.outside[51].player_id, None);
    }

    #[test]
    fn move_piece_test_enter_goal() {
        let mut board = Board::new();
        board.move_from_home(0, 0);
        board.update_outside(0, 0, 50);
        board.enter_goal(0, 50);

        assert_eq!(board.goal[0].number_of_pieces, 1);

        board.move_from_home(0, 50);
        board.move_inside(0, 50, 56);
            assert_eq!(board.inside[4].number_of_pieces, 1);
        board.enter_goal(0, 56);
        assert_eq!(board.goal[0].number_of_pieces, 2);
    }

    #[test]
    fn move_all_to_goal_test() {
        let mut board = Board::new();
        for i in 0..4 {
            for _ in 0..4 {
                board.move_from_home(i, 0);
                board.update_outside(i, 0, 50);
                board.enter_goal(i, 50);
            }
            assert_eq!(board.goal[i as usize].number_of_pieces, 4);
        }
    }

    #[test]
    fn is_occupied_test() {
        let mut board = Board::new();
        board.move_from_home(0, 0);
        board.update_outside(0, 0, 4);
        assert!(board.is_occupied(4));
        assert!(!board.is_occupied(0));

        board.update_outside(0, 4, 10);
        assert!(board.is_occupied(10));
        assert!(!board.is_occupied(4));
    }
}
