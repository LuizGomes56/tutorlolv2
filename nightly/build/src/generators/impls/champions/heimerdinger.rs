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
                ("Initial Rocket Magic Damage", Min),
                ("Combined Total Non-Minion Damage", Max),
            ],
        )
        .ability(
            Key::W,
            [
                ("Initial Rocket Damage", _1Min),
                ("Combined Total Non-Minion Damage", _1Max),
            ],
        )
        .ability_nth(1, Key::E, [("Magic Damage", Void)])
        .ability(Key::E, [("Magic Damage", _1)])
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
