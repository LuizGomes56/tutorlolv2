use super::*;

impl Generator for Rumble {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Overheated", Void)])
            .modify(P(Void), |_| {
                f![5 + 39.12 / 17 * (Level - 1) + 0.25 * AbilityPower + 0.04 * EnemyMaxHealth]
            })?
            .comment(P(Void), "Overheated bonus magic damage on-hit")?
            .ability(
                Key::Q,
                [
                    ("Minimum Magic Damage", Min),
                    ("Maximum Magic Damage", Max),
                    ("Minimum Enhanced Damage", _1Min),
                    ("Maximum Enhanced Damage", _1Max),
                ],
            )
            .ability(
                Key::E,
                [
                    ("Magic Damage", Min),
                    ("Total Magic Damage", Max),
                    ("Enhanced Damage", _1Min),
                    ("Total Enhanced Damage", _1Max),
                ],
            )
            .ability(
                Key::R,
                [
                    ("Magic Damage per Second", Min),
                    ("Maximum Magic Damage", Max),
                ],
            )
            .combos([
                &[
                    Ability(R(Max)),
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[Ability(E(Max)), Ability(Q(Max)), Attack, Attack, Attack],
                &[
                    Ability(Q(Max)),
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
            ])?
            .end()
    }
}
