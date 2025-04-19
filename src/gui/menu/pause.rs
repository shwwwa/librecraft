use bevy::{prelude::*, ui::FocusPolicy};

use crate::{gui::GUIState, GameState};

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
    gui_state: Res<State<GUIState>>,
) {
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
                GlobalZIndex(5),
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
            TextFont::from_font_size(16.0),
            TextColor(Color::WHITE),
        ))
        .id();

    commands.entity(pause_menu).add_children(&[pause_gui_root]);
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
                        TextFont::from_font_size(16.),
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
    mut next_gui_state: ResMut<NextState<GUIState>>,
    mut exit: EventWriter<AppExit>,
) {
    let mut vis = visibility.single_mut();

    if keys.just_pressed(KeyCode::Escape) {
        let is_closed: bool = *gui_state.get() == GUIState::Closed;

        *vis = match *vis {
            Visibility::Visible | Visibility::Inherited => Visibility::Hidden,
            Visibility::Hidden => Visibility::Visible,
        };

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
                    *vis = Visibility::Hidden;
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
