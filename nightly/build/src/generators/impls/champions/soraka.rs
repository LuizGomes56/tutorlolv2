use super::*;

impl Generator for Soraka {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .ability(
                Key::E,
                [
                    ("Magic Damage", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .combos([
                &[Ability(Q(Void)), Ability(E(Max)), Attack, Attack],
                &[Ability(E(Max)), Ability(Q(Void))],
                &[Ability(E(Max)), Ability(Q(Void)), Attack, Attack],
                &[Ability(Q(Void)), Attack, Ability(E(Max))],
            ])?
            .end()
    }
}
