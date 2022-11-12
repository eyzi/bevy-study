use crate::core::state;
use crate::fader::plugin::create as create_fader;
use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenuScreen;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub enum MenuItem {
    Play,
    Exit,
}

pub fn setup_menu(app: &mut App) {
    app.add_system_set(SystemSet::on_enter(state::GameState::MainMenu).with_system(create_menu))
        .add_system_set(SystemSet::on_exit(state::GameState::MainMenu).with_system(remove_menu))
        .add_system_set(
            SystemSet::on_update(state::GameState::MainMenu).with_system(handle_button),
        );
}

fn create_menu(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font: Handle<Font> = asset_server.load("common/font.ttf");

    commands
        .spawn((
            MainMenuScreen,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    align_self: AlignSelf::Center,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    MenuItem::Play,
                    ButtonBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            size: Size::new(Val::Px(500.), Val::Px(100.)),
                            margin: UiRect::all(Val::Px(20.)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::ALICE_BLUE),
                        ..default()
                    },
                ))
                .with_children(|button_parent| {
                    button_parent.spawn(TextBundle {
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
                .spawn((
                    MenuItem::Exit,
                    ButtonBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            size: Size::new(Val::Px(500.), Val::Px(100.)),
                            margin: UiRect::all(Val::Px(20.)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::ALICE_BLUE),
                        ..default()
                    },
                ))
                .with_children(|button_parent| {
                    button_parent.spawn(TextBundle {
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
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &MenuItem, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, menu_item, mut color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                *color = BackgroundColor(Color::rgb(0.9, 0.9, 0.9));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::WHITE);
            }
            Interaction::Clicked => match menu_item {
                MenuItem::Play => {
                    create_fader(
                        &mut commands,
                        0.5,
                        Color::BLACK,
                        state::GameState::OptionsMenu,
                    );
                }
                MenuItem::Exit => {
                    exit.send(AppExit);
                }
            },
        }
    }
}

fn remove_menu(mut commands: Commands, main_menu_query: Query<Entity, &MainMenuScreen>) {
    if let Some(main_menu) = main_menu_query.iter().next() {
        commands.entity(main_menu).despawn_recursive();
    }
}
