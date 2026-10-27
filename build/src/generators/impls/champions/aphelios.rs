use super::*;

impl Generator for Aphelios {
    fn generate(&mut self) -> MayFail {
        // 31 - 100 (+ 40% - 60% bonus AD) from level 1 to 13
        let sentry_base = stringify!(match ctx.level {
            ..=1 => 31,
            2 => 36.75,
            3 => 42.5,
            4 => 48.25,
            5 => 54,
            6 => 59.75,
            7 => 65.5,
            8 => 71.25,
            9 => 77,
            10 => 82.75,
            11 => 88.5,
            12 => 94.25,
            13.. => 100,
        });

        let sentry_ratio = stringify!(match ctx.level {
            ..=1 => 0.4,
            2 => 0.42,
            3 => 0.43,
            4 => 0.45,
            5 => 0.47,
            6 => 0.48,
            7 => 0.5,
            8 => 0.52,
            9 => 0.53,
            10 => 0.55,
            11 => 0.57,
            12 => 0.58,
            13.. => 0.6,
        });

        self.insert(
            P(_1),
            Ability {
                name: "Calibrum".into(),
                damage_type: Physical,
                attributes: Undefined,
                comment: "Calibrum mark bonus damage per mark consumed".into(),
                damage: f![15 + 0.15 * BonusAd],
            },
        )
        .insert(
            P(_2),
            Ability {
                name: "Crescendum".into(),
                damage_type: Physical,
                attributes: Undefined,
                comment: "Crescendum bonus damage, Stacks = Chakrams (up to 20)".into(),
                damage: f![1.46 * Stacks / 20 * AttackDamage],
            },
        )
        .insert(
            Q(Void),
            Ability {
                name: "Sentry".into(),
                damage_type: Physical,
                attributes: Undefined,
                comment: "Pet: Crescendum Sentry basic attack".into(),
                damage: f![(sentry_base) + (sentry_ratio) * BonusAd + 0.5 * AbilityPower],
            },
        )
        .combos([
            &[Ability(Q(Void)), Ability(Q(Void))],
            &[
                Ability(Q(Void)),
                Attack,
                Attack,
                Attack,
                Attack,
                Ability(Q(Void)),
                Attack,
                Attack,
            ],
            &[Ability(Q(Void)), Ability(Q(Void)), Attack],
            &[Ability(Q(Void)), Ability(Q(Void)), Attack, Attack],
            &[Ability(Q(Void)), Attack],
            &[Ability(Q(Void)), Attack, Ability(Q(Void)), Attack],
            &[Ability(Q(Void)), Attack, Attack],
        ])?
        .end()
    }
}
