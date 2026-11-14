use super::*;

impl Generator for Kayle {
    fn generate(&mut self) -> MayFail {
        self.take("Level 11 - Aflame", P(Void))?
            .comment(P(Void), "Exalted fire wave, available from level 11")
            .map_change(P(Void), "Wave bonus AD ratio")?
            .map_change(P(Void), "Wave AP ratio")?
            .take("Magic Damage", Q(Void))?
            .take("Bonus Magic Damage", E(Void))?
            .map_change(E(Void), "Health ratio")?
            .take("Passive Damage", E(_1))?
            .take("Magic Damage", R(Void))?
            .combos([
                &[Ability(Q(Void)), Attack, Ability(E(Void))],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(E(Void)), Ability(E(Void)), Attack],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[Attack, Ability(E(Void)), Ability(Q(Void))],
                &[Ability(Q(Void)), Attack, Attack, Ability(E(Void))],
            ])?
            .end()
    }
}
