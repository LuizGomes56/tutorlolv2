use super::*;

impl Generator for Heimerdinger {
    fn generate(&mut self) -> MayFail {
        self.insert(
            Q(Void),
            Ability {
                name: "H-28G Evolution Turret".into(),
                damage_type: Magic,
                attributes: Undefined,
                comment: "Pet: H-28G Evolution Turret basic attack".into(),
                damage: f![7 + 4 * (QLevel - 1) + 0.35 * AbilityPower],
            },
        )
        .insert(
            Q(_1),
            Ability {
                name: "H-28Q Apex Turret".into(),
                damage_type: Magic,
                attributes: Undefined,
                comment: "Pet: H-28Q Apex Turret basic attack".into(),
                damage: f![80 + 20 * (RLevel - 1) + 0.35 * AbilityPower],
            },
        )
        .ability_nth(
            1,
            Key::W,
            [
                (2, Min), /* Initial Rocket Magic Damage */
                (1, Max), /* Combined Total Non-Minion Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (2, _1Min), /* Initial Rocket Damage */
                (1, _1Max), /* Combined Total Non-Minion Damage */
            ],
        )
        .ability_nth(1, Key::E, [(0, Void) /* Magic Damage */])
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .combos([
            &[
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(W(Max)),
                Attack,
            ],
            &[
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(W(Max)),
                Attack,
                Ability(Q(Void)),
                Attack,
            ],
            &[
                Ability(Q(Void)),
                Ability(Q(Void)),
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
                Ability(W(Max)),
            ],
            &[
                Ability(Q(Void)),
                Ability(Q(Void)),
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(W(Max)),
            ],
            &[Ability(E(Void)), Ability(W(Max)), Attack],
            &[
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
                Ability(Q(Void)),
                Ability(W(Max)),
                Attack,
            ],
            &[Ability(E(Void)), Ability(Q(Void)), Ability(W(Max)), Attack],
            &[Ability(E(Void)), Ability(W(Max)), Ability(Q(Void)), Attack],
        ])?
        .end()
    }
}
