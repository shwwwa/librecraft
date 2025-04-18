use bevy::prelude::*;

use crate::gui::{GUIMode, GUIModeChanged};

/** Marker to find pause menu background entity. */
#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub enum PauseButtonAction {
    Resume,
    Exit,
}

/** Creates and setups pause menu. */
pub fn setup_pause_menu(mut commands: Commands) {
    let pause_menu = commands
        .spawn((
            PauseMenu,
	    Name::new("PauseMenu"),
            (
                BackgroundColor(Color::BLACK.with_alpha(0.6)),
                GlobalZIndex(5),
                (
		    Node {
			width: Val::Vw(100.),
			height: Val::Vh(100.),
			display: Display::Flex,
			align_items: AlignItems::Center,
			justify_content: JustifyContent::Center,
			..Default::default()
                    },
		)
            ),
        ))
        .id();

    let pause_gui_root = commands
	.spawn(
	    Node {
		display: Display::Flex,
		flex_direction: FlexDirection::Column,
		align_items: AlignItems::Center,
		justify_content: JustifyContent::SpaceAround,
		height: Val::Vh(40.),
		min_width: Val::Vw(40.),
		..Default::default()
	    }
	)
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

    commands.entity(pause_gui_root)
	.with_children(|wrapper| {
	    for (msg, action) in [
		("Resume", PauseButtonAction::Resume),
		("Exit", PauseButtonAction::Exit),
	    ] {
		wrapper.spawn((
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
                            ..Default::default()
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

pub fn render_pause_menu(
    queries: (
	Query<(&PauseButtonAction, &mut BorderColor, &Interaction)>,
	Query<&mut Visibility, With<PauseMenu>>,
    ),
    keys: Res<ButtonInput<KeyCode>>,
    // todo: rewrite it using states?
    mut gui_mode: ResMut<GUIMode>,
    mut gui_mode_writer: EventWriter<GUIModeChanged>,
    mut exit: EventWriter<AppExit>,
) {
    let (mut button, mut visibility) = queries;
    let mut vis = visibility.single_mut();
    
    if keys.just_pressed(KeyCode::Escape) {
	*vis = match *vis {
	    Visibility::Visible | Visibility::Inherited => Visibility::Hidden,
	    Visibility::Hidden => Visibility::Visible,
	};

	if *gui_mode == GUIMode::Closed {
            *gui_mode = GUIMode::Opened;
        } else {
            *gui_mode = GUIMode::Closed;
        }
	
	info!("Pause menu was opened (via key): {:?}", *gui_mode);
        gui_mode_writer.send(GUIModeChanged {
            gui_mode: *gui_mode,
        });
    }

    if *vis != Visibility::Visible {
	return;
    }
    
    for (action, mut bcolor, interaction) in button.iter_mut() {
        match *interaction {
            Interaction::Pressed => match *action {
                PauseButtonAction::Resume => {
                    *vis = Visibility::Hidden;
		    *gui_mode = GUIMode::Closed;

		    info!("Resuming game: {:?}", *gui_mode);
		    gui_mode_writer.send(GUIModeChanged {
			gui_mode: *gui_mode,
		    });
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
