use super::*;

impl GeneratorV2 for Mel {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage per Subsequent Explosion", Q(_1))?
            .take("Maximum Magic Damage", Q(Max))?
            // The first explosion, written in the description
            .take_nth("Active", 0, Q(Min))?
            .comment(Q(Min), "Initial Explosion Magic Damage")
            .take("Orb Magic Damage", E(Void))?
            .take_nth("Field Magic Damage per Second", 1, E(_1))?
            .take("Magic Damage", R(Void))?
            .set(R(Void), {
                f![125
                    + 37.5 * (RLevel - 1)
                    + 0.3 * AbilityPower
                    + Stacks * (4 + 1.5 * (RLevel - 1) + 0.04 * AbilityPower)]
            })
            .comment(
                R(Void),
                "Magic damage, Stacks = Overwhelm stacks on the target",
            )
            .combos([
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Void)),
                ],
                &[Ability(E(Void)), Ability(Q(Max)), Ability(R(Void))],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Void)),
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Void)),
                ],
            ])?
            .end()
    }
}
