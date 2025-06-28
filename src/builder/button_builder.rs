use crate::builder::node_builder::NodeBundle;
use bevy::prelude::*;

pub struct ButtonBuilder {
    border_radius: BorderRadius,
    interaction: Interaction,
    node_bundle: NodeBundle,
    text: Entity,
}

impl ButtonBuilder {
    pub fn new(node_bundle: NodeBundle, text: Entity) -> Self {
        Self {
            border_radius: BorderRadius::MAX,
            interaction: Interaction::Pressed,
            text,
            node_bundle,
        }
    }

    pub fn border_radius(mut self, border_radius: BorderRadius) -> Self {
        self.border_radius = border_radius;
        self
    }
    
   pub fn interaction(mut self, interaction: Interaction) -> Self {
        self.interaction = interaction;
        self
    }

    pub fn build_and_spawn(self, commands: &mut Commands) -> Entity {
        let button = (
            Button,
            self.node_bundle,
            self.border_radius,
            self.interaction,
        );
        let parent = commands.spawn(button).id();
        commands.entity(parent).add_children(&[self.text]);
        parent
    }
}
