use super::*;

impl Generator for Kayle {
    fn generate(&mut self) -> MayFail {
        self.take("Level 11 - Aflame", P(Void))?
            .comment(P(Void), "Exalted fire wave, available from level 11")
            .take("Magic Damage", Q(Void))?
            .take("Bonus Magic Damage", E(Void))?
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
