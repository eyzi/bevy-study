use crate::core::config;
use crate::core::state;
use crate::fader::plugin::{create as create_fader, Fader};
use crate::menu::plugin;
use bevy::prelude::*;

pub struct ScenePlugin;

#[derive(Component)]
pub struct SceneScreen;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(state::GameState::Playing).with_system(create_scene),
        )
        .add_system_set(
            SystemSet::on_exit(state::GameState::Playing)
                .with_system(plugin::remove_menu_screen::<SceneScreen>),
        )
        .add_system_set(
            SystemSet::on_update(state::GameState::Playing).with_system(handle_interaction),
        );
    }
}

#[derive(Component)]
pub struct BevnScene;

#[derive(Component)]
pub struct BevnUI;

pub fn create_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_config: Res<config::GameConfig>,
) {
    commands
        .spawn((BevnScene, SpatialBundle::default()))
        .with_children(|parent| {
            // bg
            parent.spawn(SpriteBundle {
                texture: asset_server.load("vn/BG.png"),
                global_transform: GlobalTransform::from_xyz(0., 0., -10.),
                ..default()
            });

            // sprite
            parent.spawn(SpriteBundle {
                texture: asset_server.load("vn/Hana/Hana Smile.png"),
                transform: Transform {
                    scale: Vec3::new(0.8, 0.8, 0.),
                    translation: Vec3::new(0., -40., 0.),
                    ..default()
                },
                global_transform: GlobalTransform::from_xyz(0., 0., 0.),
                ..default()
            });
        });

    commands
        .spawn((
            BevnUI,
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                z_index: ZIndex::Global(10),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Px(250.)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(1., 1., 1., 0.3)),
                    ..default()
                })
                .with_children(|textbox| {
                    textbox.spawn(TextBundle {
                        style: Style {
                            align_self: AlignSelf::Auto,
                            ..default()
                        },
                        text: Text::from_section(
                            "Hana",
                            TextStyle {
                                font_size: 15.,
                                font: game_config.game_font.clone(),
                                color: Color::BLACK,
                            },
                        ),
                        global_transform: GlobalTransform::from_xyz(100., 100., 0.),
                        ..default()
                    });
                });
        });
}

pub fn handle_interaction(
    mut commands: Commands,
    mouse_event: Res<Input<MouseButton>>,
    mut fader_q: Query<&Fader>,
) {
    if fader_q.iter_mut().next().is_none() && mouse_event.just_pressed(MouseButton::Left) {
        create_fader(
            &mut commands,
            0.5,
            Color::BLACK,
            state::GameState::OptionsMenu,
        );
    }
}
