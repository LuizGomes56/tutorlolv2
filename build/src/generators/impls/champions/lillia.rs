use super::*;

impl Generator for Lillia {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .modify(P(Void), |_| {
                f![(0.05 + 0.000125 * AbilityPower) * EnemyMaxHealth]
            })?
            .comment(P(Void), "Dream Dust total magic damage over 3 seconds")?
            .ability(
                Key::Q,
                [
                    ("Magic Damage", Min),
                    ("Total Mixed Damage", Max),
                ],
            )
            .damage_type(Q(Min), Magic)?
            .ability(
                Key::W,
                [
                    ("Magic Damage", Min),
                    ("Increased Damage", Max),
                ],
            )
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(Key::R, [("Magic Damage", Void)])
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Ability(W(Max)),
                    Ability(Q(Min)),
                ],
                &[Ability(Q(Max)), Ability(R(Void))],
                &[
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(W(Max)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(W(Max)),
                    Ability(Q(Max)),
                ],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[Ability(E(Void)), Ability(W(Max)), Ability(Q(Max))],
            ])?
            .end()
    }
}
