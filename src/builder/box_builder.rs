use bevy::color::Color;
use bevy::color::palettes::basic::RED;
use bevy::prelude::{BackgroundColor, BorderColor, Commands, Entity, Node, UiRect, Val};

pub struct BoxBuilder {
    node: Node,
    border_color: BorderColor,
}

impl Default for BoxBuilder {
    fn default() -> Self {
        Self {
            node: Node {
                border: UiRect::all(Val::Px(1.0)),
                ..Node::default()
            },
            border_color: BorderColor::from(Color::WHITE),
        }
    }
}

impl BoxBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn border(mut self, border: UiRect) -> Self {
        self.node.border = border;
        self
    }

    pub fn border_of(mut self, val: Val) -> Self {
        self.node.border = UiRect::all(val);
        self
    }

    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border_color = BorderColor::from(border_color);
        self
    }

    pub fn width(mut self, val: Val) -> Self {
        self.node.width = val;
        self
    }

    pub fn height(mut self, val: Val) -> Self {
        self.node.height = val;
        self
    }

    pub fn full_box(mut self) -> Self {
        self.width(Val::Percent(100.0))
            .height(Val::Percent(100.0))
            .border_color(RED.into())
            .border_of(Val::Px(1.))
    }

    pub fn build_and_spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn((self.node, self.border_color, BackgroundColor(Color::BLACK)))
            .id()
    }
}
