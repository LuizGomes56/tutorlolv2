use super::*;

impl Generator for Nautilus {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate */])
            .ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, Min), /* Magic Damage per Instance */
                    (2, Max), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, Min), /* Magic Damage */
                    (1, Max), /* Maximum Total Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, Max), /* Increased Damage */
                    (2, Min), /* Magic Damage */
                ],
            )
            .combos([
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(W(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(W(Max)),
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                ],
                &[Attack, Ability(Q(Void)), Ability(E(Max))],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                ],
            ])?
            .end()
    }
}
