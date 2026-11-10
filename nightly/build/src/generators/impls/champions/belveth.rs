use super::*;

impl Generator for Belveth {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Physical Damage", Void)])
            .ability(Key::W, [("Magic Damage", Void)])
            .ability(
                Key::E,
                [
                    ("Maximum Physical Damage per hit", Max),
                    ("Minimum Physical Damage per hit", Min),
                ],
            )
            .ability(
                Key::R,
                [
                    ("Bonus True Damage", _1),
                    ("True Damage", Void),
                ],
            )
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Max)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Max)),
                ],
            ])?
            .end()
    }
}
