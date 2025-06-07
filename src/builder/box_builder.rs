use bevy::color::{Color, Srgba};
use bevy::prelude::{BorderColor, Commands, Entity, Node, UiRect};

pub struct BoxBuilder {
    node: Node,
    border_color: Color,
}

impl Default for BoxBuilder {
    fn default() -> Self {
        Self {
            node: Node::default(),
            border_color: Color::default(),
        }
    }
}

impl BoxBuilder {
    pub fn border(mut self, border: UiRect, border_color: Color) -> Self {
        self.node.border = border;
        self.border_color = border_color;
        self
    }

    pub fn build_and_spawn(self, commands: &mut Commands) -> Entity {
        let id = commands.spawn(self.node).id();
        commands
            .entity(id)
            .insert(BorderColor(Color::Srgba(Srgba::RED)));
        id
    }
}
