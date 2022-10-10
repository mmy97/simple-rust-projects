pub mod game {
    use Marker::O;
    use crate::game::CannotMark::{LocationNotEmpty, LocationOutOfBounds};
    use crate::game::Marker::X;
    use crate::game::Outcome::Winner;

    const GRID_SIZE: usize = 3;

    pub struct Game {
        pub grid: [[Option<Marker>; GRID_SIZE]; GRID_SIZE],
        pub outcome: Option<Outcome>,
        pub current_turn: Marker,
    }

    impl Game {
        pub fn new() -> Self {
            Game {
                grid: Default::default(),
                outcome: None,
                current_turn: X,
            }
        }

        pub fn mark(&mut self, x: usize, y: usize) -> Result<(), CannotMark> {
            if x >= GRID_SIZE || y >= GRID_SIZE {
                return Err(LocationOutOfBounds)
            }

            if self.grid[x][y].is_some() {
                return Err(LocationNotEmpty)
            }

            self.mark_grid(x, y);

            self.update_outcome();

            self.swap_turns();
            Ok(())
        }

        fn swap_turns(&mut self) {
            self.current_turn = match self.current_turn {
                X => O,
                O => X
            }
        }

        fn mark_grid(&mut self, x: usize, y: usize) {
            self.grid[x][y] = Some(self.current_turn);
        }

        fn update_outcome(&mut self) {
            let top_left = self.grid[0][0];
            let top_centre = self.grid[1][0];
            let top_right = self.grid[2][0];

            if top_left == top_centre && top_centre == top_right {
                self.outcome = Some(Winner(X));
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
    pub enum Outcome {
        Winner(Marker),
        Draw,
    }
}
