use bevy::prelude::*;
use bevy_seedling::{
    SeedlingPlugin,
    pool::SamplerPool,
    prelude::{PoolLabel, RepeatMode, Volume, VolumeNode},
    sample::{AudioSample, SamplePlayer},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(SeedlingPlugin::default())
        .add_systems(
            Update,
            apply_global_volume.run_if(resource_changed::<GlobalVolume>),
        )
        .add_systems(Startup, setup);
}

#[derive(PoolLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct MusicPool;

fn setup(mut cmd: Commands) {
    cmd.spawn((Name::new("MusicPool"), SamplerPool(MusicPool)));
    cmd.spawn((Name::new("SoundPool"), SamplerPool(SoundPool)));
}

/// A music audio instance.
pub fn music(handle: Handle<AudioSample>) -> impl Bundle {
    (
        SamplePlayer {
            sample: handle,
            repeat_mode: RepeatMode::RepeatEndlessly,
            ..Default::default()
        },
        MusicPool,
    )
}

#[derive(PoolLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct SoundPool;

/// A sound effect audio instance.
pub fn sound_effect(handle: Handle<AudioSample>) -> impl Bundle {
    (SamplePlayer::new(handle), SoundPool)
}

/// [`GlobalVolume`] doesn't apply to already-running audio entities, so this system will update them.
fn apply_global_volume(global_volume: Res<GlobalVolume>, mut master: Single<&mut VolumeNode>) {
    master.volume = Volume::Linear(global_volume.volume.to_linear());
}
