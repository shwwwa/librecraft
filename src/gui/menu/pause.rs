use bevy::{prelude::*, ui::FocusPolicy};

use crate::settings::Settings;
use crate::{GameState, game::player::Player, gui::GUIState};
use crate::{assets::RuntimeAsset, consts};

/** Marker to find pause menu background entity. */
#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub enum PauseButtonAction {
    Resume,
    Options,
    Exit,
}

/** Creates and setups pause menu. */
pub fn setup_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    runtime_asset: Res<RuntimeAsset>,
    gui_state: Res<State<GUIState>>,
    player: Res<Player>,
    settings: Res<Settings>,
) {
    let text_font_16 = TextFont {
	font: asset_server.load(runtime_asset.font_path.clone()),
        font_size: 16.,
        ..default()
    };
    let text_font_14 = text_font_16.clone().with_font_size(14.);

    let vis_state = match gui_state.get() {
        GUIState::Opened => Visibility::Visible,
        GUIState::Closed | GUIState::Typing => Visibility::Hidden,
    };

    let pause_menu = commands
        .spawn((
            PauseMenu,
            Name::new("PauseMenu"),
            vis_state,
            FocusPolicy::Block,
            StateScoped(GameState::InGame),
            (
                BackgroundColor(Color::BLACK.with_alpha(0.6)),
                GlobalZIndex(15),
                (Node {
                    width: Val::Vw(100.),
                    height: Val::Vh(100.),
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },),
            ),
        ))
        .id();

    let pause_left_corner_root = commands
        .spawn(Node {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            left: Val::Percent(1.),
            bottom: Val::Percent(1.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceAround,
            ..default()
        })
        .id();

    let data_version_text = commands
        .spawn((
            Text::new("Data version: "),
            text_font_14.clone(),
            TextColor(Color::WHITE),
        ))
        .with_child((
            TextSpan::new(player.data_version.to_string()),
            text_font_14.clone(),
            TextColor(Color::WHITE),
        ))
        .id();

    let dimension_text = commands
        .spawn((
            Text::new("Dimension: "),
            text_font_14.clone(),
            TextColor(Color::WHITE),
        ))
        .with_child((
            TextSpan::new(player.dimension.clone()),
            text_font_14.clone(),
            TextColor(Color::WHITE),
        ))
        .id();

    let score_text = commands
        .spawn((
            Text::new("Score: "),
            text_font_14.clone(),
            TextColor(Color::WHITE),
        ))
        .with_child((
            TextSpan::new(player.score.to_string()),
            text_font_14.clone(),
            TextColor(Color::WHITE),
        ))
        .id();

    let pause_right_corner_root = commands
        .spawn(Node {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            right: Val::Percent(1.),
            bottom: Val::Percent(1.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceAround,
            ..default()
        })
        .id();

    let about_text = commands
        .spawn((
            Text::new(consts::about!().to_string()),
            text_font_14.clone(),
            TextColor(Color::WHITE),
        ))
        .id();

    let pause_player_name_root = commands
        .spawn(Node {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            top: Val::Percent(1.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceAround,
            ..default()
        })
        .id();

    let player_name_text = commands
        .spawn((
            Text::new(settings.player_name.clone()),
            text_font_14.clone(),
            TextColor(Color::WHITE),
        ))
        .id();

    let pause_gui_root = commands
        .spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceAround,
            height: Val::Vh(40.),
            min_width: Val::Vw(40.),
            ..default()
        })
        .id();

    let pause_text = commands
        .spawn((
            Text::new("Pause menu"),
            text_font_16.clone(),
            TextColor(Color::WHITE),
        ))
        .id();

    commands.entity(pause_menu).add_children(&[
        pause_gui_root,
        pause_left_corner_root,
        pause_right_corner_root,
        pause_player_name_root,
    ]);

    commands.entity(pause_left_corner_root).add_children(&[
        score_text,
        dimension_text,
        data_version_text,
    ]);

    commands
        .entity(pause_right_corner_root)
        .add_children(&[about_text]);

    commands
        .entity(pause_player_name_root)
        .add_children(&[player_name_text]);

    commands.entity(pause_gui_root).add_children(&[pause_text]);
    commands.entity(pause_gui_root).with_children(|wrapper| {
        for (msg, action) in [
            ("Resume", PauseButtonAction::Resume),
            ("Options", PauseButtonAction::Options),
            ("Exit", PauseButtonAction::Exit),
        ] {
            wrapper
                .spawn((
                    action,
                    (
                        Button,
                        Node {
                            width: Val::Percent(100.),
                            border: UiRect::all(Val::Px(3.)),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::all(Val::Px(7.)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                        BorderColor(Color::BLACK),
                    ),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(msg),
                        text_font_16.clone(),
                        TextColor(Color::WHITE),
                    ));
                });
        }
    });
}

/** Renders pause menu on request */
pub fn render_pause_menu(
    keys: Res<ButtonInput<KeyCode>>,
    mut button: Query<(&PauseButtonAction, &mut BorderColor, &Interaction)>,
    mut visibility: Query<&mut Visibility, With<PauseMenu>>,
    gui_state: Res<State<GUIState>>,
    mut gui_state_reader: EventReader<StateTransitionEvent<GUIState>>,
    mut next_gui_state: ResMut<NextState<GUIState>>,
    mut exit: EventWriter<AppExit>,
) {
    let mut vis = visibility.single_mut();

    for ev in gui_state_reader.read() {
        if GUIState::Closed == ev.entered.unwrap() {
            *vis = Visibility::Hidden;
        } else {
            *vis = Visibility::Visible;
        }
    }

    if keys.just_pressed(KeyCode::Escape) {
        let is_closed: bool = *gui_state.get() == GUIState::Closed;

        if is_closed {
            next_gui_state.set(GUIState::Opened);
        } else {
            next_gui_state.set(GUIState::Closed);
        }

        let state = if is_closed { "opened" } else { "closed" };
        info!("Pause menu was {} (via key).", state);
    }

    if *vis != Visibility::Visible {
        return;
    }

    for (action, mut bcolor, interaction) in button.iter_mut() {
        match *interaction {
            Interaction::Pressed => match *action {
                PauseButtonAction::Resume => {
                    next_gui_state.set(GUIState::Closed);
                    info!("Resuming game.");
                }
                PauseButtonAction::Options => {
                    info!("todo!(options)");
                }
                PauseButtonAction::Exit => {
                    exit.send(AppExit::Success);
                }
            },
            Interaction::Hovered => {
                bcolor.0 = Color::WHITE;
            }
            Interaction::None => {
                bcolor.0 = Color::BLACK;
            }
        }
    }
}
