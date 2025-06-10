use bevy::prelude::*;

pub struct ButtonBuilder {
    border_color: Color,
    border_radius: BorderRadius,
    background_color: Color,
    justify_content: JustifyContent,
    align_items: AlignItems,
    interaction: Interaction,
    id: ButtonId,
    node: Node,
    text: Entity,
}

#[derive(Debug, Component)]
pub struct ButtonId(String);

impl ButtonBuilder {
    pub fn new(node: Node, text: Entity) -> Self {
        Self {
            id: ButtonId("(none)".to_string()),
            border_color: Color::WHITE,
            border_radius: BorderRadius::MAX,
            background_color: Color::srgb(0.15, 0.15, 0.15),
            interaction: Interaction::Pressed,

            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            text,
            node,
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = ButtonId(id.into());
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn border_radius(mut self, border_radius: BorderRadius) -> Self {
        self.border_radius = border_radius;
        self
    }

    pub fn justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.justify_content = justify_content;
        self
    }

    pub fn align_items(mut self, align_items: AlignItems) -> Self {
        self.align_items = align_items;
        self
    }

    pub fn interaction(mut self, interaction: Interaction) -> Self {
        self.interaction = interaction;
        self
    }

    pub fn build_and_spawn(self, commands: &mut Commands) -> Entity {
        let button = (
            Button,
            self.id,
            self.node,
            BackgroundColor(self.background_color),
            BorderColor(self.border_color),
            self.border_radius,
            self.interaction,
        );
        let parent = commands.spawn(button).id();
        commands.entity(parent).add_children(&[self.text]);
        parent
    }
}
