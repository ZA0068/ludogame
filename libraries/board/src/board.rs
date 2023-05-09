mod board {
    pub struct Board {
        home: [i8; 16],
        goal: [i8; 4],
        outside: [i8; 52],
        inside: [i8; 20],
        globe: [i8; 4],
        invincible: [i8; 4],
        star: [i8; 8],
    }
    impl Default for Board {
        fn default() -> Self {
            Self::new()
        }
    }
    
    impl Board {
        pub fn new() -> Self {
            let outside: [i8; 52] = (0..52)
                .map(|i| i as i8)
                .collect::<Vec<i8>>()
                .try_into()
                .unwrap();
            let inside: [i8; 20] = (52..72)
                .map(|i| i as i8)
                .collect::<Vec<i8>>()
                .try_into()
                .unwrap();

            Self {
                home: [-1; 16],
                goal: [99; 4],
                outside,
                inside,
                globe: [8, 21, 34, 47],
                invincible: [0, 13, 26, 39],
                star: [5, 11, 18, 25, 31, 38, 44, 51],
            }
        }

        pub fn home(&self) -> [i8; 16] {
            self.home
        }

        pub fn goal(&self) -> [i8; 4] {
            self.goal
        }

        pub fn outside(&self) -> [i8; 52] {
            self.outside
        }

        pub fn inside(&self) -> [i8; 20] {
            self.inside
        }

        pub fn globe(&self) -> [i8; 4] {
            self.globe
        }

        pub fn invincible(&self) -> [i8; 4] {
            self.invincible
        }

        pub fn star(&self) -> [i8; 8] {
            self.star
        }
    }
}

pub use board::Board;
