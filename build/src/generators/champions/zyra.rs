use super::*;

impl Generator for Zyra {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage", R(Void))?
            .insert(
                P(Void),
                Damage {
                    name: "Garden of Thorns".into(),
                    damage_type: Magic,
                    attributes: Undefined,
                    comment: "Pet: Thorn Spitter / Vine Lasher basic attack".into(),
                    formula: f![15 + 60 / 17 * (Level - 1) + 0.2 * AbilityPower],
                },
            )?
            .combos([
                &[Ability(R(Void)), Ability(E(Void)), Ability(Q(Void))],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(R(Void)), Attack],
                &[Ability(R(Void)), Ability(Q(Void))],
                &[Ability(Q(Void)), Ability(E(Void)), Ability(R(Void))],
                &[Ability(Q(Void)), Ability(E(Void))],
                &[Ability(E(Void)), Ability(Q(Void))],
            ])?
            .end()
    }
}
