use super::*;

impl Generator for Gnar {
    fn generate(&mut self) -> MayFail {
        self.ability_nth(
            1,
            Key::Q,
            [
                (0, Max), /* Physical Damage */
                (1, Min), /* Reduced Damage */
            ],
        )
        .ability(Key::Q, [(0, Mega) /* Physical Damage */])
        .ability_nth(1, Key::W, [(0, Void) /* Bonus Magic Damage */])
        .ability(Key::W, [(0, Mega) /* Physical Damage */])
        .ability_nth(1, Key::E, [(1, Void) /* Physical Damage */])
        .ability(Key::E, [(0, Mega) /* Physical Damage */])
        .ability(
            Key::R,
            [
                (2, Max), /* Increased Damage */
                (3, Min), /* Physical Damage */
            ],
        )
        .combos([
            &[Ability(E(Void)), Ability(R(Max))],
            &[
                Ability(E(Void)),
                Attack,
                Ability(R(Max)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
            ],
            &[
                Ability(E(Void)),
                Ability(R(Max)),
                Attack,
                Ability(Q(Max)),
                Attack,
            ],
            &[
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
            ],
            &[Attack, Ability(E(Void)), Ability(Q(Max)), Attack],
        ])?
        .end()
    }
}
