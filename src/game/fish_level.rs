use crate::{
    audio::sound_effect, menus::credits::CreditsAssets, screens::Screen,
    theme::interaction::InteractionAssets,
};
use bevy::{input::mouse::MouseButtonInput, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_pancam::PanCam;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<FishGrey>("Fish_grey")
        .register_ldtk_entity::<FishGold>("Fish_golden")
        .register_ldtk_entity::<Alga1x1Bundle>("Alga_1x1")
        .register_ldtk_entity::<Alga1x2Bundle>("Alga_1x2")
        .register_ldtk_entity::<Player>("Player")
        .add_systems(OnEnter(Screen::Gameplay), setup)
        .add_systems(
            Update,
            (
                on_player_spawn,
                player_control,
                on_fish_spawn,
                on_alga1_spawn,
                on_alga2_spawn,
            )
                .run_if(in_state(Screen::Gameplay)),
        )
        .add_systems(
            FixedUpdate,
            (update_ball, update_fish, alga_update).run_if(in_state(Screen::Gameplay)),
        );
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut cam: Single<&mut PanCam>,
    music: Res<CreditsAssets>,
) {
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
    commands.spawn((
        Name::new("Fish Level Music"),
        DespawnOnExit(Screen::Gameplay),
        crate::audio::music(music.music.clone()),
    ));
}

#[derive(Component)]
struct AlgaTimer(Timer);

impl Default for AlgaTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Once))
    }
}

#[derive(Default, Component)]
struct Alga1x1;

#[derive(Default, Bundle, LdtkEntity)]
pub struct Alga1x1Bundle {
    #[sprite]
    sprite: Sprite,
    alga: Alga1x1,
    marler: AlgaTimer,
}

#[derive(Default, Component)]
struct Alga1x2;

#[derive(Default, Bundle, LdtkEntity)]
pub struct Alga1x2Bundle {
    #[sprite]
    sprite: Sprite,
    alga: Alga1x2,
    marler: AlgaTimer,
}

#[derive(Default, Component, PartialEq)]
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

#[derive(Component)]
struct EatingTimer(Timer);

impl Default for EatingTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(5.0, TimerMode::Once))
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct FishGrey {
    #[sprite]
    sprite: Sprite,
    name: Name,
    despawn: DespawnOnExit<Screen>,
    fish: FishState,
    timer: EatingTimer,
}

impl Default for FishGrey {
    fn default() -> Self {
        Self {
            name: Name::new("Fish_grey"),
            sprite: Sprite::default(),
            despawn: DespawnOnExit(Screen::Gameplay),
            fish: FishState::default(),
            timer: EatingTimer::default(),
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
    timer: EatingTimer,
}

impl Default for FishGold {
    fn default() -> Self {
        Self {
            name: Name::new("Fish_golden"),
            sprite: Sprite::default(),
            despawn: DespawnOnExit(Screen::Gameplay),
            fish: FishState::default(),
            gold_marker: GoldMarker,
            timer: EatingTimer::default(),
        }
    }
}

fn update_fish(
    mut query: Query<
        (
            &mut FishState,
            &mut Transform,
            &mut EatingTimer,
            &GlobalTransform,
        ),
        Without<Ball>,
    >,
    ball_query: Query<(Entity, &GlobalTransform), With<Ball>>,
    mut commands: Commands,
    time: Res<Time>,
    sounds: If<Res<InteractionAssets>>,
) {
    for (mut state, mut tf, mut timer, gtf) in &mut query {
        // Update eating timer
        timer.0.tick(time.delta());

        let is_eating = matches!(*state, FishState::EatingLeft | FishState::EatingRight);
        let mut hit_by_ball = false;
        let mut going_faster = false;

        if !is_eating {
            // Check collision with balls
            for (ball_entity, ball_tf) in &ball_query {
                let distance = gtf.translation().distance(ball_tf.translation());
                if distance < 16.0 {
                    hit_by_ball = true;
                    commands.entity(ball_entity).try_despawn();
                    break;
                }

                // Check if ball is close enough to make the fish go faster
                if distance < 32.0 {
                    going_faster = true;
                }
            }
        }

        if hit_by_ball {
            *state = match *state {
                FishState::Idle => FishState::EatingLeft,
                FishState::SlowLeft => FishState::EatingLeft,
                FishState::FastLeft => FishState::EatingLeft,
                FishState::SlowRight => FishState::EatingRight,
                FishState::FastRight => FishState::EatingRight,
                FishState::EatingLeft => FishState::EatingLeft,
                FishState::EatingRight => FishState::EatingRight,
            };
            timer.0.reset();
            commands.spawn(sound_effect(sounds.click.clone()));
        } else {
            // Check if eating timer has finished
            if is_eating && timer.0.remaining().as_secs() == 0 {
                *state = if *state == FishState::EatingLeft {
                    FishState::SlowLeft
                } else {
                    FishState::SlowRight
                };
            }

            *state = match *state {
                FishState::Idle => FishState::SlowLeft,
                FishState::SlowLeft => {
                    tf.translation.x -= 1.0;
                    if tf.translation.x < 0.0 {
                        FishState::SlowRight
                    } else {
                        if going_faster {
                            FishState::FastLeft
                        } else {
                            FishState::SlowLeft
                        }
                    }
                }
                FishState::FastLeft => {
                    tf.translation.x -= 3.0;
                    if tf.translation.x < 0.0 {
                        FishState::SlowRight
                    } else {
                        if going_faster {
                            FishState::FastLeft
                        } else {
                            FishState::SlowLeft
                        }
                    }
                }
                FishState::SlowRight => {
                    tf.translation.x += 1.0;
                    if tf.translation.x > 384.0 {
                        FishState::SlowLeft
                    } else {
                        if going_faster {
                            FishState::FastRight
                        } else {
                            FishState::SlowRight
                        }
                    }
                }
                FishState::FastRight => {
                    tf.translation.x += 3.0;
                    if tf.translation.x > 384.0 {
                        FishState::SlowLeft
                    } else {
                        if going_faster {
                            FishState::FastRight
                        } else {
                            FishState::SlowRight
                        }
                    }
                }
                FishState::EatingRight => FishState::EatingRight,
                FishState::EatingLeft => FishState::EatingLeft,
            };
        }
    }
}

#[derive(Default, Component)]
struct PlayerMarker;

#[derive(Component)]
struct BallSpawnTimer(Timer);

impl Default for BallSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Once))
    }
}

