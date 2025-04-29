use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::Monitor;
use bevy::window::PrimaryMonitor;

// Marker to find fps text entity
#[derive(Component)]
pub struct FpsText;

pub fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    monitor_q: Query<(&Monitor, Has<PrimaryMonitor>)>,
    mut fps_q: Query<(&mut TextSpan, &mut TextColor), With<FpsText>>,
) {
    for (mut span, mut color) in fps_q.iter_mut() {
        // Try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            **span = format!("{value:>3.0}");

	    let mut hz: f64 = 60.;
	    match monitor_q.single() {
		Ok((monitor, _)) => {
		    hz = (monitor.refresh_rate_millihertz.unwrap_or(0).div_ceil(10000) * 10) as f64;
		}
		Err(_) => warn!("Couldn't get monitor refresh rate") 
	    }

            // Adjust text color based on FPS value.
            color.0 = if value >= hz {
                Color::srgb(0.0, 1.0, 0.0)
            } else if value >= hz / 2. {
                Color::srgb((1.0 - (value - hz / 2.) / (hz / 2.)) as f32, 1.0, 0.0)
            } else if value >= hz / 4. {
                Color::srgb(1.0, ((value - hz / 4.) / (hz / 4.)) as f32, 0.0)
            } else {
                Color::srgb(1.0, 0.0, 0.0)
            };
        } else {
            **span = " N/A".into();
            color.0 = Color::WHITE;
        }
    }
}
