use crate::core::state;
use bevy::{prelude::*, ui::FocusPolicy};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FadeDirection {
    In,
    Out,
}

#[derive(Component)]
pub struct Fader {
    pub duration: f32,
    pub direction: FadeDirection,
    pub timer: Timer,
    pub next_state: state::GameState,
}

impl Fader {
    fn new(duration: f32, next_state: state::GameState) -> Self {
        Self {
            duration,
            next_state,
            direction: FadeDirection::In,
            timer: Timer::from_seconds(duration / 2., TimerMode::Once),
        }
    }
}

pub struct FaderPlugin;

impl Plugin for FaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle);
    }
}

pub fn handle(
    time: Res<Time>,
    mut commands: Commands,
    mut fader_q: Query<(Entity, &mut Fader, &mut BackgroundColor)>,
    mut state: ResMut<State<state::GameState>>,
) {
    for (fader_entity, mut fader, mut color) in fader_q.iter_mut() {
        match fader.direction {
            FadeDirection::In => {
                color.0.set_a(fader.timer.percent());
                if fader.timer.just_finished() {
                    state.set(fader.next_state).unwrap();
                    fader.direction = FadeDirection::Out;
                    fader.timer = Timer::from_seconds(fader.duration / 2., TimerMode::Once);
                } else {
                    fader.timer.tick(time.delta());
                }
            }
            FadeDirection::Out => {
                color.0.set_a(fader.timer.percent_left());
                if fader.timer.just_finished() {
                    commands.entity(fader_entity).despawn_recursive();
                } else {
                    fader.timer.tick(time.delta());
                }
            }
        }
    }
}

pub fn create(commands: &mut Commands, duration: f32, color: Color, next_state: state::GameState) {
    commands.spawn((
        Fader::new(duration, next_state),
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..default()
            },
            background_color: BackgroundColor(color),
            z_index: ZIndex::Global(99),
            focus_policy: FocusPolicy::Pass,
            ..default()
        },
    ));
}
