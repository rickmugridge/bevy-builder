use bevy::prelude::*;

pub struct ButtonBuilder {
    border_radius: BorderRadius,
    justify_content: JustifyContent,
    align_items: AlignItems,
    interaction: Interaction,
    node: Node,
    text: Entity,
}

impl ButtonBuilder {
    pub fn new(node: Node, text: Entity) -> Self {
        Self {
            border_radius: BorderRadius::MAX,
            interaction: Interaction::Pressed,

            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            text,
            node,
        }
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
        let button = (Button, self.node, self.border_radius, self.interaction);
        let parent = commands.spawn(button).id();
        commands.entity(parent).add_children(&[self.text]);
        parent
    }
}
