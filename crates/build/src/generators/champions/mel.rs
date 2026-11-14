use super::*;

impl Generator for Mel {
    fn generate(&mut self) -> MayFail {
        // "{{pp|type=… 20px …|4;5;6;7|0 to 3}}" by Golden Eclipse's rank 0 to 3, by the
        // position of its numbers: the values of ranks 1 to 3 follow the icon's size and
        // rank 0's
        let (mode, numbers) = self.map_numbers(Key::P, "Overwhelm additional stack base damage")?;
        let [_, _, ref ranks @ .., _, _] = numbers[..] else {
            return Err("[Mel] P: no values by rank in its Overwhelm change".into());
        };

        let per_stack = Formula::ByRank {
            rank: RLevel,
            values: ranks.to_vec(),
        };

        self.take("Magic Damage per Subsequent Explosion", Q(_1))?
            .take("Maximum Magic Damage", Q(Max))?
            // The first explosion, written in the description
            .take_nth("Active", 0, Q(Min))?
            .comment(Q(Min), "Initial Explosion Magic Damage")
            .take("Orb Magic Damage", E(Void))?
            .take_nth("Field Magic Damage per Second", 1, E(_1))?
            .take("Magic Damage", R(Void))?
            .map_ratio(R(Void), mode, &[Stacks], per_stack)?
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
