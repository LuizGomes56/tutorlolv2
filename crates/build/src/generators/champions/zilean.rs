use super::*;

impl Generator for Zilean {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .combos([
                &[Ability(Q(Void)), Ability(Q(Void))],
                &[Attack, Ability(Q(Void)), Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
