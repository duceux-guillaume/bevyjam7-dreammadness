use crate::screens::Screen;
use bevy::{input::mouse::MouseButtonInput, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_pancam::PanCam;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<FishGrey>("Fish_grey")
        .register_ldtk_entity::<FishGold>("Fish_golden")
        .register_ldtk_entity::<LdtkEntityBundle>("Alga_1x1")
        .register_ldtk_entity::<LdtkEntityBundle>("Alga_1x2")
        .register_ldtk_entity::<Player>("Player")
        .add_systems(OnEnter(Screen::Gameplay), setup)
        .add_systems(
            Update,
            (on_player_spawn, player_control).run_if(in_state(Screen::Gameplay)),
        )
        .add_systems(
            FixedUpdate,
            (update_ball, update_fish).run_if(in_state(Screen::Gameplay)),
        );
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut cam: Single<&mut PanCam>) {
    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("levels/level_1.ldtk").into(),
            transform: Transform::from_translation(Vec3::new(-192., -216., 0.0)),
            ..Default::default()
        },
        Name::new("Level"),
        DespawnOnExit(Screen::Gameplay),
    ));
    cam.enabled = true;
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct LdtkEntityBundle {
    #[sprite]
    sprite: Sprite,
}

#[derive(Default, Component)]
enum FishState {
    #[default]
    Idle,
    SlowLeft,
    FastLeft,
    SlowRight,
    FastRight,
    EatingLeft,
    EatingRight,
}

#[derive(Bundle, LdtkEntity)]
pub struct FishGrey {
    #[sprite]
    sprite: Sprite,
    name: Name,
    despawn: DespawnOnExit<Screen>,
    fish: FishState,
}

impl Default for FishGrey {
    fn default() -> Self {
        Self {
            name: Name::new("Fish_grey"),
            sprite: Sprite::default(),
            despawn: DespawnOnExit(Screen::Gameplay),
            fish: FishState::default(),
        }
    }
}

#[derive(Default, Component)]
struct GoldMarker;

#[derive(Bundle, LdtkEntity)]
pub struct FishGold {
    #[sprite]
    sprite: Sprite,
    name: Name,
    despawn: DespawnOnExit<Screen>,
    fish: FishState,
    gold_marker: GoldMarker,
}

impl Default for FishGold {
    fn default() -> Self {
        Self {
            name: Name::new("Fish_golden"),
            sprite: Sprite::default(),
            despawn: DespawnOnExit(Screen::Gameplay),
            fish: FishState::default(),
            gold_marker: GoldMarker,
        }
    }
}

fn update_fish(mut query: Query<(&mut FishState, &mut Transform)>) {
    for (mut state, mut tf) in &mut query {
        *state = match *state {
            FishState::Idle => FishState::SlowLeft,
            FishState::SlowLeft => {
                tf.translation.x -= 1.0;
                if tf.translation.x < 0.0 {
                    FishState::SlowRight
                } else {
                    FishState::SlowLeft
                }
            }
            FishState::FastLeft => FishState::SlowRight,
            FishState::SlowRight => {
                tf.translation.x += 1.0;
                if tf.translation.x > 384.0 {
                    FishState::SlowLeft
                } else {
                    FishState::SlowRight
                }
            }
            FishState::FastRight => FishState::EatingRight,
            FishState::EatingRight => FishState::EatingLeft,
            FishState::EatingLeft => FishState::Idle,
        };
    }
}

#[derive(Default, Component)]
struct PlayerMarker;

#[derive(Bundle, LdtkEntity, Default)]
pub struct Player {
    #[sprite]
    sprite: Sprite,
    name: Name,
    despawn: DespawnOnExit<Screen>,
    marker: PlayerMarker,
}

fn on_player_spawn(
    mut player: Single<(&mut Sprite, &mut Name), Added<PlayerMarker>>,
    server: Res<AssetServer>,
) {
    player.0.image = server.load("images/player.png");
    player.0.custom_size = Some(Vec2::splat(32.));
    *player.1 = Name::new("Player");
}

#[derive(Default, Component)]
struct Ball;

fn ball(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    tf: Transform,
) -> impl Bundle {
    (
        Mesh2d(meshes.add(Circle::new(4.))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::BLACK))),
        tf,
        Ball,
        Name::new("Ball"),
        DespawnOnExit(Screen::Gameplay),
    )
}

fn player_control(
    camera: Single<(&Camera, &GlobalTransform), Without<PlayerMarker>>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut player_tf: Single<(&mut Transform, &mut GlobalTransform), With<PlayerMarker>>,
    mut mouse_button_input_reader: MessageReader<MouseButtonInput>,
    mut cursor_moved_reader: MessageReader<CursorMoved>,
) {
    for mouse_button_input in mouse_button_input_reader.read() {
        if mouse_button_input.state.is_pressed() {
            let mut tf = Transform::from_translation(player_tf.1.translation());
            tf.translation.z = 10.0;
            commands.spawn(ball(meshes, materials, tf));
            break;
        }
    }

    for cursor_moved in cursor_moved_reader.read() {
        let cursor = camera
            .0
            .viewport_to_world(camera.1, cursor_moved.position)
            .map(|ray| ray.origin.truncate());
        if cursor.is_err() {
            continue;
        }
        if cursor.unwrap().x < -192.0 || cursor.unwrap().x > 192.0 {
            continue;
        }
        player_tf.0.translation.x = cursor.unwrap().x + 192.;
    }
}

fn update_ball(mut commands: Commands, mut query: Query<(Entity, &mut Transform), With<Ball>>) {
    for (entity, mut tf) in &mut query {
        tf.translation.y -= 1.0;
        if tf.translation.y < -216.0 {
            commands.entity(entity).try_despawn();
        }
    }
}
