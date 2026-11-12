use super::*;

impl GeneratorV2 for Chogath {
    fn generate(&mut self) -> MayFail {
        let feast_health = self
            .value("Bonus Health Per Stack", Key::R)?
            .default_values()
            .ok_or("[Chogath] Bonus Health Per Stack has no values")?;

        self.constant("FEAST_HEALTH_PER_STACK", feast_health)?
            .take("Magic damage", Q(Void))?
            .take("Magic damage", W(Void))?
            .take("Magic Damage", E(Min))?
            .take("Total Magic Damage", E(Max))?
            .take("Champion True Damage", R(Void))?
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[Ability(Q(Void)), Ability(W(Void)), Ability(R(Void))],
                &[
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(R(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
