use super::*;

impl Generator for Veigar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, Max), /* Maximum Magic Damage */
                    (1, Min), /* Minimum Magic Damage */
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
