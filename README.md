**API Design**

**1.0** 

Game

    - grid: [[Option<Marker>; 3] ; 3]
    - winner: Option<Winner>
    - current_turn: Marker

    - factory new() -> Game
    - mark(x,y) -> Result<Ok, Err>
    - state() -> Game


Winner 

    O | X | Draw

Marker

    O | X
