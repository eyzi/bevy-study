// use super::state;
// use bevy::{prelude::*, ui::FocusPolicy};

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum FadeDirection {
//     In,
//     Out,
// }

// #[derive(Debug, Component)]
// pub struct Fader {
//     pub duration: f32,
//     pub direction: FadeDirection,
//     pub timer: Timer,
//     pub next_state: state::GameState,
// }

// impl Fader {
//     fn new(duration: f32, next_state: state::GameState) -> Self {
//         Self {
//             duration: 0.5,
//             direction: FadeDirection::In,
//             timer: Timer::from_seconds(duration / 2., TimerMode::Once),
//             next_state,
//         }
//     }
// }

// pub fn create(commands: &mut Commands, duration: f32, color: Color, next_state: state::GameState) {
//     commands
//         .spawn()
//         .insert(Fader::new(duration, next_state))
//         .insert_bundle(NodeBundle {
//             style: Style {
//                 position_type: PositionType::Absolute,
//                 size: Size::new(Val::Percent(100.), Val::Percent(100.)),
//                 ..default()
//             },
//             color: UiColor::from(color.clone()),
//             focus_policy: FocusPolicy::Block,
//             transform: Transform::from_xyz(0., 0., 99.),
//             ..default()
//         });
// }

// pub fn handle(
//     time: Res<Time>,
//     mut commands: Commands,
//     mut fader_query: Query<(Entity, &mut Fader, &mut UiColor)>,
//     mut state: ResMut<State<state::GameState>>,
// ) {
// for (fader_entity, mut fader, mut color) in fader_query.iter_mut() {
//     match fader.direction {
//         FadeDirection::In => {
//             color.0.set_a(fader.timer.percent());
//             if fader.timer.just_finished() {
//                 state.set(fader.next_state).unwrap();
//                 fader.direction = FadeDirection::Out;
//                 fader.timer = Timer::from_seconds(fader.duration / 2., false);
//             } else {
//                 fader.timer.tick(time.delta());
//             }
//         }
//         FadeDirection::Out => {
//             color.0.set_a(fader.timer.percent_left());
//             if fader.timer.just_finished() {
//                 commands.entity(fader_entity).despawn_recursive();
//             } else {
//                 fader.timer.tick(time.delta());
//             }
//         }
//     }
// }
// }
