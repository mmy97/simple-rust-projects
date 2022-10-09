**API Design**

**1.0** 

Game

    - new() -> Game
    - mark(x,y) -> Result<Ok, Err>
    - state() -> State

State

    - grid: Grid
    - winner: Option<Winner>
    - current_turn: Mark

Grid

    - tiles: [[Mark; 3] ; 3]

Winner 

    Mark | Tie

Mark

    O | X
