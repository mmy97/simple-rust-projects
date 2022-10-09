pub mod game {

    const GRID_SIZE: usize = 3;

    pub struct Game {
        pub grid: [[Option<Marker>; GRID_SIZE]; GRID_SIZE],
        pub winner: Option<Winner>,
        pub current_turn: Marker,
    }

    impl Game {
        pub fn new() -> Self {
            Game {
                grid: Default::default(),
                winner: None,
                current_turn: Marker::X,
            }
        }

        pub fn mark(&mut self, x: u8, y: u8) -> Result<(), InvalidChoice> {
            self.swap_turns();
            Ok(())
        }

        fn swap_turns(&mut self) {
            self.current_turn = match self.current_turn {
                Marker::X => Marker::O,
                Marker::O => Marker::X
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum InvalidChoice {

    }

    #[derive(Debug, PartialEq)]
    pub enum Marker {
        X,
        O,
    }

    #[derive(Debug, PartialEq)]
    pub enum Winner {
        O,
        X,
        Draw,
    }
}
