use players::Player;

#[cfg(test)]
mod player_tests {
    use super::*;

    #[test]
    fn add_player_test() {
        let player = Player::new(0);
        assert_eq!(player.id(), 0);
    }

    #[test]
    fn get_pieces_test() {
        let mut player = Player::new(0);

        let piece = player.piece(0);
        assert_eq!(piece.id(), 0);

        let piece = player.piece(1);
        assert_eq!(piece.id(), 1);

        let piece = player.piece(2);
        assert_eq!(piece.id(), 2);

        let piece = player.piece(3);
        assert_eq!(piece.id(), 3);
    }

    #[test]
    fn get_piece_test() {
        let mut player = Player::new(0);
        let pieces = player.pieces();
        assert_eq!(pieces[0].is_home(), true);
        assert_eq!(pieces[1].is_home(), true);
        assert_eq!(pieces[2].is_home(), true);
        assert_eq!(pieces[3].is_home(), true);
        assert_eq!(player.piece(0).is_safe(), true);
    }

    #[test]
    fn free_piece_test() {
        let mut player = Player::new(0);
        player.free_piece(0);
        assert_eq!(player.piece(0).is_home(), false);
        assert_eq!(player.piece(0).is_safe(), true);
    }

    #[test]
    fn can_free_test() {
        let mut player = Player::new(0);
        assert_eq!(player.can_free(1, 0), false);
        assert_eq!(player.can_free(6, 0), true);
    }

    #[test]
    fn move_piece_test() {
        let mut player = Player::new(0);
        player.free_piece(0);
        assert_eq!(player.piece(0).is_home(), false);
        assert_eq!(player.piece(0).position(), 0);

        player.move_piece(0, 6);
        assert_eq!(player.piece(0).position(), 6);

        player.free_piece(1);
        player.move_piece(1, 6);
        player.move_piece(1, 2);
        assert_eq!(player.piece(1).position(), 8);
    }

    #[test]
    fn move_piece_inside_safe_test() {
        let mut player = Player::new(0);
        player.free_piece(0);
        player.piece(0).set_position(52);
        assert_eq!(player.piece(0).position(), 52);
        assert_eq!(player.piece(0).is_safe(), true);
        player.move_piece(0, 6);
        assert_eq!(player.piece(0).position(), 56);
    }

}

#[cfg(test)]
mod invalidation_tests {
    use super::*;

    #[test]
    fn invalid_move_test() {
        let player = Player::new(0);
        assert_eq!(player.is_move_valid(0, 5), false);
    }

    #[test]
    fn valid_move_test() {
        let mut player = Player::new(0);
        player.free_piece(0);
        assert_eq!(player.is_move_valid(0, 5), true);
    }
}

