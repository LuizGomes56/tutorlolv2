use super::*;

impl Generator for Hecarim {
    fn generate(&mut self) -> MayFail {
        /* Rampage bonus damage per stack */
        let rampage = |dmg: &str| f![(dmg) * Stacks * (1.03 + 0.03 * BonusAd / 100)];

        self.ability(Key::Q, [("Physical Damage", Void)])
            .modify(Q(Void), rampage)?
            .map_modify(Q(Void), Key::Q, "Physical Damage", rampage)?
            .ability(
                Key::W,
                [
                    ("Magic Damage Per Tick", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .ability(
                Key::E,
                [
                    ("Maximum Physical Damage", Max),
                    ("Minimum Physical Damage", Min),
                ],
            )
            .ability(Key::R, [("Magic damage", Void)])
            .combos([
                &[
                    Ability(E(Min)),
                    Ability(W(Max)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Max)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(R(Void)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(W(Max)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(W(Max)),
                    Ability(E(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
