    let hud_root = commands
        .spawn((
            DebugHudRoot,
            (
                BackgroundColor(Color::BLACK.with_alpha(0.5)),
                GlobalZIndex(i32::MAX),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(1.),
                    top: Val::Percent(1.),
                    // Resized depending on text inside.
                    bottom: Val::Auto,
                    right: Val::Auto,
                    padding: UiRect::all(Val::Px(4.0)),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
            ),
        ))
        .id();
