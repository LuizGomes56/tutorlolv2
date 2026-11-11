use super::*;

impl Generator for Dragonheart {
    fn generate(&mut self) -> MayFail {
        // Inner Flame. Stacks are left unbounded, although the wiki caps them at 7 souls
        let [inner_flame] = self
            .description(Passive)?
            .capture(regex!(r"and \{\{as\|ability haste\}\} by (\d+)% for each \{\{bi\|Dragon Soul\}\}"))?;

        self.constant("INNER_FLAME_STATS_PER_SOUL", inner_flame / 100.0)?;

        Ok(())
    }
}
