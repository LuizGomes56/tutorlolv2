use super::*;

impl Generator for Sion {
    fn generate(&mut self) -> MayFail {
        self.ability_nth(1, Key::P, [("Description 1", Void)])
            .modify(P(Void), |dmg| f![(dmg) / 10])?
            .damage_type(P(Void), Physical)?
            .comment(P(Void), "Glory in Death bonus damage on-hit")?
            .ability(
                Key::Q,
                [
                    ("Maximum Physical Damage", Max),
                    ("Minimum Physical Damage", Min),
                ],
            )
            .ability(Key::W, [("Magic Damage", Void)])
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Maximum Physical Damage", Max),
                    ("Minimum Physical Damage", Min),
                ],
            )
            .combos([
                &[
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
            ])?
            .end()
    }
}
