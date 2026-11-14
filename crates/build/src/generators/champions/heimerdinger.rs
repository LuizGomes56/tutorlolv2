use super::*;

impl Generator for Heimerdinger {
    fn generate(&mut self) -> MayFail {
        // The turrets' bases by rank, which map changes replace
        let (turret, turret_type) = self.pet("H-28G Evolution Turret", Key::Q)?;
        let turret = self.map_changed(Key::Q, "Turret base damage", turret)?;
        let turret = self.map_changed(Key::Q, "Turret AP ratio", turret)?;

        let (apex, apex_type) = self.pet("H-28Q Apex Turret", Key::R)?;
        let apex = self.map_changed(
            Key::R,
            "{{ai|H-28Q Apex Turret|Heimerdinger}} base damage",
            apex,
        )?;

        self.insert(
            Q(Void),
            Damage {
                name: "H-28G Evolution Turret".into(),
                damage_type: turret_type,
                attributes: Undefined,
                comment: "Pet: H-28G Evolution Turret basic attack".into(),
                formula: turret,
            },
        )?
        .insert(
            Q(_1),
            Damage {
                name: "H-28Q Apex Turret".into(),
                damage_type: apex_type,
                attributes: Undefined,
                comment: "Pet: H-28Q Apex Turret basic attack".into(),
                formula: apex,
            },
        )?
        .take("Initial Rocket Magic Damage", W(Min))?
        .map_change(W(Min), "Initial damage AP ratio")?
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
        // Arena's initial rocket AP ratio is of the basic W only: the model also
        // scaled the empowered one's by it
        .edit(W(_1Min), |w| w.on(MapMode::Default));

        let initial = self.damage(W(Min))?;
        let subsequent = self.value("Subsequent Rocket Magic Damage", Key::W)?;

        // The initial rocket and the subsequent ones of its wave
        let [rockets] = self
            .description(Key::W, "Active")?
            .capture(regex!(r"launches a wave of (\d+) rockets"))?;

        let subsequent_rockets = rockets - 1.0;

        self.set(W(Max), f![initial + subsequent_rockets * subsequent])
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
