use super::*;

impl Generator for Lulu {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Min), /* Magic Damage */
                (4, Max), /* Total Magic Damage */
                (2, _1),  /* Reduced Damage */
            ],
        )
        .insert(
            E(Void),
            Ability {
                name: "Help, Pix!".into(),
                damage_type: Magic,
                attributes: Undefined,
                comment: "Magic Damage: 70 / 110 / 150 / 190 / 230 (+ 50% AP)".into(),
                damage: f![70 + 40 * (ELevel - 1) + 0.5 * AbilityPower],
            },
        )
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
