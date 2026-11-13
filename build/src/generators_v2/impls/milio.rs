use super::*;

impl GeneratorV2 for Milio {
    fn generate(&mut self) -> MayFail {
        let burn = self.value_nth("Innate", 1, Key::P)?;

        // The enchantment's damage, then its whole burn
        self.take_nth("Innate", 0, P(Void))?
            .edit(P(Void), |p| f![p + burn])
            .take("Magic Damage", Q(Void))?
            .combos([&[Ability(Q(Void)), Attack, Attack, Attack, Attack]])?
            .end()
    }
}
