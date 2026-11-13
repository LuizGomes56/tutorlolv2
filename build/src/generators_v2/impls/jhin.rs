use super::*;

impl GeneratorV2 for Jhin {
    fn generate(&mut self) -> MayFail {
        self.take("Innate - Death In 4 Acts", P(Void))?
            .comment(P(Void), "Fourth shot bonus damage")
            .take("Physical Damage", Q(Min))?
            .take("Maximum Final Bounce Physical Damage", Q(Max))?
            .take("Physical Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .take("Minimum Physical Damage per Bullet", R(Min))?
            .take("Maximum Physical Damage per Bullet", R(Max))?
            .take("Minimum Fourth Shot Damage", R(_1Min))?
            .take("Maximum Fourth Shot Damage", R(_1Max))?
            .combos([
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Ability(R(Max)),
                    Ability(R(Max)),
                    Ability(R(Max)),
                    Ability(R(Max)),
                ],
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[Ability(Q(Max)), Attack],
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[Attack, Attack, Attack, Ability(Q(Max)), Attack],
                &[
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
