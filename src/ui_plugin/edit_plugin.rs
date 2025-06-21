use crate::ui_plugin::button_edit_plugin::ButtonEditPlugin;
use crate::ui_plugin::color_sample_plugin::ColorSamplePlugin;
use bevy::app::{App, Plugin};
use crate::ui_plugin::debounce_mouse_press_plugin::DebounceMousePressPlugin;
use crate::ui_plugin::open_close_plugin::OpenClosePlugin;

pub struct EditPlugin;

impl Plugin for EditPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ButtonEditPlugin, ColorSamplePlugin, OpenClosePlugin, DebounceMousePressPlugin));
    }
}
