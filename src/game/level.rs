use crate::screens::Screen;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_pancam::PanCam;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<LdktBundle>("LdtkEntityId")
        .add_systems(OnEnter(Screen::Gameplay), setup);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut cam: Single<&mut PanCam>) {
    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("levels/level0.ldtk").into(),
            ..Default::default()
        },
        Name::new("Level"),
        DespawnOnExit(Screen::Gameplay),
    ));
    cam.enabled = true;
}

#[derive(Default, Component)]
struct ComponentA;

#[derive(Default, Component)]
struct ComponentB;

#[derive(Default, Bundle, LdtkEntity)]
pub struct LdktBundle {
    a: ComponentA,
    b: ComponentB,
    #[sprite_sheet]
    sprite_sheet: Sprite,
}
