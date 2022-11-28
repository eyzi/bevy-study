use bevy::prelude::*;
use std::time::Duration;

const LETTERS_PER_SECOND: f32 = 50.;

pub struct DialoguePlugin;

#[derive(Component)]
pub struct DialogueBox;

#[derive(Component)]
pub struct DialogueTextAnimating;

#[derive(Component)]
pub struct DialogueText {
    current: String,
    next: String,
    timer: Timer,
}

impl DialogueText {
    fn new(text: &'static str) -> Self {
        Self {
            current: String::from(""),
            next: String::from(text),
            timer: Timer::new(
                Duration::from_secs_f32(1. / LETTERS_PER_SECOND),
                TimerMode::Once,
            ),
        }
    }

    fn reset_timer(&mut self) {
        self.timer = Timer::new(
            Duration::from_secs_f32(1. / LETTERS_PER_SECOND),
            TimerMode::Once,
        );
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum DialogueState {
    Open,
    Closed,
}

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(DialogueState::Closed)
            .add_system_set(
                SystemSet::on_enter(DialogueState::Open).with_system(spawn_dialogue_box),
            )
            .add_system_set(
                SystemSet::on_update(DialogueState::Open)
                    .with_system(handle_interaction)
                    .with_system(animate_text),
            )
            .add_system_set(SystemSet::on_exit(DialogueState::Open).with_system(clear_dialogue_box))
            .add_system_set(
                SystemSet::on_update(DialogueState::Closed).with_system(handle_interaction),
            );
    }
}

fn spawn_dialogue_box(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let text = "According to all known laws of aviation, there is no way a bee should be able to fly. It's wings are too small to get its fat little body off the ground.";

    commands
        .spawn((
            DialogueBox,
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::FlexStart,
                    padding: UiRect::new(
                        Val::Px(100.),
                        Val::Undefined,
                        Val::Undefined,
                        Val::Px(50.),
                    ),
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
                        flex_direction: FlexDirection::Row,
                        size: Size::new(Val::Px(600.), Val::Px(100.)),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            size: Size::new(Val::Px(100.), Val::Px(100.)),
                            ..default()
                        },
                        image: asset_server.load("rpg/Characters/MC_Portrait.png").into(),
                        ..default()
                    });
                    parent
                        .spawn(ImageBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(10.)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                DialogueTextAnimating,
                                DialogueText::new(text),
                                TextBundle {
                                    style: Style {
                                        max_size: Size::new(Val::Px(500.), Val::Px(80.)),
                                        ..default()
                                    },
                                    text: Text::from_sections([]),
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

fn animate_text(
    mut commands: Commands,
    mut q_text: Query<(Entity, &DialogueTextAnimating, &mut DialogueText, &mut Text)>,
    asset_server: ResMut<AssetServer>,
    time: Res<Time>,
) {
    let font = asset_server.load("rpg/font.ttf");

    for (entity, _, mut dialogue, mut text) in q_text.iter_mut() {
        if dialogue.next.len() > 0 {
            if dialogue.timer.just_finished() {
                let next_copy = dialogue.next.clone();
                let (next_char, rest) = next_copy.split_at(1);
                dialogue.current += next_char;
                dialogue.next = rest.to_string();

                text.sections = vec![
                    TextSection::new(
                        dialogue.current.clone(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.,
                            color: Color::BLACK,
                        },
                    ),
                    TextSection::new(
                        dialogue.next.clone(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.,
                            color: Color::rgba(0., 0., 0., 0.),
                        },
                    ),
                ];

                dialogue.reset_timer();
            } else {
                dialogue.timer.tick(time.delta());
            }
        } else {
            commands.entity(entity).remove::<DialogueTextAnimating>();
        }
    }
}

fn clear_dialogue_box(mut commands: Commands, mut q_dialogue_box: Query<Entity, &DialogueBox>) {
    for dialogue_box in q_dialogue_box.iter_mut() {
        commands.entity(dialogue_box).despawn_recursive();
    }
}

#[derive(Resource)]
pub struct InteractionTimer {
    timer: Timer,
}

impl Default for InteractionTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        }
    }
}

fn handle_interaction(
    key_code: Res<Input<KeyCode>>,
    mut dialogue_state: ResMut<State<DialogueState>>,
    mut interaction_timer: Local<InteractionTimer>,
    time: Res<Time>,
) {
    if interaction_timer.timer.finished() && key_code.just_pressed(KeyCode::I) {
        match dialogue_state.current() {
            DialogueState::Open => dialogue_state.set(DialogueState::Closed).unwrap(),
            DialogueState::Closed => dialogue_state.set(DialogueState::Open).unwrap(),
        };

        *interaction_timer = InteractionTimer::default();
    } else {
        interaction_timer.timer.tick(time.delta());
    }
}
