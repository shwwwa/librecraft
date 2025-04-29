use bevy::prelude::*;
use bevy::audio::{Volume, PlaybackMode};
use bevy::window::{PrimaryWindow, WindowFocused};
use rand::prelude::*;

use crate::assets;

/** Used to get references for all music in-game e.g. to mute it. */
#[derive(Component)]
pub struct Music;

#[derive(Resource)]
pub struct SoundtrackPlayer {
    track_list: Vec<Handle<AudioSource>>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct SoundtrackTimer(Timer);

#[derive(Component)]
pub struct FadeIn;

#[derive(Component)]
pub struct FadeOut;

pub const FADE_TIME: f32 = 2.0;

impl SoundtrackPlayer {
    fn new(track_list: Vec<Handle<AudioSource>>) -> Self {
        Self { track_list }
    }
}

pub fn setup_soundtrack(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(SoundtrackTimer(Timer::from_seconds(
        20.0,
        TimerMode::Repeating,
    )));

    let calm1 = asset_server.load::<AudioSource>(assets::CALM1_PATH);
    let calm2 = asset_server.load::<AudioSource>(assets::CALM2_PATH);
    let calm3 = asset_server.load::<AudioSource>(assets::CALM3_PATH);

    let hal1 = asset_server.load::<AudioSource>(assets::HAL1_PATH);
    let hal2 = asset_server.load::<AudioSource>(assets::HAL2_PATH);
    let hal3 = asset_server.load::<AudioSource>(assets::HAL3_PATH);
    let hal4 = asset_server.load::<AudioSource>(assets::HAL4_PATH);

    let nuance1 = asset_server.load::<AudioSource>(assets::NUANCE1_PATH);
    let nuance2 = asset_server.load::<AudioSource>(assets::NUANCE2_PATH);

    let piano1 = asset_server.load::<AudioSource>(assets::PIANO1_PATH);
    let piano2 = asset_server.load::<AudioSource>(assets::PIANO2_PATH);
    let piano3 = asset_server.load::<AudioSource>(assets::PIANO3_PATH);

    let track_list = vec![
        calm1, calm2, calm3, hal1, hal2, hal3, hal4, nuance1, nuance2, piano1, piano2, piano3,
    ];

    commands.insert_resource(SoundtrackPlayer::new(track_list));
}

pub fn fade_in(
    mut commands: Commands,
    mut audio_sink_q: Query<(&mut AudioSink, Entity), With<FadeIn>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    for (mut audio, entity) in audio_sink_q.iter_mut() {
        for window in window_q.iter() {
            if !window.focused {
                commands.entity(entity).remove::<FadeIn>();
                return;
            }
        }

	let volume = Volume::Linear(audio.volume().to_linear() + time.delta_secs() / FADE_TIME);
        audio.set_volume(volume);
	
        if audio.volume() >= Volume::Linear(1.0) {
            audio.set_volume(Volume::Linear(1.0));
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}

pub fn fade_out(
    mut commands: Commands,
    mut audio_sink_q: Query<(&mut AudioSink, Entity), With<FadeOut>>,
    time: Res<Time>,
) {
    for (mut audio, entity) in audio_sink_q.iter_mut() {

	let volume = Volume::Linear(audio.volume().to_linear() - time.delta_secs() / FADE_TIME);
        audio.set_volume(volume);
	
        if audio.volume() <= Volume::Linear(0.0) {
            info!("deleting fade out tracks");
            commands.entity(entity).despawn();
        }
    }
}

pub fn mute_music_on_focus(
    mut audio_sink_q: Query<&mut AudioSink, With<Music>>,
    mut focus_reader: EventReader<WindowFocused>,
) {
    for evr in focus_reader.read() {
        if evr.focused {
            for mut audio in audio_sink_q.iter_mut() {
                audio.set_volume(Volume::Linear(1.0));
            }
        } else {
            for mut audio in audio_sink_q.iter_mut() {
                audio.set_volume(Volume::SILENT);
            }
        }
    }
}

pub fn change_track(
    mut commands: Commands,
    mut timer: ResMut<SoundtrackTimer>,
    soundtrack_player: Res<SoundtrackPlayer>,
    soundtrack_q: Query<Entity, With<AudioSink>>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        // Fade out and despawn all currently running tracks
        for track in soundtrack_q.iter() {
            commands.entity(track).insert(FadeOut);
        }
        // todo: tracks cannot repeat themselves
        let chosen_track = soundtrack_player
            .track_list
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();
	
        info!("Next track: {:?}", chosen_track);
        // Volume is set to ZERO to make use of fade in system, which increments volume until FADE_TIME
        commands.spawn((
            AudioPlayer(chosen_track),
            PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::SILENT,
                ..default()
            },
            FadeIn,
            Music,
        ));
    }
}
