use super::*;

impl GeneratorV2 for Urgot {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .damage_type(P(Void), Physical)
            .take("Physical Damage", Q(Void))?
            .take("Modified Physical Damage", W(Void))?
            .take("Physical Damage", E(Void))?
            .take("Physical Damage", R(Void))?
            .damage_type(R(Void), Physical)
            .comment(P(Void), "Echoing Flames shotgun blast")
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
