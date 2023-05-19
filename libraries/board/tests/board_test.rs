use board::{Board, BoardState, PlayerID};

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
        assert_eq!(TypeId::of::<BoardState>(), board_state.type_id());
        assert_eq!(board_state.position, 0);
        assert_eq!(board_state.number_of_pieces, 0);
        assert_eq!(board_state.player_id, None);
    }

    #[test]
    fn create_a_board_state_test_2() {
        let board_state = BoardState::create(-1, 1, Some(PlayerID::Player1));
        assert_eq!(TypeId::of::<BoardState>(), board_state.type_id());
        assert_eq!(board_state.position, -1);
        assert_eq!(board_state.number_of_pieces, 1);
        assert_eq!(board_state.player_id, Some(PlayerID::Player1));
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
        }

        for board_state in player1.iter() {
            assert_eq!(board_state.player_id, Some(PlayerID::Player1));
            assert_eq!(board_state.position, -1);
            assert_eq!(board_state.number_of_pieces, 1);
        }

        for board_state in player2.iter() {
            assert_eq!(board_state.player_id, Some(PlayerID::Player2));
            assert_eq!(board_state.position, -1);
            assert_eq!(board_state.number_of_pieces, 1);
        }

        for board_state in player3.iter() {
            assert_eq!(board_state.player_id, Some(PlayerID::Player3));
            assert_eq!(board_state.position, -1);
            assert_eq!(board_state.number_of_pieces, 1);
        }
    }

    // #[test]
    // #[ignore]
    // fn board_goal_spaces_test() {
    //     let board = Board::new();
    //     for state in board.goal().iter() {
    //         assert_eq!(state.player, Player::None);
    //         assert_eq!(state.number_of_pieces, 0);
    //     }
    // }

    // #[test]
    // #[ignore]
    // fn board_outside_spaces_test() {
    //     let board = Board::new();
    //     for state in board.outside().iter() {
    //         assert_eq!(state.player, Player::None);
    //         assert_eq!(state.number_of_pieces, 0);
    //     }
    // }

    // #[test]
    // #[ignore]
    // fn board_inside_spaces_test() {
    //     let board = Board::new();
    //     for state in board.inside().iter() {
    //         assert_eq!(state.player, Player::None);
    //         assert_eq!(state.number_of_pieces, 0);
    //     }
    // }

    // #[test]
    // #[ignore]
    // fn board_globe_spaces_test() {
    //     let board = Board::new();
    //     for state in board.globe().iter() {
    //         assert_eq!(state.player, Player::None);
    //         assert_eq!(state.number_of_pieces, 0);
    //     }
    // }

    // #[test]
    // #[ignore]
    // fn invincible_test() {
    //     let board = Board::new();
    //     for state in board.invincible().iter() {
    //         assert_eq!(state.player, Player::None);
    //         assert_eq!(state.number_of_pieces, 0);
    //     }
    // }

    // #[test]
    // #[ignore]
    // fn board_star_spaces_test() {
    //     let board = Board::new();
    //     for state in board.star().iter() {
    //         assert_eq!(state.player, Player::None);
    //         assert_eq!(state.number_of_pieces, 0);
    //     }
    // }
}

// #[cfg(test)]
// mod occupy_board_test {
//     use super::*;

//     #[test]
//     #[ignore]
//     fn occupy_board_space_test() {
//         let mut board = Board::new();
//         let space_number = 6;
//         board.occupy(space_number, Player::Player1);
//         assert_eq!(board.outside()[space_number as usize].player, Player::Player1);
//         assert_eq!(board.outside()[space_number as usize].number_of_pieces, 1);
//      }
// }
