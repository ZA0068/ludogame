use players::Player;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_player_test() {
        let player = Player::new(0);
        assert_eq!(player.id(), 0);
        
    }

    #[test]
    fn get_pieces_test()
    {
        let player = Player::new(0);
        let pieces = player.pieces();
        assert_eq!(pieces.len(), 4);
    }
}