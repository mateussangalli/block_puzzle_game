use crate::game_objects::grid::GridPosition;
use crate::game_objects::piece::{Controllable, GameGrid};
use bevy::prelude::*;

#[derive(Component)]
pub struct InputTimer {
    start_timer: f32,
    repeat_timer: f32,
    repeat_delay: f32,
    start_delay: f32,
    key_pressed: Option<KeyCode>,
}

impl InputTimer {
    pub fn new(repeat_delay: f32, start_delay: f32) -> Self {
        Self {
            repeat_delay,
            start_delay,
            start_timer: 0.,
            repeat_timer: 0.,
            key_pressed: None,
        }
    }

    fn reset(&mut self) {
        self.start_timer = 0.;
        self.repeat_timer = 0.;
        self.key_pressed = None;
    }

    fn can_repeat(&mut self, delta_seconds: f32) -> bool {
        if self.start_timer > self.start_delay {
            self.repeat_timer += delta_seconds;
            if self.repeat_timer > self.repeat_delay {
                self.repeat_timer = 0.;
                true
            } else {
                false
            }
        } else {
            self.start_timer += delta_seconds;
            false
        }
    }

    pub fn update(&mut self, new_key: Option<KeyCode>, delta_seconds: f32) -> Option<KeyCode> {
        match (self.key_pressed, new_key) {
            (None, None) => None,
            (Some(key), None) => {
                self.reset();
                None
            }
            (Some(key1), Some(key2)) => {
                if key1 == key2 {
                    if self.can_repeat(delta_seconds) {
                        Some(key1)
                    } else {
                        None
                    }
                } else {
                    self.key_pressed = Some(key2);
                    Some(key2)
                }
            }
            (_, Some(key)) => {
                self.key_pressed = Some(key);
                Some(key)
            }
        }
    }
}

pub fn move_active(
    query_piece: Query<(&mut Transform, &mut GridPosition, &Controllable)>,
    query_grid: Query<&GameGrid>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
}
