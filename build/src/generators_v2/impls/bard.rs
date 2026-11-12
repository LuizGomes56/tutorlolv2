use super::*;

impl GeneratorV2 for Bard {
    fn generate(&mut self) -> MayFail {
        self.insert(
            P(Void),
            Damage {
                formula: f![30 + 6 * Stacks + 0.4 * AbilityPower],
                name: "Traveler's Call".into(),
                damage_type: Magic,
                comment: "Meeps bonus magic damage".into(),
                attributes: Undefined,
            },
        )?
        .take("Magic Damage", Q(Void))?
        .combos([
            &[Ability(Q(Void)), Attack],
            &[Attack, Ability(Q(Void)), Attack],
        ])?
        .end()
    }
}
