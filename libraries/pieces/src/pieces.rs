mod pieces {

    #[derive(PartialEq, Debug, Clone)]
    pub enum State {
        Home,
        Free,
        Goal,
    }

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum Color {
        Green,
        Yellow,
        Blue,
        Red,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct Piece {
        id: i8,
        color: Color,
        state: State,
        position: i8,
    }

    impl Piece {
        pub fn new(id: i8, color: Color) -> Piece {
            Piece {
                id,
                color,
                position: -1,
                state: State::Home,
                }
            }

        pub fn id(&self) -> i8 {
            self.id
        }

        pub fn color(&self) -> Color {
            self.color.clone()
        }

        pub fn is_home(&self) -> bool {
            self.state == State::Home
        }

        pub fn is_free(&self) -> bool {
            self.state == State::Free
        }

        pub fn is_goal(&self) -> bool {
            self.state == State::Goal
        }

        pub fn position(&self) -> i8 {
            self.position
        }

        pub fn set_position(&mut self, position: i8) {
            assert!((-1..=71).contains(&position) && position != 99);
            self.position = position;
        }


        pub fn dead(&mut self) {
            self.home();
        }
        pub fn free(&mut self) {
            self.state = State::Free;
        }

        pub fn home(&mut self) {
            self.position = -1;
            self.state = State::Home;
        }

        pub fn goal(&mut self) {
            self.position = 99;
            self.state = State::Goal;
        }
    }

    impl Default for Piece {
        fn default() -> Piece {
            Piece {
                id: 0,
                color: Color::Red,
                position: -1,
                state: State::Home,
            }
        }
    }
}

pub use pieces::Color;
pub use pieces::Piece;
