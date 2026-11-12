use super::*;

impl GeneratorV2 for Mel {
    fn generate(&mut self) -> MayFail {
        self.insert(
            Q(Min),
            Damage {
                formula: f![60 + 25 * (QLevel - 1) + 0.55 * AbilityPower],
                name: "Radiant Volley".into(),
                damage_type: Magic,
                comment: "Initial Explosion Magic Damage".into(),
                attributes: Undefined,
            },
        )?
        .insert(
            Q(Max),
            Damage {
                formula: f![85
                    + 42 * (QLevel - 1)
                    + 4 * (QLevel - 1) * (QLevel - 2) * 0.5
                    + (0.8 + 0.05 * (QLevel - 1)) * AbilityPower],
                name: "Radiant Volley".into(),
                damage_type: Magic,
                comment: "Total Magic Damage".into(),
                attributes: Undefined,
            },
        )?
        .insert(
            Q(_1),
            Damage {
                formula: f![5 + 2 * (QLevel - 1) + 0.05 * AbilityPower],
                name: "Radiant Volley".into(),
                damage_type: Magic,
                comment: "Magic Damage per Subsequent Explosion".into(),
                attributes: Undefined,
            },
        )?
        .take("Orb Magic Damage", E(Void))?
        .take_nth("Field Magic Damage per Second", 1, E(_1))?
        .take("Magic Damage", R(Void))?
        .set(R(Void), {
            f![125
                + 37.5 * (RLevel - 1)
                + 0.3 * AbilityPower
                + Stacks * (4 + 1.5 * (RLevel - 1) + 0.04 * AbilityPower)]
        })
        .comment(
            R(Void),
            "Magic damage, Stacks = Overwhelm stacks on the target",
        )
        .combos([
            &[
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Attack,
                Attack,
                Attack,
                Ability(Q(Min)),
                Ability(R(Void)),
            ],
            &[Ability(E(Void)), Ability(Q(Max)), Ability(R(Void))],
            &[
                Ability(E(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Attack,
                Attack,
                Ability(Q(Min)),
                Ability(R(Void)),
            ],
            &[
                Ability(Q(Min)),
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Ability(Q(Min)),
                Ability(R(Void)),
            ],
        ])?
        .end()
    }
}
