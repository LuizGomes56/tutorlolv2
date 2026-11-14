use super::*;

impl Generator for Taliyah {
    fn generate(&mut self) -> MayFail {
        let [shards, subsequent] = self.description(Key::Q, "Active")?.capture(regex!(
            r"barrages (\d+) Stone Shards.*?Subsequent hits deal ([\d.]+)% damage"
        ))?;

        let [empowered] = self
            .description(Key::Q, "Empowered Active")?
            .capture(regex!(r"dealing ([\d.]+)% damage to them"))?;

        let subsequent = subsequent / 100.0;

        self.take("Magic Damage", Q(Min))?
            .map_change(Q(Min), "Initial rock base damage")?
            .map_change(Q(Min), "Initial rock AP ratio")?
            .derive(Q(_1), Q(Min), |q| q.scale(empowered / 100.0))?
            .comment(Q(_1), "Empowered Damage")
            .derive(Q(_2), Q(Min), |q| q.scale(subsequent))?
            .comment(Q(_2), "Reduced Damage")
            .derive(Q(Max), Q(Min), |q| {
                q.scale(1.0 + (shards - 1.0) * subsequent)
            })?
            .comment(Q(Max), "Total Magic Damage")
            .take("Magic Damage", E(Void))?
            .take("Detonation Magic Damage", E(_1))?
            .take("Total Maximum Detonation Damage", E(_2))?
            .end()
    }
}
