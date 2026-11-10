use super::*;

impl Generator for Kalista {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Physical Damage", Void)])
            .ability(Key::W, [("Bonus Magic Damage", Void)])
            .ability(
                Key::E,
                [
                    ("Bonus Damage per Additional Stack", _1),
                    ("Physical Damage", Void),
                ],
            );

        let e_dmg = self.merge_damage([E(_1), E(Void)], |[e1, e]| f![(e) + ((e1) * Stacks)])?;
        self.set_damage(E(Void), e_dmg)?
            .combos([
                &[
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                ],
                &[Attack, Ability(Q(Void)), Ability(E(Void))],
                &[Ability(Q(Void)), Attack, Ability(E(Void))],
                &[Attack, Ability(Q(Void)), Attack, Ability(E(Void))],
            ])?
            .end()
    }
}
