use super::*;

impl Generator for Blitzcrank {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void)])
            .modify(E(Void), |_| {
                /* 100% AD (+ 25% AP) */
                AttackDamage.plus(0.25).times(AbilityPower)
            })?
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, _1)])
            .modify(R(_1), |_| {
                /* 50 / 100 / 150 */
                let base = 50.plus(50).times(RLevel).parenthesize();
                /* (+ 30 / 40 / 50% AP) */
                let ap = 0.3.plus(0.1).times(RLevel).parenthesize();
                /* (+ 2% maximum mana) */
                let mana = 0.02.times(MaxMana).parenthesize();
                base.plus(ap).plus(mana)
            })?
            .comment(R(_1), "R Passive Onhit Damage")?
            .end()
    }
}
