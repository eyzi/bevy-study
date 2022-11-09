use super::super::core::state;
use bevy::app::AppExit;
use bevy::prelude::*;

pub struct MainMenuPlugin;

#[derive(Component)]
pub struct MainMenuScreen;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub enum MenuItem {
    Play,
    Exit,
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(state::GameState::MainMenu).with_system(add_main_menu),
        )
        .add_system_set(
            SystemSet::on_exit(state::GameState::MainMenu).with_system(remove_main_menu),
        )
        .add_system_set(SystemSet::on_update(state::GameState::MainMenu).with_system(handle_button))
        .add_system_set(SystemSet::on_update(state::GameState::MainMenu).with_system(hover_button));
    }
}

fn add_main_menu(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font: Handle<Font> = asset_server.load("common/font.ttf");

    commands
        .spawn()
        .insert(MainMenuScreen)
        .insert_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: UiColor::from(Color::BLACK),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn()
                .insert(MenuItem::Play)
                .insert_bundle(ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        position: UiRect {
                            top: Val::Percent(-10.),
                            ..default()
                        },
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Px(500.), Val::Px(100.)),
                        ..default()
                    },
                    color: UiColor::from(Color::ALICE_BLUE),
                    ..default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn().insert_bundle(TextBundle {
                        text: Text::from_section(
                            "Play",
                            TextStyle {
                                font: font.clone(),
                                font_size: 80.,
                                color: Color::RED,
                            },
                        ),
                        ..default()
                    });
                });

            parent
                .spawn()
                .insert(MenuItem::Exit)
                .insert_bundle(ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        position: UiRect {
                            top: Val::Percent(10.),
                            ..default()
                        },
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Px(500.), Val::Px(100.)),
                        ..default()
                    },
                    color: UiColor::from(Color::ALICE_BLUE),
                    ..default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn().insert_bundle(TextBundle {
                        text: Text::from_section(
                            "Exit",
                            TextStyle {
                                font: font.clone(),
                                font_size: 80.,
                                color: Color::RED,
                            },
                        ),
                        ..default()
                    });
                });
        });
}

fn handle_button(
    mut interaction_query: Query<(&Interaction, &MenuItem, Changed<Interaction>)>,
    mut exit: EventWriter<AppExit>,
    mut state: ResMut<State<state::GameState>>,
) {
    for (interaction, menu_item, _) in interaction_query.iter_mut() {
        if let Interaction::Clicked = interaction {
            if let MenuItem::Exit = menu_item {
                exit.send(AppExit);
            }

            match menu_item {
                MenuItem::Play => {
                    state.set(state::GameState::Playing).unwrap();
                }
                MenuItem::Exit => {
                    exit.send(AppExit);
                }
            }
        }
    }
}

fn hover_button(mut interaction_query: Query<(&Interaction, &mut UiColor, Changed<Interaction>)>) {
    for (interaction, mut color, _) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                *color = UiColor::from(Color::rgb(0.9, 0.9, 0.9));
            }
            Interaction::None => {
                *color = UiColor::from(Color::WHITE);
            }
            _ => {}
        }
    }
}

fn remove_main_menu(mut commands: Commands, main_menu_query: Query<Entity, &MainMenuScreen>) {
    if let Some(main_menu) = main_menu_query.iter().next() {
        commands.entity(main_menu).despawn_recursive();
    }
}
