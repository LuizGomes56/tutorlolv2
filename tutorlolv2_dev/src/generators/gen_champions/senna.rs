use super::*;

impl Generator for Senna {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (2, _1),   /* Innate - Relic Cannon [1] */
                (3, Void), /* Innate - Weakened Soul */
            ],
        )
        .modify(P(_1), |dmg| {
            dmg.replace(&SteelcapsEffect.to_string(), &AttackDamage.to_string())
        })?
        .modify(P(Void), |dmg| dmg.parenthesize().times(EnemyCurrentHealth))?
        .ability(Key::Q, [(1, Void) /* Physical Damage */])
        .ability(Key::W, [(0, Void) /* Physical Damage */])
        .ability(Key::R, [(0, Void) /* Physical Damage */])
        .end()
    }
}
