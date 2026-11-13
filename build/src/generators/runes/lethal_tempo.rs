use super::*;

impl Generator for LethalTempo {
    fn generate(&mut self) -> MayFail {
        let [increase, attack_speed] = self.description("description2")?.capture(regex!(
            r"increased\s+by\s+([\d.]+)%\s+per\s+([\d.]+)%\s+bonus\s+attack\s+speed"
        ))?;

        if attack_speed <= 0.0 {
            return Err("Expected positive bonus attack speed amplification denominator".into());
        }

        let ratio = increase / attack_speed;

        for (attack, name) in [
            (Melee, "MELEE_ATTACK_SPEED_PER_STACK"),
            (Ranged, "RANGED_ATTACK_SPEED_PER_STACK"),
        ] {
            let speed = self.attack_speed_per_stack(attack)?;
            let sample = |mode| -> MayFail<Vec<f64>> {
                (1..=18)
                    .map(|level| {
                        speed
                            .on(mode)
                            .eval(&Sample::new([(Level, level as f64)]))
                            .ok_or_else(|| "Unevaluable per-stack attack speed".into())
                    })
                    .collect()
            };

            self.constant(name, sample(MapMode::Default)?)?;
            self.map_constant(
                name,
                speed
                    .modes()
                    .into_iter()
                    .map(|mode| Ok((mode, sample(mode)?)))
                    .collect::<MayFail<Vec<_>>>()?,
            )?;

            let damage = self.native("description2", attack, SourceForm::Base)?;

            self.assign(
                attack,
                Min,
                f![damage * (1 + ratio * BonusAttackSpeedRatio)],
            );
        }

        self.damage_type(Adaptive).end()
    }
}
