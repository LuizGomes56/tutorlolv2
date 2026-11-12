use super::*;

impl GeneratorV2 for Heimerdinger {
    fn generate(&mut self) -> MayFail {
        self.insert(
            Q(Void),
            Damage {
                name: "H-28G Evolution Turret".into(),
                damage_type: Magic,
                attributes: Undefined,
                comment: "Pet: H-28G Evolution Turret basic attack".into(),
                formula: f![7 + 4 * (QLevel - 1) + 0.35 * AbilityPower],
            },
        )?
        .insert(
            Q(_1),
            Damage {
                name: "H-28Q Apex Turret".into(),
                damage_type: Magic,
                attributes: Undefined,
                comment: "Pet: H-28Q Apex Turret basic attack".into(),
                formula: f![80 + 20 * (RLevel - 1) + 0.35 * AbilityPower],
            },
        )?
        .take("Initial Rocket Magic Damage", W(Min))?
        .take_in(
            "Hextech Micro-Rockets",
            "Combined Total Non-Minion Damage",
            W(Max),
        )?
        .take("Initial Rocket Damage", W(_1Min))?
        .take_in(
            "Hextech Rocket Swarm",
            "Combined Total Non-Minion Damage",
            W(_1Max),
        )?
        .take_in("CH-2 Electron Storm Grenade", "Magic Damage", E(Void))?
        .take_in("CH-3X Lightning Grenade", "Magic Damage", E(_1))?
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
