use super::*;

impl Generator for Ambessa {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Medarda Maxim", 0, P(Void))?
            .take_in("Cunning Sweep", "Physical Damage", Q(Min))?
            .take_in("Sundering Slam", "Physical Damage", Q(_1Min))?
            .take_in("Cunning Sweep", "Increased Physical Damage", Q(Max))?
            .take_in("Sundering Slam", "Increased Physical Damage", Q(_1Max))?
            .take("Physical Damage", W(Min))?
            .take("Increased Physical Damage", W(Max))?
            .take("Physical Damage", E(Min))?
            .take("Total Physical Damage", E(Max))?
            .take("Physical Damage", R(Void))?
            .damage_type(P(Void), Physical);

        // Both sweeps' health ratio, of the increased damage, which is halved elsewhere
        let (mode, health) = self.map_value(Key::Q, "Health ratio")?;

        for (id, share) in [
            (Q(Max), 1.0),
            (Q(_1Max), 1.0),
            (Q(Min), 0.5),
            (Q(_1Min), 0.5),
        ] {
            self.map_ratio(id, mode, &[EnemyMaxHealth], health.clone().times(share))?;
        }

        self.combos([
            &[
                Ability(Q(Min)),
                Ability(E(Max)),
                Attack,
                Ability(Q(Min)),
                Ability(R(Void)),
                Attack,
            ],
            &[
                Ability(W(Max)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(E(Max)),
                Attack,
                Ability(Q(Min)),
            ],
            &[
                Ability(R(Void)),
                Ability(Q(Min)),
                Ability(W(Max)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(E(Max)),
                Attack,
            ],
            &[
                Ability(E(Max)),
                Attack,
                Ability(Q(Min)),
                Ability(R(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Attack,
                Ability(E(Max)),
                Attack,
                Ability(R(Void)),
                Ability(Q(Min)),
            ],
            &[
                Ability(Q(Min)),
                Attack,
                Ability(E(Max)),
                Attack,
                Ability(Q(Min)),
                Ability(W(Max)),
            ],
        ])?
        .end()
    }
}
