use bevy::app::{App, Plugin};
use bevy::prelude::Resource;
use std::time::{Duration, Instant};

pub struct DebounceMousePressPlugin;

impl Plugin for DebounceMousePressPlugin {
    fn build(&self, app: &mut App) {
        app            .insert_resource(DebounceMousePress::default());
    }
}

#[derive(Resource)]
pub struct DebounceMousePress {
    pub last_pressed: Instant,
    pub cooldown: Duration,
}

impl Default for DebounceMousePress {
    fn default() -> Self {
        Self {
            last_pressed: Instant::now(),
            cooldown: Duration::from_millis(200), // Adjust cooldown as needed
        }
    }
}

impl DebounceMousePress {
    pub fn pressed(&mut self) -> bool {
        let now = Instant::now();
        if now - self.last_pressed < self.cooldown {
            return false;
        }
        self.last_pressed = now;
        true
    }
}