#[derive(Bundle, LdtkEntity, Default)]
pub struct Player {
    #[sprite]
    sprite: Sprite,
    name: Name,
    despawn: DespawnOnExit<Screen>,
    marker: PlayerMarker,
    spawn_timer: BallSpawnTimer,
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
    player_query: Single<
        (&mut Transform, &GlobalTransform, &mut BallSpawnTimer),
        With<PlayerMarker>,
    >,
    mut mouse_button_input_reader: MessageReader<MouseButtonInput>,
    mut cursor_moved_reader: MessageReader<CursorMoved>,
    time: Res<Time>,
    sounds: If<Res<InteractionAssets>>,
) {
    let (mut player_tf, player_global_tf, mut spawn_timer) = player_query.into_inner();

    // Update spawn timer
    spawn_timer.0.tick(time.delta());

    for mouse_button_input in mouse_button_input_reader.read() {
        if mouse_button_input.state.is_pressed() && spawn_timer.0.remaining_secs() == 0.0 {
            let mut tf = Transform::from_translation(player_global_tf.translation());
            tf.translation.z = 10.0;
            commands.spawn(ball(meshes, materials, tf));
            commands.spawn(sound_effect(sounds.hover.clone()));
            spawn_timer.0.reset();
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
        player_tf.translation.x = cursor.unwrap().x + 192.;
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

fn on_fish_spawn(
    mut fish: Query<&mut Sprite, Added<FishState>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if fish.is_empty() {
        return;
    }
    let layout = TextureAtlasLayout::from_grid(UVec2::new(48, 16), 3, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    for mut sprite in &mut fish {
        sprite.texture_atlas = Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        });
    }
}

fn on_alga1_spawn(
    mut alga: Query<&mut Sprite, Added<Alga1x1>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if alga.is_empty() {
        return;
    }
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 1, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    for mut sprite in &mut alga {
        sprite.texture_atlas = Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        });
    }
}

fn on_alga2_spawn(
    mut alga: Query<&mut Sprite, Added<Alga1x2>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if alga.is_empty() {
        return;
    }
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 32), 1, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    for mut sprite in &mut alga {
        sprite.texture_atlas = Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        });
    }
}

fn alga_update(mut alga: Query<(&mut Sprite, &mut AlgaTimer)>, time: Res<Time>) {
    for (mut sprite, mut algatimer) in &mut alga {
        algatimer.0.tick(time.delta());
        if algatimer.0.remaining().as_millis() == 0 {
            algatimer.0.reset();
            sprite.texture_atlas.as_mut().and_then(|t| {
                t.index = (t.index + 1) % 2;
                Some(())
            });
        }
    }
}
