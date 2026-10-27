use super::*;

impl Generator for Taric {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Innate */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .combos([
                &[Ability(E(Void)), Attack, Attack, Attack, Attack],
                &[
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
