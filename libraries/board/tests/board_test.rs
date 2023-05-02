use board;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_board_test() {
        let board = board::Board::new();
        assert_eq!(board.home(), [-1; 16]);
    }
}