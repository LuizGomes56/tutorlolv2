use super::*;

impl Generator for Ryze {
    fn generate(&mut self) -> MayFail {
        let r = &self.formula(Key::R, "Bonus Overload Damage")?.to_string();

        self.ability(Key::Q, [("Magic Damage", Void)])
            .modify(Q(Void), |dmg| f![(dmg) * r])?
            .ability(Key::W, [("Magic Damage", Void)])
            .ability(Key::E, [("Magic Damage", Void)])
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Ability(E(Void)), Ability(Q(Void))],
                &[Ability(E(Void)), Ability(W(Void)), Attack, Ability(Q(Void))],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(E(Void)), Ability(Q(Void))],
            ])?
            .end()
    }
}
