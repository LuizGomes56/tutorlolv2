use super::*;

impl GeneratorV2 for Lulu {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Min))?
            .take("Total Magic Damage", Q(Max))?
            .take("Reduced Damage", Q(_1))?
            .insert(
                E(Void),
                Damage {
                    name: "Help, Pix!".into(),
                    damage_type: Magic,
                    attributes: Undefined,
                    comment: "Magic Damage: 70 / 110 / 150 / 190 / 230 (+ 50% AP)".into(),
                    formula: f![70 + 40 * (ELevel - 1) + 0.5 * AbilityPower],
                },
            )?
            .combos([
                &[Attack, Ability(E(Void)), Attack, Ability(Q(Min)), Attack],
                &[Ability(E(Void)), Ability(Q(Min))],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[Ability(Q(Min)), Attack, Ability(E(Void))],
                &[Ability(E(Void)), Attack, Ability(Q(Min))],
                &[Ability(E(Void)), Ability(Q(Min)), Attack, Attack],
                &[Ability(Q(Min)), Ability(E(Void)), Attack],
                &[Ability(E(Void)), Attack, Ability(Q(Min)), Attack],
            ])?
            .end()
    }
}
