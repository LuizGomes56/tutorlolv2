use super::*;

impl Generator for Malphite {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Additional Physical Damage */
                    (3, _2), /* Physical Damage */
                ],
            )
            .ability(Key::E, [(1, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .damage_types([W(_1), W(_2)], Physical)?
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(_1)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Ability(W(_1)),
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(_1)),
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(_1)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(_1)),
                ],
                &[
                    Ability(R(Void)),
                    Attack,
                    Ability(W(_1)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(_1)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(_1)),
                    Attack,
                ],
            ])?
            .end()
    }
}
