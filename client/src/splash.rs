use crate::GameState;
use crate::assets;
use bevy::prelude::*;

//todo: when startup time will be bad, start loading resources in SplashPlugin

/** How much splash screen takes time until switching. */
const SPLASH_SECS: f32 = 2.0;

/** Plugin that renders splash screen for [`SPLASH_SECS`]. Dependent only on [`assets::SPLASH_PATH`] and [`GameState`]. */
pub struct SplashPlugin<S: States> {
    pub state: S,
}

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

impl Default for SplashTimer {
    fn default() -> Self {
        SplashTimer(Timer::from_seconds(SPLASH_SECS, TimerMode::Once))
    }
}

impl<S: States> Plugin for SplashPlugin<S> {
    fn build(&self, app: &mut App) {
        app.init_resource::<SplashTimer>()
            .add_systems(Startup, setup_splash.run_if(in_state(self.state.clone())))
            .add_systems(
                Update,
                splash_countdown.run_if(in_state(self.state.clone())),
            )
            .add_systems(OnExit(self.state.clone()), |mut commands: Commands| {
                commands.remove_resource::<SplashTimer>();
            });
    }
}

/** Setups splash screen with background icon. */
fn setup_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load(assets::SPLASH_PATH);
    commands
        .spawn((
            (
                BackgroundColor(Srgba::hex("F3F3E8").unwrap().into()),
                Node {
                    width: Val::Vw(100.),
                    height: Val::Vh(100.),
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ),
            StateScoped(GameState::Splash),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    height: Val::Vh(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ImageNode::new(icon),
            ));
        });
}

/** Splash countdown. Sets new game state when [`SplashTimer`] is expired. */
fn splash_countdown(
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::InGame);
    }
}
