use crate::core::config;
use crate::core::state;
use crate::fader::plugin::{create as create_fader, Fader};
use crate::menu::plugin;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct ScenePlugin;

#[derive(Component)]
pub struct SceneScreen;

#[derive(Clone, Copy)]
pub enum BevnCommandType {
    Dialogue,
}

#[derive(Clone)]
pub struct BevnCommand {
    pub command_type: BevnCommandType,
    pub speaker: String,
    pub line: String,
}

#[derive(Resource)]
pub struct BevnScriptTracker {
    pub scene: String,
    pub index: u16,
}

#[derive(Resource)]
pub struct BevnScript {
    pub script: HashMap<String, Vec<BevnCommand>>,
    pub start_scene: String,
}

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        let mut script = HashMap::<String, Vec<BevnCommand>>::new();
        script.insert(
            String::from("start"),
            vec![
                BevnCommand {
                    command_type: BevnCommandType::Dialogue,
                    speaker: String::from("Hana"),
                    line: String::from("Hi, my name is Hana."),
                },
                BevnCommand {
                    command_type: BevnCommandType::Dialogue,
                    speaker: String::from("Hana"),
                    line: String::from("This is a pen."),
                },
            ],
        );

        app.insert_resource(BevnScript {
            script,
            start_scene: String::from("start"),
        });

        app.add_system_set(
            SystemSet::on_enter(state::GameState::Playing)
                .with_system(create_scene)
                .with_system(reset_tracking),
        )
        .add_system_set(
            SystemSet::on_exit(state::GameState::Playing)
                .with_system(plugin::remove_menu_screen::<SceneScreen>)
                .with_system(plugin::remove_menu_screen::<BevnScene>)
                .with_system(plugin::remove_menu_screen::<BevnUI>),
        )
        .add_system_set(
            SystemSet::on_update(state::GameState::Playing)
                .with_system(handle_interaction)
                .with_system(animate_text)
                .with_system(manage_scene),
        );
    }
}

pub fn reset_tracking(mut commands: Commands) {
    commands.insert_resource(BevnScriptTracker {
        scene: String::from("start"),
        index: 0,
    });
}

#[derive(Component)]
pub struct BevnScene;

#[derive(Component)]
pub struct BevnUI;

#[derive(Component)]
pub struct BevnUiWho;

#[derive(Component)]
pub struct BevnUiWhoBox;

#[derive(Component)]
pub struct BevnUiWhat;

#[derive(Component)]
pub struct BevnUiWhatBox;

#[derive(Component)]
pub struct TextAnimating {
    pub text: String,
    pub timer: Timer,
}

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
                        flex_direction: FlexDirection::Column,
                        size: Size::new(Val::Percent(100.), Val::Px(200.)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(1., 1., 1., 0.5)),
                    ..default()
                })
                .with_children(|textbox| {
                    // who box
                    textbox
                        .spawn((
                            BevnUiWhoBox,
                            NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.), Val::Percent(20.)),
                                    padding: UiRect::new(
                                        Val::Px(200.),
                                        Val::Px(200.),
                                        Val::Px(10.),
                                        Val::Px(10.),
                                    ),
                                    ..default()
                                },
                                ..default()
                            },
                        ))
                        .with_children(|who_box| {
                            who_box.spawn((
                                BevnUiWho,
                                TextBundle {
                                    style: Style { ..default() },
                                    text: Text::from_section(
                                        "",
                                        TextStyle {
                                            font_size: 30.,
                                            font: game_config.game_font.clone(),
                                            color: Color::BLACK,
                                        },
                                    ),
                                    ..default()
                                },
                            ));
                        });

                    // what box
                    textbox
                        .spawn((
                            BevnUiWhatBox,
                            NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                    padding: UiRect::new(
                                        Val::Px(100.),
                                        Val::Px(100.),
                                        Val::Px(20.),
                                        Val::Px(20.),
                                    ),
                                    ..default()
                                },
                                ..default()
                            },
                        ))
                        .with_children(|what_box| {
                            what_box.spawn((
                                BevnUiWhat,
                                TextAnimating {
                                    text: String::from(""),
                                    timer: Timer::from_seconds(0.05, TimerMode::Once),
                                },
                                TextBundle {
                                    text: Text::from_section(
                                        "",
                                        TextStyle {
                                            font_size: 20.,
                                            font: game_config.game_font.clone(),
                                            color: Color::BLACK,
                                        },
                                    ),
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

pub fn manage_scene(
    mut commands: Commands,
    bevn_script: Res<BevnScript>,
    bevn_script_tracker: Res<BevnScriptTracker>,
    mut q_bevn_what: Query<(Entity, &mut Text, With<BevnUiWhat>, Without<BevnUiWho>)>,
    mut q_bevn_who: Query<(&mut Text, With<BevnUiWho>, Without<BevnUiWhat>)>,
) {
    if bevn_script_tracker.is_changed() {
        let bevn_scripts = bevn_script.script.get("start").unwrap();
        let bevn_current_scripts = bevn_scripts[bevn_script_tracker.index as usize].clone();

        if let Some((mut bevn_who, _, _)) = q_bevn_who.iter_mut().next() {
            bevn_who.sections[0].value = bevn_current_scripts.speaker;
            if let Some((entity, mut bevn_what, _, _)) = q_bevn_what.iter_mut().next() {
                bevn_what.sections[0].value = String::from("");
                commands.entity(entity).insert(TextAnimating {
                    text: bevn_current_scripts.line,
                    timer: Timer::from_seconds(0.05, TimerMode::Once),
                });
            }
        }
    }
}

pub fn animate_text(
    mut commands: Commands,
    mut q_what: Query<(Entity, &mut Text, &mut TextAnimating), With<BevnUiWhat>>,
    time: Res<Time>,
) {
    for (entity, mut what, mut text_animation) in q_what.iter_mut() {
        if text_animation.timer.just_finished() {
            if what.sections[0].value == text_animation.text {
                commands.entity(entity).remove::<TextAnimating>();
            } else {
                let next_index = what.sections[0].value.len();
                let next_character = text_animation.text.chars().nth(next_index).unwrap();
                what.sections[0].value += &next_character.to_string();
                text_animation.timer = Timer::from_seconds(0.05, TimerMode::Once);
            }
        } else {
            text_animation.timer.tick(time.delta());
        }
    }
}

pub fn handle_interaction(
    mut commands: Commands,
    mouse_event: Res<Input<MouseButton>>,
    mut fader_q: Query<&Fader>,
    keyboard_event: Res<Input<KeyCode>>,
    mut q_bevn_ui: Query<(&BevnUI, &mut Visibility)>,
    bevn_script: Res<BevnScript>,
    mut bevn_script_tracker: ResMut<BevnScriptTracker>,
) {
    if keyboard_event.just_pressed(KeyCode::H) {
        if let Some((_, mut visibility)) = q_bevn_ui.iter_mut().next() {
            visibility.is_visible = !visibility.is_visible;
        }
    }

    if mouse_event.just_pressed(MouseButton::Left) {
        if let Some(bevn_commands) = bevn_script.script.get("start") {
            let next_index = (bevn_script_tracker.index + 1) as usize;
            if next_index < bevn_commands.len() {
                bevn_script_tracker.index += 1;
            } else if fader_q.iter_mut().next().is_none() {
                create_fader(
                    &mut commands,
                    0.5,
                    Color::BLACK,
                    state::GameState::OptionsMenu,
                );
            }
        }
    }
}
