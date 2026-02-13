use crate::screens::Screen;
use bevy::prelude::*;
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
        .add_systems(Update, on_player_spawn.run_if(in_state(Screen::Gameplay)))
        .add_systems(FixedUpdate, update_fish.run_if(in_state(Screen::Gameplay)));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut cam: Single<&mut PanCam>) {
    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("levels/level_1.ldtk").into(),
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

fn on_player_spawn(mut sprite: Single<&mut Sprite, Added<PlayerMarker>>, server: Res<AssetServer>) {
    sprite.image = server.load("images/player.png");
    sprite.custom_size = Some(Vec2::splat(32.));
}
