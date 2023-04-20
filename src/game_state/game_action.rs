use ggez::glam::Vec2;
use legion::{*, systems::CommandBuffer};
pub type ActionQueue = Vec<(legion::Entity, GameAction)>;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum GameAction{
    Remove,
    Move{delta: Vec2},
    TakeDamage{dmg: i32},
    TakeHealing{heal: i32},
    TakeCityDamage{dmg: i32},
    GainGold{amount: i32},
    ExecutiveAction(&'static(dyn Fn(&mut World, &mut Resources) + Send + Sync)),
    AddImmunity{other: Entity},
    CastSpell(usize),
}

impl PartialEq for GameAction{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Move { delta: l_delta }, Self::Move { delta: r_delta }) => l_delta == r_delta,
            (Self::TakeDamage { dmg: l_dmg }, Self::TakeDamage { dmg: r_dmg }) => l_dmg == r_dmg,
            (Self::TakeCityDamage { dmg: l_dmg }, Self::TakeCityDamage { dmg: r_dmg }) => l_dmg == r_dmg,
            (Self::GainGold { amount: l_amount }, Self::GainGold { amount: r_amount }) => l_amount == r_amount,
            (Self::ExecutiveAction(_), Self::ExecutiveAction(_)) => false,
            (Self::AddImmunity { other: l_other }, Self::AddImmunity { other: r_other }) => l_other == r_other,
            (Self::CastSpell(l0), Self::CastSpell(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[system]
pub fn resolve_executive(#[resource] actions: &ActionQueue, cmd: &mut CommandBuffer) {
    for (_, action) in actions {
        if let GameAction::ExecutiveAction(lambda) = action {
            cmd.exec_mut(*lambda);
        }
    }
}
