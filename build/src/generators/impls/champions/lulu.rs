use super::*;

impl Generator for Lulu {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1Min), /* Magic Damage */
                (2, Min),   /* Reduced Damage */
                (4, _1Max), /* Total Magic Damage */
            ],
        )
        .insert(
            E(Void),
            Ability {
                name: "Help, Pix!".into(),
                damage_type: Magic,
                attributes: Undefined,
                comment: "Magic Damage: 70 / 110 / 150 / 190 / 230 (+ 50% AP)".into(),
                damage: 70.plus(ELevel).times(40).plus(0.5).times(AbilityPower),
            },
        )
        .end()
    }
}
