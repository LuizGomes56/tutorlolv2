use super::*;

impl Generator for Sona {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate - Power Chord", 0, P(Void))?
            .take("Bonus Magic Damage", Q(Void))?
            .take("Magic Damage", Q(_1))?
            .take("Magic Damage", R(Void))?
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(R(Void)), Ability(Q(Void)), Attack, Attack],
                &[Attack, Ability(Q(Void)), Attack],
                &[Ability(R(Void)), Ability(Q(Void)), Attack],
                &[Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
