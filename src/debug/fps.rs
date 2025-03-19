use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

// Marker to find fps text entity
#[derive(Component)]
pub struct FpsText;

pub fn limit_fps(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        use bevy_framepace::Limiter;

        settings.limiter = match settings.limiter {
            Limiter::Auto => Limiter::Off,
            Limiter::Off => Limiter::from_framerate(30.0),
            Limiter::Manual(_) => Limiter::Auto,
        }
    }
}

pub fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<(&mut TextSpan, &mut TextColor), With<FpsText>>,
) {
    for (mut span, mut color) in query.iter_mut() {
        // Try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            // Format the number as to leave space for 4 digits, just in case,
            // right-aligned and rounded. This helps readability when the
            // number changes rapidly.
            **span = format!("{value:>4.1}");

            // Adjust text color based on FPS value
            color.0 = if value >= 120.0 {
                // Above 120 FPS, use green color
                Color::srgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                // Between 60-120 FPS, gradually transition from yellow to green
                Color::srgb((1.0 - (value - 60.0) / (120.0 - 60.0)) as f32, 1.0, 0.0)
            } else if value >= 30.0 {
                // Between 30-60 FPS, gradually transition from red to yellow
                Color::srgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
            } else {
                // Below 30 FPS, use red color
                Color::srgb(1.0, 0.0, 0.0)
            };
        } else {
            // Display "N/A" if we can't get a FPS measurement
            **span = " N/A".into();
            color.0 = Color::WHITE;
        }
    }
}
