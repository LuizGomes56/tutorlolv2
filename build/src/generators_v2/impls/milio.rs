use super::*;

impl GeneratorV2 for Milio {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            // v1's value, with the burn's AP ratio (tests/passive_regressions.rs)
            .edit(P(Void), |p| f![p + 0.2 * AbilityPower])
            .take("Magic Damage", Q(Void))?
            .combos([&[Ability(Q(Void)), Attack, Attack, Attack, Attack]])?
            .end()
    }
}
