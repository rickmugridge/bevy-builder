use crate::builder::button_builder::ButtonBuilder;
use crate::builder::node_builder::NodeBundle;
use crate::builder::text_builder::TextBuilder;
use bevy::prelude::{Commands, Component, Entity};
use bevy::ui::Display;

#[derive(Component)]
#[relationship(relationship_target = Tabs)]
pub struct TabElement(Entity);

#[derive(Component, Default)]
#[relationship_target(relationship = TabElement, linked_spawn)]
pub struct Tabs(Vec<Entity>);

#[derive(Component)]
pub struct TabContent(Entity);

pub struct TabBuilder {
    menus: Vec<(String, Entity)>,
    node_bundle: NodeBundle,
}

impl TabBuilder {
    pub fn new() -> Self {
        Self {
            menus: Vec::new(),
            node_bundle: NodeBundle::default(),
        }
    }

    pub fn add_menus(mut self, menus: Vec<(String, Entity)>) -> Self {
        self.menus = menus;
        self
    }

    pub fn node(mut self, node: NodeBundle) -> Self {
        self.node_bundle = node;
        self
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        let container = self.make_container(commands);
        container
    }

    fn make_container(mut self, commands: &mut Commands) -> Entity {
        self.node_bundle.node.display = Display::Grid;
        let node = commands.spawn(self.node_bundle).id();
        let mut children: Vec<Entity> = Vec::new();
        self.menus.iter().for_each(|(text, entity)| {
            let button = ButtonBuilder::new(TextBuilder::new().content(text).spawn(commands))
                .spawn_with(TabContent(*entity), commands);
            children.push(button);
            commands.entity(button).insert(TabElement(node));
            children.push(*entity);
        });
        commands.entity(node).add_children(&children);
        node
    }
}
