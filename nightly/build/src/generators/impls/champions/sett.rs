use super::*;

impl Generator for Sett {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Description 2", Void)])
            .ability(
                Key::Q,
                [
                    ("Bonus Physical Damage", Min),
                    ("Total Bonus Physical Damage", Max),
                ],
            )
            .ability(Key::W, [("Damage", Void)])
            .ability(Key::E, [("Physical Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Physical Damage", Max),
                    ("Reduced Damage", Min),
                ],
            )
            .damage_types([P(Void), W(Void), R(Min), R(Max)], Physical)?
            .combos([
                &[Ability(Q(Max)), Ability(E(Void)), Ability(R(Max))],
                &[
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[Attack, Attack, Ability(Q(Max)), Attack, Attack],
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
            ])?
            .end()
    }
}
