use super::*;

/// The abilities v1 edited are written with v1's formulas, to be read from the wiki
impl GeneratorV2 for Urgot {
    fn generate(&mut self) -> MayFail {
        self.insert(
                P(Void),
                Damage {
                    formula: Formula::Raw("((match ctx.level { ..6 => 0.4, 6..9 => 0.52, 9..11 => 0.64, 11..13 => 0.76, 13..15 => 0.88, 15.. => 1.0 }) * ctx.attack_damage) + ((match ctx.level { ..4 => 0.02, 4..7 => 0.03, 7..10 => 0.04, 10..13 => 0.05, 13.. => 0.06 }) * ctx.enemy_max_health)".into()),
                    name: "Echoing Flames".into(),
                    damage_type: Physical,
                    comment: String::new(),
                    attributes: Undefined,
                },
            )?
            .take("Physical Damage", Q(Void))?
            .take("Modified Physical Damage", W(Void))?
            .take("Physical Damage", E(Void))?
            .take("Physical Damage", R(Void))?
            .damage_type(R(Void), Physical)
            .comment(P(Void), "Echoing Flames shotgun blast")
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Attack,
                    Ability(W(Void)),
                ],
                &[Ability(E(Void)), Ability(R(Void)), Attack, Ability(W(Void))],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(R(Void)),
                ],
            ])?
            .end()
    }
}
