use crate::core::state;
use crate::fader::plugin::create as create_fader;
use bevy::prelude::*;

#[derive(Component)]
pub struct OptionsMenuScreen;

#[derive(Component)]
pub struct OptionsItem;

pub fn setup_menu(app: &mut App) {
    app.add_system_set(SystemSet::on_enter(state::GameState::OptionsMenu).with_system(create_menu))
        .add_system_set(SystemSet::on_exit(state::GameState::OptionsMenu).with_system(remove_menu))
        .add_system_set(
            SystemSet::on_update(state::GameState::OptionsMenu).with_system(handle_button),
        );
}

fn create_menu(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font: Handle<Font> = asset_server.load("common/font.ttf");

    commands
        .spawn((
            OptionsMenuScreen,
            NodeBundle {
                style: Style {
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
                    OptionsItem,
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
                            "Main",
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
        (&Interaction, &OptionsItem, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, _, mut color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                *color = BackgroundColor(Color::rgb(0.9, 0.9, 0.9));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::WHITE);
            }
            Interaction::Clicked => {
                create_fader(&mut commands, 1., Color::BLACK, state::GameState::MainMenu);
            }
        }
    }
}

fn remove_menu(mut commands: Commands, options_menu_query: Query<Entity, &OptionsMenuScreen>) {
    if let Some(options_menu) = options_menu_query.iter().next() {
        commands.entity(options_menu).despawn_recursive();
    }
}
