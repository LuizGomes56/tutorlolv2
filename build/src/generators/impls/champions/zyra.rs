use super::*;

impl Generator for Zyra {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .insert(
                P(Void),
                Ability {
                    name: "Garden of Thorns".into(),
                    damage_type: Magic,
                    attributes: Undefined,
                    comment: "Pet: Thorn Spitter / Vine Lasher basic attack".into(),
                    damage: f![15 + 60 / 17 * (Level - 1) + 0.2 * AbilityPower],
                },
            )
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
