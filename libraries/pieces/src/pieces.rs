mod pieces {

    #[derive(PartialEq, Debug, Clone)]
    pub struct State {
        position: i8,
        home: bool,
        safe: bool,
        dangerous: bool,
        goal: bool,
    }

    #[derive(PartialEq, Debug, Clone)]

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
    }

    impl Piece {
        pub fn new(id: i8, color: Color) -> Piece {
            Piece {
                id,
                color,
                state: State {
                    home: true,
                    goal: false,
                    safe: true,
                    dangerous: false,
                    position: -1,
                },
            }
        }

        pub fn id(&self) -> i8 {
            self.id
        }

        pub fn color(&self) -> Color {
            self.color.clone()
        }

        pub fn is_home(&self) -> bool {
            self.state.home
        }

        pub fn is_goal(&self) -> bool {
            self.state.goal
        }

        pub fn is_safe(&self) -> bool {
            self.state.safe
        }

        pub fn is_dangerous(&self) -> bool {
            self.state.dangerous
        }

        pub fn position(&self) -> i8 {
            self.state.position
        }

        pub fn set_position(&mut self, position: i8) {
            assert!((-1..=71).contains(&position) && position != 99);
            self.state.position = position;
        }

        pub fn free(&mut self) {
            self.state.position = 0;
            self.dangerous();
        }

        pub fn dangerous(&mut self) {
            self.state.dangerous = true;
            self.state.safe = true;
            self.state.home = false;
            self.state.goal = false;
        }

        pub fn not_safe(&mut self) {
            self.state.dangerous = false;
            self.state.safe = false;
            self.state.home = false;
            self.state.goal = false;
        }

        pub fn dead(&mut self) {
            self.state.position = -1;
            self.state.home = true;
            self.state.goal = false;
            self.state.safe = true;
            self.state.dangerous = false;
        }

        pub fn home(&mut self) {
            self.dead();
        }

        pub fn safe(&mut self) {
            self.state.dangerous = false;
            self.state.safe = true;
            self.state.home = false;
            self.state.goal = false;
        }

        pub fn goal(&mut self) {
            self.state.position = 99;
            self.state.goal = true;
            self.state.safe = true;
            self.state.dangerous = false;
            self.state.home = false;
        }
    }

    impl Default for Piece {
        fn default() -> Piece {
            Piece {
                id: 0,
                color: Color::Red,
                state: State {
                    position: -1,
                    home: true,
                    goal: false,
                    safe: true,
                    dangerous: false,
                },
            }
        }
    }
}

pub use pieces::Color;
pub use pieces::Piece;
