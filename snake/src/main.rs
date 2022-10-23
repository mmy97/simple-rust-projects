use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState::Pressed;
use bevy::prelude::*;
use KeyCode::{Down, Left, Right, Up};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SnakePlugin)
        .run();
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MoveTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(spawn_snake)
            .add_system(move_snake)
            .add_system(take_keyboard_input);
    }
}

#[derive(Component)]
struct Snake;

#[derive(Component, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: i8,
    y: i8,
}

#[derive(Component, Debug)]
struct Body(Vec<Position>);

struct MoveTimer(Timer);

fn spawn_snake(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn()
        .insert(Snake)
        .insert(Direction::Right)
        .insert(Body(vec![
            Position { x: 0, y: 0 },
            Position { x: -1, y: 0 },
            Position { x: -2, y: 0 },
        ]));
}

fn move_snake(
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    mut query: Query<(&mut Body, &Direction), With<Snake>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let (mut body, direction): (Mut<Body>, &Direction) = query.single_mut();

        body.0.pop();

        let mut head = body.0.first_mut().unwrap().clone();

        match direction {
            Direction::Up => head.y += 1,
            Direction::Right => head.x += 1,
            Direction::Down => head.y -= 1,
            Direction::Left => head.x -= 1,
        }

        body.0.insert(0, head);
        println!("{:?}", body);
    }
}

fn take_keyboard_input(
    mut key_evr: EventReader<KeyboardInput>,
    mut query: Query<&mut Direction, With<Snake>>,
) {
    use bevy::input::ButtonState;

    let mut direction = query.single_mut();

    let key = key_evr
        .iter()
        .find(|ev| ev.state == Pressed)
        .map(|ev| ev.key_code)
        .flatten();

    match key {
        Some(KeyCode::Up) => *direction = Direction::Up,
        Some(KeyCode::Right) => *direction = Direction::Right,
        Some(KeyCode::Down) => *direction = Direction::Down,
        Some(KeyCode::Left) => *direction = Direction::Left,
        _ => {}
    };
}
