use super::*;

impl GeneratorV2 for Poppy {
    fn generate(&mut self) -> MayFail {
        let [bonus, low_health_bonus, low_health_threshold] =
            self.exception("Stubborn to a Fault")?.capture(regex!(
                r"by (\d+)%, doubled to (\d+)% while she is below (\d+)% maximum health"
            ))?;

        self.constant("STUBBORN_TO_A_FAULT_RESISTS", bonus / 100.0)?
            .constant(
                "STUBBORN_TO_A_FAULT_LOW_HEALTH_RESISTS",
                low_health_bonus / 100.0,
            )?
            .constant(
                "STUBBORN_TO_A_FAULT_HEALTH_THRESHOLD",
                low_health_threshold / 100.0,
            )?
            .take_nth("Innate", 0, P(Void))?
            .take("Physical Damage", Q(Min))?
            .take("Total Physical Damage", Q(Max))?
            .take("Magic Damage", W(Void))?
            .take("Physical Damage", E(Min))?
            .take("Total Physical Damage", E(Max))?
            .take("Increased Damage", R(Min))?
            .take("Physical Damage", R(Max))?
            .combos([
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Max)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(Q(Max)),
                    Ability(R(Max)),
                    Attack,
                ],
                &[Attack, Ability(E(Max)), Attack, Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
