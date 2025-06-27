use crate::edit_plugin::text_edit_plugin::TextEditPlugin;
use crate::ui_plugin::color_plugin::ColorPlugin;
use crate::ui_plugin::debounce_mouse_press_plugin::DebounceMousePressPlugin;
use crate::ui_plugin::open_close_plugin::OpenClosePlugin;
use bevy::app::{App, Plugin};
use crate::ui_plugin::number_plugin::NumberPlugin;

pub mod text_edit_plugin;

pub struct EditPlugin;

impl Plugin for EditPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TextEditPlugin,
            ColorPlugin,
            OpenClosePlugin,
            DebounceMousePressPlugin,
            NumberPlugin,
        ));
    }
}
