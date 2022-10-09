pub mod game {
    use crate::game::CannotMark::{LocationNotEmpty, LocationOutOfBounds};

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

        pub fn mark(&mut self, x: usize, y: usize) -> Result<(), CannotMark> {
            if x >= GRID_SIZE || y >= GRID_SIZE {
                return Err(LocationOutOfBounds)
            }

            if self.grid[x][y].is_some() {
                return Err(LocationNotEmpty)
            }

            self.grid[x][y] = Some(self.current_turn);
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
    pub enum CannotMark {
        LocationOutOfBounds,
        LocationNotEmpty
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
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
