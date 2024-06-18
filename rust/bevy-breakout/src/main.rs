use bevy::{input::*, math::{bounding::{BoundingCircle, BoundingVolume, IntersectsVolume}, *}, prelude::*, sprite::MaterialMesh2dBundle};

const PADDLE_START_Y: f32 = 0.0;
const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_SPEED: f32 = 500.0;

const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(-75.0, 75.0, 1.0);
const BALL_DIAMETER: f32 = 30.;
const BALL_SPEED: f32 = 400.;
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

// walls
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const WALL_THICKNESS: f32 = 10.;
const WALL_BLOCK_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
const WALL_BLOCK_HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
             (
                apply_velocity,
                move_paddle,
                check_ball_collision,
            ).chain(),
        )
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left   => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right  => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top    => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        assert!(WALL_BLOCK_HEIGHT > 0.0);
        assert!(WALL_BLOCK_WIDTH > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, WALL_BLOCK_HEIGHT + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(WALL_THICKNESS + WALL_BLOCK_WIDTH, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.),
                    scale: location.size().extend(0.),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // paddle
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(0., PADDLE_START_Y, 0.),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider,
    ));

    // ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(BALL_COLOR),
            transform: Transform::from_translation(BALL_STARTING_POSITION)
                .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
            ..default()
        },
        Ball,
        Velocity(BALL_INITIAL_DIRECTION.normalize() * BALL_SPEED),
    ));

    // walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}


fn move_paddle(
    input: Res<ButtonInput<KeyCode>>,
    time_step: Res<Time>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = query.single_mut();

    let mut direction = 0.0;
    if input.pressed(KeyCode::KeyA) {
        direction -= 1.0
    }
    if input.pressed(KeyCode::KeyD) {
        direction += 1.0
    }

    let mut new_x: f32 = paddle_transform.translation.x + direction * PADDLE_SPEED * time_step.delta_seconds();

    new_x = new_x.min(RIGHT_WALL - (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);
    new_x = new_x.max(LEFT_WALL + (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);

    paddle_transform.translation.x = new_x;
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.delta_seconds();
        transform.translation.y += velocity.y * time_step.delta_seconds();
    }
}

fn check_ball_collision(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<&Transform, With<Collider>>
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();

    for transform in &collider_query {
        let collision = collide_with_side(
            BoundingCircle::new(
                ball_transform.translation.truncate(),
                BALL_DIAMETER / 2.
            ), 
            bounding::Aabb2d::new(
                transform.translation.truncate(),
                transform.scale.truncate() / 2.,
            )
        );
        
        if let Some(collision) = collision {
            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left   => reflect_x = ball_velocity.x > 0.0,
                Collision::Right  => reflect_x = ball_velocity.x < 0.0,
                Collision::Top    => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
            }

            if reflect_x {
                ball_velocity.x *= -1.05;
            }
            if reflect_y {
                ball_velocity.y *= -1.05;
            }

            println!("Ball Velocity (x, y) = ({}, {})", ball_velocity.x, ball_velocity.y);
        }
    }
}

fn collide_with_side( ball: BoundingCircle, wall: bounding::Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest = wall.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    // println!("{:#?}", side);

    Some(side)
}