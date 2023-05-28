use super::super::controller;
use ggez::glam::Vec2;
use legion::system;

use super::Actions;

/// A component that allows an entity to be controlled by a player.
pub struct Control {
    /// The speed at which movement orders move this entity.
    move_speed: f32,
}
impl Control {
    /// Creates a new control component.
    pub fn new(move_speed: f32) -> Self {
        Self { move_speed }
    }
}

#[system(for_each)]
/// A system that manages the translation of orders coming from the controller (via the interactions resource) to actions of control components.
pub fn control(
    control: &Control,
    actions: &mut Actions,
    gfx: Option<&mut super::Graphics>,
    #[resource] ix: &controller::Interactions,
) {
    // Movement

    let mut del = Vec2::ZERO;

    let mut pref_state = 0;

    if let Some(true) = ix.commands.get(&controller::Command::MoveLeft) {
        del.x -= 1.;
        pref_state = 2;
    }

    if let Some(true) = ix.commands.get(&controller::Command::MoveRight) {
        del.x += 1.;
        pref_state = 1;
    }

    actions.push(super::actions::GameAction::Move {
        delta: del * control.move_speed * ix.delta.as_secs_f32(),
    });

    // Spell casting

    if let Some(true) = ix.commands.get(&controller::Command::Spell0) {
        actions.push(super::actions::GameAction::CastSpell(0));
    }
    if let Some(true) = ix.commands.get(&controller::Command::Spell1) {
        actions.push(super::actions::GameAction::CastSpell(1));
    }
    if let Some(true) = ix.commands.get(&controller::Command::Spell2) {
        actions.push(super::actions::GameAction::CastSpell(2));
    }
    if let Some(true) = ix.commands.get(&controller::Command::Spell3) {
        actions.push(super::actions::GameAction::CastSpell(3));
    }

    if let Some(gfx) = gfx{
        //TODO: This should already be a check in set_variant in mooeye.
        if gfx.sprite.get_variant() != pref_state{
            gfx.sprite.set_variant(pref_state);
        }
    }
}
