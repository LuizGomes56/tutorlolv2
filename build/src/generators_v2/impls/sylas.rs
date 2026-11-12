use super::*;

/// The abilities v1 edited are written with v1's formulas, to be read from the wiki
impl GeneratorV2 for Sylas {
    fn generate(&mut self) -> MayFail {
        self.insert(
            P(Min),
            Damage {
                formula: Formula::Raw("0.4 * ctx.attack_damage + 0.2 * ctx.ability_power".into()),
                name: "Petricite Burst".into(),
                damage_type: Magic,
                comment: String::new(),
                attributes: Undefined,
            },
        )?
        .insert(
            P(Max),
            Damage {
                formula: Formula::Raw("1.3 * ctx.attack_damage + 0.3 * ctx.ability_power".into()),
                name: "Petricite Burst".into(),
                damage_type: Magic,
                comment: String::new(),
                attributes: Undefined,
            },
        )?
        .take_nth("Magic Damage", 1, Q(Min))?
        .take("Total Magic Damage", Q(Max))?
        .take("Magic Damage", W(Void))?
        .take("Magic Damage", E(Void))?
        .comment(P(Min), "Secondary Target Damage")
        .comment(P(Max), "Primary Target Damage")
        .combos([
            &[Ability(Q(Max)), Ability(W(Void)), Attack, Ability(E(Void))],
            &[
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Ability(W(Void)),
                Attack,
                Attack,
            ],
            &[Ability(E(Void)), Ability(E(Void)), Ability(W(Void)), Attack],
            &[
                Ability(E(Void)),
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Attack,
            ],
            &[
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
                Attack,
            ],
            &[
                Ability(E(Void)),
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
                Attack,
                Attack,
            ],
            &[
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Ability(E(Void)),
                Attack,
            ],
        ])?
        .end()
    }
}
