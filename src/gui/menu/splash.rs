use bevy::prelude::*;
use crate::GameState;

pub struct SplashPlugin;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

impl Default for SplashTimer {
    fn default() -> Self {
	SplashTimer(Timer::from_seconds(3.0, TimerMode::Once))
    }
}

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
	app.init_resource::<SplashTimer>()
	    .add_systems(OnEnter(GameState::Splash), setup_splash)
	    .add_systems(Update, splash_countdown.run_if(in_state(GameState::Splash)));
    }
}

fn setup_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("icon/logo-highres.png");

    commands.spawn((
	(
	    BackgroundColor(Srgba::hex("F3F3E8").unwrap().into()),
	    Node {
		width: Val::Vw(100.),
		height: Val::Vh(100.),
		display: Display::Flex,
		align_items: AlignItems::Center,
		justify_content: JustifyContent::Center,
		..default()
	    }),
	StateScoped(GameState::Splash),
    )).with_children(|parent| {
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

fn splash_countdown(
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if timer.tick(time.delta()).finished() {
	game_state.set(GameState::InGame);
    }
}
