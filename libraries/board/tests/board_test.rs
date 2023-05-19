use board::{Board, BoardState, PlayerID, State};

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};

    use super::*;

    #[test]
    fn create_a_board_test() {
        let board = Board::new();
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
        let home = board.home();
        assert!(home.len() == 16);

        let player0 = &home[0..4];
        let player1 = &home[4..8];
        let player2 = &home[8..12];
        let player3 = &home[12..16];

        for board_state in player0.iter() {
            assert_eq!(board_state.player_id, Some(PlayerID::Player0));
            assert_eq!(board_state.position, -1);
            assert_eq!(board_state.number_of_pieces, 1);
            assert_eq!(board_state.state, State::Home)
        }

        for board_state in player1.iter() {
            assert_eq!(board_state.player_id, Some(PlayerID::Player1));
            assert_eq!(board_state.position, -1);
            assert_eq!(board_state.number_of_pieces, 1);
            assert_eq!(board_state.state, State::Home)
        }

        for board_state in player2.iter() {
            assert_eq!(board_state.player_id, Some(PlayerID::Player2));
            assert_eq!(board_state.position, -1);
            assert_eq!(board_state.number_of_pieces, 1);
            assert_eq!(board_state.state, State::Home)
        }

        for board_state in player3.iter() {
            assert_eq!(board_state.player_id, Some(PlayerID::Player3));
            assert_eq!(board_state.position, -1);
            assert_eq!(board_state.number_of_pieces, 1);
            assert_eq!(board_state.state, State::Home)
        }
    }

    #[test]
    fn board_goal_spaces_test() {
        let board = Board::new();
        for state in board.goal().iter() {
            assert_eq!(state.player_id, None);
            assert_eq!(state.number_of_pieces, 0);
            assert_eq!(state.position, 99);
            assert_eq!(state.state, State::Goal);
        }
    }

    #[test]
    fn board_outside_spaces_test() {
        let board = Board::new();
        for (cnt, state) in board.outside().iter().enumerate() {
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
        for (cnt, state) in board.inside().iter().enumerate() {
            assert_eq!(state.player_id, None);
            assert_eq!(state.number_of_pieces, 0);
            assert_eq!(state.position, (cnt + 52) as i8);
            assert_eq!(state.state, State::Inside)
        }
    }

    #[test]
    fn board_globe_spaces_test() {
        let board = Board::new();
        let vec = [8, 21, 34, 47];
        for (cnt, state) in board.globe().iter().enumerate() {
            assert_eq!(state.player_id, None);
            assert_eq!(state.number_of_pieces, 0);
            assert_eq!(state.position, vec[cnt]);
        }
    }

    #[test]
    fn invincible_test() {
        let board = Board::new();
        let vec = [0, 13, 26, 39];
        for (cnt, state) in board.invincible().iter().enumerate() {
            assert_eq!(state.player_id, None);
            assert_eq!(state.number_of_pieces, 0);
            assert_eq!(state.position, vec[cnt]);
        }
    }

    #[test]
    fn board_star_spaces_test() {
        let board = Board::new();
        for state in board.star().iter() {
            assert_eq!(state.player_id, None);
            assert_eq!(state.number_of_pieces, 0);
        }
    } 
}
