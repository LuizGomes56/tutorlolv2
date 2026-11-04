use super::*;

impl Generator for Veigar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .ability(Key::W, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Maximum Magic Damage", Max),
                    ("Minimum Magic Damage", Min),
                ],
            )
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[Ability(W(Void)), Ability(Q(Void)), Ability(R(Max))],
                &[Ability(Q(Void)), Ability(R(Max))],
                &[Ability(W(Void)), Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
