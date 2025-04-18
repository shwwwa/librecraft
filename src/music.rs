use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowFocused};
use rand::prelude::*;

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

    let calm1 = asset_server.load::<AudioSource>("music/calm1.ogg");
    let calm2 = asset_server.load::<AudioSource>("music/calm2.ogg");
    let calm3 = asset_server.load::<AudioSource>("music/calm3.ogg");

    let hal1 = asset_server.load::<AudioSource>("music/hal1.ogg");
    let hal2 = asset_server.load::<AudioSource>("music/hal2.ogg");
    let hal3 = asset_server.load::<AudioSource>("music/hal3.ogg");
    let hal4 = asset_server.load::<AudioSource>("music/hal4.ogg");

    let nuance1 = asset_server.load::<AudioSource>("music/nuance1.ogg");
    let nuance2 = asset_server.load::<AudioSource>("music/nuance2.ogg");

    let piano1 = asset_server.load::<AudioSource>("music/piano1.ogg");
    let piano2 = asset_server.load::<AudioSource>("music/piano2.ogg");
    let piano3 = asset_server.load::<AudioSource>("music/piano3.ogg");

    let track_list = vec![
        calm1, calm2, calm3, hal1, hal2, hal3, hal4, nuance1, nuance2, piano1, piano2, piano3,
    ];

    commands.insert_resource(SoundtrackPlayer::new(track_list));
}

pub fn fade_in(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeIn>>,
    window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    for (audio, entity) in audio_sink.iter_mut() {
        for pw in window.iter() {
            if !pw.focused {
                commands.entity(entity).remove::<FadeIn>();
                return;
            }
        }
        audio.set_volume(audio.volume() + time.delta_secs() / FADE_TIME);
        if audio.volume() >= 1.0 {
            audio.set_volume(1.0);
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}

pub fn fade_out(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeOut>>,
    time: Res<Time>,
) {
    for (audio, entity) in audio_sink.iter_mut() {
        audio.set_volume(audio.volume() - time.delta_secs() / FADE_TIME);
        if audio.volume() <= 0.0 {
            info!("deleting fade out tracks");
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn mute_music_on_focus(
    mut audio_sink: Query<&mut AudioSink, With<Music>>,
    mut focus_reader: EventReader<WindowFocused>,
) {
    for evr in focus_reader.read() {
        if evr.focused {
            for audio in audio_sink.iter_mut() {
                audio.set_volume(1.);
            }
        } else {
            for audio in audio_sink.iter_mut() {
                audio.set_volume(0.);
            }
        }
    }
}

pub fn change_track(
    mut commands: Commands,
    mut timer: ResMut<SoundtrackTimer>,
    soundtrack_player: Res<SoundtrackPlayer>,
    soundtrack: Query<Entity, With<AudioSink>>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        // Fade out and despawn all currently running tracks
        for track in soundtrack.iter() {
            commands.entity(track).insert(FadeOut);
        }
        // todo: tracks cannot repeat themselves
        let chosen_track = soundtrack_player
            .track_list
            .choose(&mut rand::rng())
            .unwrap()
            .clone();
        info!("Next track: {:?}", chosen_track);
        // Volume is set to ZERO to make use of fade in system, which increments volume until FADE_TIME
        commands.spawn((
            AudioPlayer(chosen_track),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: bevy::audio::Volume::ZERO,
                ..default()
            },
            FadeIn,
            Music,
        ));
    }
}
