use super::*;

impl Generator for Poppy {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate */])
            .ability(
                Key::Q,
                [
                    (2, Min), /* Physical Damage */
                    (5, Max), /* Total Physical Damage */
                ],
            )
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Min), /* Physical Damage */
                    (2, Max), /* Total Physical Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, Min), /* Increased Damage */
                    (1, Max), /* Physical Damage */
                ],
            )
            .combos([
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Max)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(Q(Max)),
                    Ability(R(Max)),
                    Attack,
                ],
                &[Attack, Ability(E(Max)), Attack, Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
