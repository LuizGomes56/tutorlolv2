use super::*;

impl Generator for Akali {
    fn generate(&mut self) -> MayFail {
        self.take("Swinging Kama", P(Void))?
            .take("Magic Damage", Q(Void))?
            .take_nth("Magic Damage", 1, E(Min))?
            .take("Total Magic Damage", E(Max))?
            .take("Magic Damage", R(_1))?
            .map_change(R(_1), "AP ratio")?
            .map_change(R(_1), "First dash AP ratio")?
            .take("Maximum Magic Damage", R(Max))?
            .take("Minimum Magic Damage", R(Min))?
            .map_change(R(Min), "AP ratio")?
            .map_change(R(Min), "Second dash minimum AP ratio")?;

        // The second dash deals up to 3 times its minimum
        for subject in ["AP ratio", "Second dash minimum AP ratio"] {
            let (mode, ratio) = self.map_value(Key::R, subject)?;
            self.map_ratio(R(Max), mode, &[AbilityPower], ratio.times(3.0))?;
        }

        self.combos([
            &[
                Ability(R(_1)),
                Ability(E(Min)),
                Ability(E(Min)),
                Attack,
                Ability(Q(Void)),
                Ability(R(Min)),
            ],
            &[
                Ability(R(_1)),
                Ability(E(Min)),
                Ability(Q(Void)),
                Ability(E(Min)),
                Attack,
                Ability(R(Min)),
            ],
            &[
                Ability(R(_1)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Ability(E(Min)),
                Ability(E(Min)),
                Attack,
                Ability(Q(Void)),
                Ability(R(Min)),
            ],
            &[Ability(E(Min)), Ability(E(Min)), Ability(Q(Void)), Attack],
            &[
                Ability(R(_1)),
                Ability(E(Min)),
                Attack,
                Ability(Q(Void)),
                Ability(E(Min)),
                Attack,
                Ability(Q(Void)),
                Ability(R(Min)),
            ],
            &[
                Ability(Q(Void)),
                Attack,
                Ability(E(Min)),
                Ability(E(Min)),
                Attack,
            ],
            &[
                Ability(E(Min)),
                Ability(E(Min)),
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
                Ability(E(Min)),
                Ability(E(Min)),
                Attack,
                Ability(Q(Void)),
                Attack,
            ],
        ])?
        .end()
    }
}
