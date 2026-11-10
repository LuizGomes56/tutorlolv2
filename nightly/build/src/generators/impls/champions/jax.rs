use super::*;

impl Generator for Jax {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Physical Damage", Void)])
            .ability(Key::W, [("Additional Magic Damage", Void)])
            .ability(
                Key::E,
                [
                    ("Maximum Magic Damage", Max),
                    ("Minimum Magic Damage", Min),
                ],
            )
            .ability(
                Key::R,
                [
                    ("Additional Magic Damage", _1),
                    ("Magic Damage", Void),
                ],
            )
            .damage_type(Q(Void), Physical)?
            .combos([
                &[
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                ],
                &[
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                ],
                &[Attack, Ability(Q(Void)), Attack, Ability(W(Void))],
                &[
                    Ability(E(Min)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Min)),
                    Attack,
                ],
            ])?
            .end()
    }
}
