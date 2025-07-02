use crate::builder::node_builder::NodeBundle;
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct ButtonBundle {
    button: Button,
    border_radius: BorderRadius,
    node_bundle: NodeBundle,
}

pub struct ButtonBuilder {
    bundle: ButtonBundle,
    text: Entity,
}

impl ButtonBuilder {
    pub fn new(text: Entity) -> Self {
        Self {
            bundle: ButtonBundle::default(),
            text,
        }
    }

    pub fn node_bundle(mut self, node_bundle: NodeBundle) -> Self {
        self.bundle.node_bundle = node_bundle;
        self
    }

    pub fn border_radius(mut self, border_radius: BorderRadius) -> Self {
        self.bundle.border_radius = border_radius;
        self
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        let parent = commands.spawn(self.bundle).id();
        commands.entity(parent).add_children(&[self.text]);
        parent
    }

    pub fn spawn_with(self, bundle: impl Bundle, commands: &mut Commands) -> Entity {
        let parent = commands.spawn((self.bundle, bundle)).id();
        commands.entity(parent).add_children(&[self.text]);
        parent
    }
}
