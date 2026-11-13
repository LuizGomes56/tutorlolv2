use super::*;

impl Generator for Taric {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .take("Magic Damage", E(Void))?
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
