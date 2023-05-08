mod board {
    pub struct Board {
        home: i8,
        goal: i8,
        outside: [i8; 52],
        inside: [i8; 20],
        globe: [i8; 8],
        star: [i8; 8],
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
                home: -1,
                goal: 99,
                outside,
                inside,
                globe: [0, 8, 13, 21, 26, 34, 39, 47],
                star: [5, 11, 18, 25, 31, 38, 44, 51],
            }
        }

        pub fn home(&self) -> i8 {
            self.home
        }

        pub fn goal(&self) -> i8 {
            self.goal
        }

        pub fn outside(&self) -> [i8; 52] {
            self.outside
        }

        pub fn inside(&self) -> [i8; 20] {
            self.inside
        }

        pub fn globe(&self) -> [i8; 8] {
            self.globe
        }

        pub fn star(&self) -> [i8; 8] {
            self.star
        }
    }
}

pub use board::Board;
