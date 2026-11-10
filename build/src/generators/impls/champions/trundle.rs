use super::*;

impl Generator for Trundle {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Bonus Physical Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Initial Magic Damage", Min),
                    ("Magic Damage Per Second", Void),
                    ("Total Magic Damage", Max),
                ],
            )
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
