use super::*;

impl Generator for Milio {
    fn generate(&mut self) -> MayFail {
        let burn = self.value_nth("Innate", 1, Key::P)?;
        let burn = self.map_changed(Key::P, "Base burn damage", burn)?;

        // The enchantment's damage, then its whole burn
        self.take_nth("Innate", 0, P(Void))?
            .edit(P(Void), |p| f![p + burn])
            .take("Magic Damage", Q(Void))?
            .combos([&[Ability(Q(Void)), Attack, Attack, Attack, Attack]])?
            .end()
    }
}
