// This file is @generated. Please do not edit.
use libcraft::EntityKind;
use quill::entities::Shulker;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Shulker).add(EntityKind::Shulker);
}
