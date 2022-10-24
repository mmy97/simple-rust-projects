use bevy::ecs::query::QueryIter;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState::Pressed;
use bevy::prelude::*;
use KeyCode::{Down, Left, Right, Up};

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 500.0,
            height: 500.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        .add_plugin(SnakePlugin)
        .run();
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MoveTimer(Timer::from_seconds(0.3, true)))
            .add_startup_system(spawn_snake)
            .add_system(move_snake)
            .add_system(take_keyboard_input)
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new().with_system(position_translation),
            );
    }
}

#[derive(Component, Debug)]
struct Snake(i8);

#[derive(Component, Copy, Clone, Debug)]
struct Segment {
    position: Position,
    order: i8,
}

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

struct MoveTimer(Timer);

fn spawn_snake(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                ..default()
            },
            ..default()
        })
        .insert(Snake(1))
        .insert(Direction::Right)
        .insert(Segment {
            position: Position { x: 5, y: 3 },
            order: 0,
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.3),
                ..default()
            },
            ..default()
        })
        .insert(Snake(1))
        .insert(Segment {
            position: Position { x: 4, y: 3 },
            order: 1,
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.3),
                ..default()
            },
            ..default()
        })
        .insert(Snake(1))
        .insert(Segment {
            position: Position { x: 3, y: 3 },
            order: 2,
        });
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Segment, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (segment, mut transform) in q.iter_mut() {
        let pos = segment.position;
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
        transform.scale = Vec3::new(50.0, 50.0, 1.0);
    }
}

fn move_snake(
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    mut query: Query<(&Snake, &mut Segment, Option<&Direction>)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // let mut segments: Vec<(&Snake, Mut<Segment>, Option<&Direction>)> =
        //     query.iter_mut().collect();
        // segments.sort_by(|a, b| b.1.order.cmp(&a.1.order));

        let mut head: (&Snake, Mut<Segment>, Option<&Direction>) =
            query.iter_mut().find(|x| x.1.order == 0).unwrap();

        match head.2 {
            None => (),
            Some(Direction::Up) => head.1.position.y += 1,
            Some(Direction::Right) => head.1.position.x += 1,
            Some(Direction::Down) => head.1.position.y -= 1,
            Some(Direction::Left) => head.1.position.x -= 1,
        }

        println!("{:?}", head);
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
        Some(Up) => *direction = Direction::Up,
        Some(Right) => *direction = Direction::Right,
        Some(Down) => *direction = Direction::Down,
        Some(Left) => *direction = Direction::Left,
        _ => {}
    };
}
