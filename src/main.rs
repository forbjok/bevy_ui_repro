use bevy::{prelude::*, window::WindowResolution};

#[derive(Resource)]
struct ViewInfo {
    pub pixel_scale: u8,
}

const TITLE_FONT_SIZE: f32 = 8.0;
const FONT_SIZE: f32 = 6.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1280.0, 720.0)
                            .with_scale_factor_override(1.0),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ViewInfo { pixel_scale: 1 })
        .add_system(input_system)
        .add_system(view_scale_system)
        .add_startup_system(setup)
        .run();
}

fn input_system(keyboard_input: Res<Input<KeyCode>>, mut view_info: ResMut<ViewInfo>) {
    if keyboard_input.just_pressed(KeyCode::X) {
        view_info.pixel_scale += 1;
    } else if keyboard_input.just_pressed(KeyCode::Z) {
        if view_info.pixel_scale > 1 {
            view_info.pixel_scale -= 1;
        }
    }
}

fn view_scale_system(
    mut commands: Commands,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    view_info: Res<ViewInfo>,
) {
    if !view_info.is_changed() {
        return;
    }

    let view_info = view_info.into_inner();
    println!("PIXEL SCALE = {}", view_info.pixel_scale);

    commands.insert_resource(UiScale {
        scale: view_info.pixel_scale as f64,
    });

    for mut camera_tf in camera_query.iter_mut() {
        camera_tf.scale = Vec2::splat(1.0 / view_info.pixel_scale as f32).extend(1.0);
    }
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let font = asset_server.load("fonts/Px437_IBM_CGA.ttf");

    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::BLACK),
            style: Style {
                size: Size::new(Val::Percent(90.0), Val::Percent(90.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|cb| {
            cb.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Percent(30.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: "TITLE TEXT".to_owned(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: TITLE_FONT_SIZE,
                            color: Color::BLUE,
                        },
                    }],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });

            // Password
            cb.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Percent(35.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Password: ".to_owned(),
                            style: TextStyle {
                                font: font.clone(),
                                font_size: FONT_SIZE,
                                color: Color::RED,
                            },
                        },
                        TextSection {
                            value: "PASSWORD".to_owned(),
                            style: TextStyle {
                                font: font.clone(),
                                font_size: FONT_SIZE,
                                color: Color::RED,
                            },
                        },
                    ],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });

            cb.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    ..Default::default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: "LOADING".to_owned(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: FONT_SIZE,
                            color: Color::WHITE,
                        },
                    }],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
