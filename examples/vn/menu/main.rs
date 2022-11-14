use crate::core::config;
use crate::core::state;
use crate::fader::plugin::{create as create_fader, Fader};
use crate::menu::plugin;
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
        .add_system_set(
            SystemSet::on_exit(state::GameState::MainMenu)
                .with_system(plugin::remove_menu_screen::<MainMenuScreen>),
        )
        .add_system_set(
            SystemSet::on_update(state::GameState::MainMenu).with_system(handle_button),
        );
}

fn create_menu(mut commands: Commands, game_config: Res<config::GameConfig>) {
    let screen = commands
        .spawn((
            MainMenuScreen,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK),
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
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            MenuItem::Play,
            ButtonBundle {
                background_color: BackgroundColor(Color::ALICE_BLUE),
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Px(500.), Val::Px(100.)),
                    margin: UiRect::all(Val::Px(20.)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Play",
                    TextStyle {
                        font: game_config.game_font.clone(),
                        font_size: 80.,
                        color: Color::RED,
                    },
                ),
                ..default()
            });
        })
        .set_parent(screen);

    commands
        .spawn((
            MenuItem::Exit,
            ButtonBundle {
                background_color: BackgroundColor(Color::ALICE_BLUE),
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Px(500.), Val::Px(100.)),
                    margin: UiRect::all(Val::Px(20.)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Exit",
                    TextStyle {
                        font: game_config.game_font.clone(),
                        font_size: 80.,
                        color: Color::RED,
                    },
                ),
                ..default()
            });
        })
        .set_parent(screen);
}

fn handle_button(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &MenuItem, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut exit: EventWriter<AppExit>,
    mut fader_q: Query<&Fader>,
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
                    if fader_q.iter_mut().next().is_none() {
                        create_fader(&mut commands, 0.5, Color::BLACK, state::GameState::Playing);
                    }
                }
                MenuItem::Exit => {
                    exit.send(AppExit);
                }
            },
        }
    }
}
