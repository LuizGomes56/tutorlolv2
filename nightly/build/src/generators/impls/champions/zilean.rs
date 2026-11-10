use super::*;

impl Generator for Zilean {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .combos([
                &[Ability(Q(Void)), Ability(Q(Void))],
                &[Attack, Ability(Q(Void)), Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
