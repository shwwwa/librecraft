use bevy::{
    prelude::*,
    window::{WindowFocused, WindowResized},
};

// Marker to find display text entity
#[derive(Component)]
pub struct DisplayText;

#[derive(Component)]
pub struct FocusText;

pub fn update_display_text(
    mut query: Query<(&mut TextSpan, &mut TextColor), With<DisplayText>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    // TODO: color change onchange()
    for (mut span, _) in query.iter_mut() {
        for e in resize_reader.read() {
            // When resolution is being changed
            **span = format!("{}x{}", e.width, e.height);
        }
    }
}

pub fn update_focus_text(
    mut query: Query<&mut TextSpan, With<FocusText>>,
    mut focus_reader: EventReader<WindowFocused>,
) {
    for mut span in query.iter_mut() {
        for e in focus_reader.read() {
            if e.focused {
                **span = "".into();
            } else {
                **span = " (not focused)".into();
            }
        }
    }
}
