use tic_tac_toe::game::Game;

#[test]
fn new_game_has_empty_grid() {
    let game: Game = Game::new();
    let is_grid_empty = game
        .grid
        .iter()
        .flatten()
        .all(|tile| tile.clone().is_none());

    assert!(is_grid_empty);
}
