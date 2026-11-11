use super::*;

impl Generator for BloodlettersCurse {
    fn generate(&mut self) -> MayFail {
        // Vile Decay, in percent like the magic penetration of the engine
        let [vile_decay] = self
            .description(Passive)?
            .capture(regex!(r"Each stack inflicts \{\{as\|\{\{fd\|([\d.]+)\}\}% magic resistance reduction\}\}"))?;

        self.constant("VILE_DECAY_MAGIC_REDUCTION_PER_STACK", vile_decay)?;

        Ok(())
    }
}
