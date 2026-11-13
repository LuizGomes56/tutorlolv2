use super::*;

impl Generator for BladeOfTheRuinedKing {
    fn generate(&mut self) -> MayFail {
        let ratio = |attack_type| -> MayFail<f64> {
            self.effect_for(Passive, attack_type)?.scalings[0]
                .try_get_value()
                .ok_or(format!("Unknown {attack_type:?} percent scaling").into())
        };

        let melee_ratio = ratio(Melee)?;
        let ranged_ratio = ratio(Ranged)?;

        let damage = |scaling: f64| {
            let modifier = f![(1 - scaling * PhysicalMultiplier)];
            f![EnemyCurrentHealth
                - ((scaling * EnemyCurrentHealth * modifier - AttackDamage
                    + AttackDamage * modifier)
                    / scaling)]
        };

        // Modes that change Mist's Edge, by attack type
        let mut map_damages = Vec::new();

        for attack_type in [Melee, Ranged] {
            for (mode, effect) in self.map_effects_for(Passive, attack_type)? {
                let ratio = effect.scalings[0].try_get_value().ok_or(format!(
                    "Unknown {attack_type:?} percent scaling on {mode:?}"
                ))?;

                map_damages.push((mode, attack_type, damage(ratio)));
            }
        }

        for (mode, attack_type, damage) in map_damages {
            self.map_assign(mode, attack_type, Min, damage);
        }

        self.damage_type(Physical)
            .assign(Melee, Min, damage(melee_ratio))
            .assign(Ranged, Min, damage(ranged_ratio))
            .end()
    }
}
