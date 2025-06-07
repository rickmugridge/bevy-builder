use bevy::color::Color;
use bevy::prelude::{BackgroundColor, Commands, Entity, Node, UiRect};

pub struct BoxBuilder {
    node: Node,
    border_color: Color,
}

impl Default for BoxBuilder {
    fn default() -> Self {
        Self {
            node: Node::default(),
            border_color: Color::WHITE,
        }
    }
}

impl BoxBuilder {
    pub fn new(mut self, border: UiRect, border_color: Color) -> Self {
        self.node.border = border;
        self.border_color = border_color;
        self
    }

    pub fn build_and_spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn((self.node, BackgroundColor(self.border_color)))
            .id()
    }
}
