use super::*;

impl Generator for Hubris {
    fn generate(&mut self) -> MayFail {
        // Eminence
        let [melee_ad, ranged_ad, melee_ad_per_stack, ranged_ad_per_stack] = self
            .description(Passive)?
            .capture(regex!(r"\{\{as\|\{\{rd\|(\d+)\|(\d+)\}\}\|ad\}\} \(\+ \{\{rd\|(\d+)\|(\d+)\}\} per stack\)"))?;

        self.constant("EMINENCE_MELEE_AD", melee_ad)?
            .constant("EMINENCE_RANGED_AD", ranged_ad)?
            .constant("EMINENCE_MELEE_AD_PER_STACK", melee_ad_per_stack)?
            .constant("EMINENCE_RANGED_AD_PER_STACK", ranged_ad_per_stack)?;

        Ok(())
    }
}
