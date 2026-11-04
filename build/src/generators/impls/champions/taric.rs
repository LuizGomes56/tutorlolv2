use super::*;

impl Generator for Taric {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .ability(Key::E, [("Magic Damage", Void)])
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
