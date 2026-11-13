use super::*;

impl Generator for Trundle {
    fn generate(&mut self) -> MayFail {
        self.take("Bonus Physical Damage", Q(Void))?
            .take("Magic Damage Per Second", R(Void))?
            .take("Initial Magic Damage", R(Min))?
            .take("Total Magic Damage", R(Max))?
            .combos([
                &[Ability(R(Void)), Attack, Ability(Q(Void)), Attack],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Attack, Ability(Q(Void)), Attack],
                &[Attack, Ability(Q(Void)), Attack, Attack, Attack],
            ])?
            .end()
    }
}
