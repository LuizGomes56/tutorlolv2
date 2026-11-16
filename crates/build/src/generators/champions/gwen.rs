use super::*;

impl Generator for Gwen {
    fn generate(&mut self) -> MayFail {
        let center = |q: Formula| f![(q) * 0.5 * MagicMultiplier + (q) * 0.5 * TrueMultiplier];

        self.take_nth("Innate", 0, P(Void))?
            .comment(P(Void), "A Thousand Cuts on-hit magic damage")
            .take("Minimum Center Damage", Q(Min))?
            .take("Maximum Center Damage", Q(Max))?
            .take("Center Damage per Snip", Q(_1))?
            .take("Final Snip Center Damage", Q(_2))?
            // The center of a snip converts half of its damage to true damage
            .edit(Q(Min), center)
            .edit(Q(Max), center)
            .edit(Q(_1), center)
            .edit(Q(_2), center)
            .take("Damage with A Thousand Cuts", R(Min))?
            .take("Maximum Total Damage", R(Max))?
            .take("Second Cast Total Damage", R(_1))?
            .take("Third Cast Total Damage", R(_2))?
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
