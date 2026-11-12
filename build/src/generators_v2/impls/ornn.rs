use super::*;

impl GeneratorV2 for Ornn {
    fn generate(&mut self) -> MayFail {
        // Master Craftsman bonus to his bonus armor, magic resist and health
        let [base, per_upgrade, max] = self
            .description(Key::P, "Innate - Master Craftsman")?
            .capture(
                regex!(r"by (\d+)% from all sources, further increased by (\d+)% each time .*total increase of (\d+)%"),
            )?;

        self.constant("MASTER_CRAFTSMAN_BASE_BONUS", base / 100.0)?
            .constant("MASTER_CRAFTSMAN_UPGRADE_BONUS", per_upgrade / 100.0)?
            .constant("MASTER_CRAFTSMAN_MAX_BONUS", max / 100.0)?
            .take_nth("Innate - Temper", 0, P(Void))?
            .take("Physical Damage", Q(Void))?
            .take("Magic Damage Per Tick", W(Min))?
            .take("Total Magic Damage", W(Max))?
            .take("Physical Damage", E(Void))?
            .take("Magic Damage", R(Min))?
            .take("Total Magic Damage", R(Max))?
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(R(Min)),
                    Ability(W(Max)),
                    Attack,
                    Ability(R(Min)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[Ability(R(Min)), Ability(E(Void)), Ability(R(Min)), Attack],
                &[
                    Ability(R(Min)),
                    Ability(Q(Void)),
                    Ability(R(Min)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[Attack, Ability(Q(Void)), Attack, Ability(E(Void)), Attack],
                &[Attack, Ability(W(Max)), Attack],
                &[
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(W(Max)), Attack],
            ])?
            .end()
    }
}
