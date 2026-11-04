use super::*;

impl Generator for Urgot {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                ("Description 1 [2]", Void),
                ("Description 1 [1]", _1),
            ],
        );

        let echoing_flames = self.sum([P(Void), P(_1)])?;

        self.set_damage(P(Void), echoing_flames)?
            .delete(P(_1))
            .comment(P(Void), "Echoing Flames shotgun blast")?
            .ability(Key::Q, [("Physical Damage", Void)])
            .ability(Key::W, [("Modified Physical Damage", Void)])
            .ability(Key::E, [("Physical Damage", Void)])
            .ability(Key::R, [("Physical Damage", Void)])
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
