use pieces::Piece;

#[cfg(test)]
mod pieces_test {
    use super::*;

    #[test]
    fn creating_a_piece_test() {
        let piece = Piece::new(1);
        assert_eq!(piece.id(), 1);
        assert_eq!(piece.is_home(), true);
        assert_eq!(piece.is_goal(), false);
        assert_eq!(piece.is_safe(), true);
        assert_eq!(piece.is_dangerous(), false);
        assert_eq!(piece.position(), -1);
    }

    #[test]
    fn move_piece_test() {
        let mut piece = Piece::new(1);
        piece.set_position(3);
        assert_eq!(piece.id(), 1);
        assert_eq!(piece.position(), 3);
    }

    #[test]
    fn free_piece_test() {
        let mut piece = Piece::new(1);
        assert_eq!(piece.id(), 1);
        piece.free();
        assert_eq!(piece.position(), 0);
        assert_eq!(piece.is_home(), false);
        assert_eq!(piece.is_goal(), false);
        assert_eq!(piece.is_safe(), true);
        assert_eq!(piece.is_dangerous(), false);
    }

    #[test]
    fn death_pieces_test()
    {
        let mut piece = Piece::new(1);
        assert_eq!(piece.id(), 1);
        piece.dead();
        assert_eq!(piece.position(), -1);
        assert_eq!(piece.is_home(), true);
        assert_eq!(piece.is_goal(), false);
        assert_eq!(piece.is_safe(), true);
        assert_eq!(piece.is_dangerous(), false);
    }
}