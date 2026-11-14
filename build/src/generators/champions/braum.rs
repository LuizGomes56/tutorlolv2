use super::*;

impl Generator for Braum {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .take_nth("Innate", 2, P(_1))?
            .take("Magic Damage", Q(Void))?
            .take("Magic Damage", R(Void))?
            .combos([
                &[Ability(Q(Void)), Ability(R(Void))],
                &[Ability(Q(Void)), Attack, Attack, Attack],
                &[Ability(R(Void)), Ability(Q(Void)), Attack, Attack, Attack],
                &[Ability(Q(Void)), Attack, Ability(R(Void)), Attack, Attack],
                &[Attack, Ability(Q(Void)), Attack, Attack],
            ])?
            .end()
    }
}
