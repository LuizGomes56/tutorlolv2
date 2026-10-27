use super::*;

impl Generator for Urgot {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (1, Void), /* Description 1 [2] */
                (0, _1),   /* Description 1 [1] */
            ],
        );

        let echoing_flames = self.sum([P(Void), P(_1)])?;

        self.set_damage(P(Void), echoing_flames)?
            .delete(P(_1))
            .comment(P(Void), "Echoing Flames shotgun blast")?
            .ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability(Key::W, [(0, Void) /* Modified Physical Damage */])
            .ability(Key::E, [(0, Void) /* Physical Damage */])
            .ability(Key::R, [(0, Void) /* Physical Damage */])
            .damage_type(R(Void), Physical)?
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Attack,
                    Ability(W(Void)),
                ],
                &[Ability(E(Void)), Ability(R(Void)), Attack, Ability(W(Void))],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(R(Void)),
                ],
            ])?
            .end()
    }
}
