mod pieces {
    pub struct State {
        position: i8,
        home: bool,
        safe: bool,
        dangerous: bool,
        goal: bool,
    }

    pub struct Piece {
        id: u8,
        state: State,
    }

    impl Piece {
        pub fn new(id: u8) -> Piece {
            Piece {
                id,
                state: State {
                    home: true,
                    goal: false,
                    safe: true,
                    dangerous: false,
                    position: -1,
                },
            }
        }

        pub fn id(&self) -> u8 {
            self.id
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
            self.state.position = position;
        }

        pub fn free(&mut self) {
            self.state.position = 0;
            self.dangerous()
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
}

pub use pieces::Piece;
