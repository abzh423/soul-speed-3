use anyhow::bail;
use libcraft::EntityKind;
use quill::{
    components::{CreativeFlying, Sneaking, Sprinting},
    entities::Player,
};
use vane::{EntityBuilder, SysResult};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Player)
        .add(CreativeFlying(false))
        .add(Sneaking(false))
        .add(Sprinting(false))
        .add(EntityKind::Player);
}

/// The hotbar slot a player's cursor is currently on
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HotbarSlot(usize);

impl HotbarSlot {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn get(&self) -> usize {
        self.0
    }

    pub fn set(&mut self, id: usize) -> SysResult {
        if id > 8 {
            bail!("invalid hotbar slot id");
        }

        self.0 = id;
        Ok(())
    }
}
