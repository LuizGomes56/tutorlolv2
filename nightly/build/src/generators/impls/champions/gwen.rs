use super::*;

impl Generator for Gwen {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .modify(P(Void), |_| {
                f![(0.01 + 0.00006 * AbilityPower) * EnemyMaxHealth]
            })?
            .comment(P(Void), "A Thousand Cuts on-hit magic damage")?
            .ability(
                Key::Q,
                [
                    ("Minimum Center Damage", Min),
                    ("Maximum Center Damage", Max),
                    ("Center Damage per Snip", _1),
                    ("Final Snip Center Damage", _2),
                ],
            )
            .ability(
                Key::R,
                [
                    ("Damage with  A Thousand Cuts", Min),
                    ("Maximum Total Damage", Max),
                    ("Second Cast Total Damage", _1),
                    ("Third Cast Total Damage", _2),
                ],
            )
            .combos([
                &[
                    Ability(R(Min)),
                    Attack,
                    Attack,
                    Ability(R(_1)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(R(_2)),
                ],
                &[Attack, Attack, Attack, Attack, Ability(Q(Max)), Attack],
                &[
                    Ability(R(Min)),
                    Attack,
                    Ability(R(_1)),
                    Attack,
                    Ability(R(_2)),
                ],
                &[
                    Ability(R(Min)),
                    Attack,
                    Attack,
                    Ability(R(_1)),
                    Attack,
                    Attack,
                    Ability(R(_2)),
                    Ability(Q(Max)),
                    Attack,
                ],
                &[Attack, Attack, Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
