use super::*;

impl Generator for Blitzcrank {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void)])
            .modify(E(Void), |_| f![AttackDamage + 0.25 * AbilityPower])?
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, _1)])
            .modify(R(_1), |_| {
                /* 50 / 100 / 150 */
                /* (+ 30 / 40 / 50% AP) */
                /* (+ 2% maximum mana) */
                f![50 + 50 * RLevel + (0.3 + 0.1 * RLevel) * AbilityPower + 0.02 * MaxMana]
            })?
            .comment(R(_1), "R Passive Onhit Damage")?
            .combo([
                Ability(Q(Void)),
                Ability(R(Void)),
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .combo([Ability(R(Void)), Attack, Ability(E(Void)), Ability(Q(Void))])?
            .combo([
                Ability(Q(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(R(Void)),
            ])?
            .combo([
                Ability(R(Void)),
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .combo([Ability(Q(Void)), Attack, Ability(E(Void))])?
            .combo([
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([Ability(Q(Void)), Ability(E(Void)), Ability(E(Void)), Attack])?
            .end()
    }
}
