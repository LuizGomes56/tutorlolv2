use super::*;

impl Generator for Tristana {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    // (0, _1), /* Bonus Damage Per Stack */
                    // (1, _2), /* Full Stack Bonus Damage */
                    (2, Max),  /* Full Stack Physical Damage */
                    (3, Void), /* Magic Damage */
                    (4, Min),  /* Minimum Physical Damage */
                ],
            )
            .comment(E(Void), "Passive explosion damage on kill")?
            .ability(Key::R, [(1, Void) /* Magic Damage */])
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[Attack, Ability(E(Void)), Attack, Attack, Attack, Attack],
            ])?
            .end()
    }
}
