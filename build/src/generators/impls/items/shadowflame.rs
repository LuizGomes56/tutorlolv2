use super::*;

impl Generator for Shadowflame {
    fn generate(&mut self) -> MayFail {
        // Cinderbloom
        let [cinderbloom_crit_damage, cinderbloom_health_threshold] =
            self.description(Passive)?.capture(
                r"\{\{as\|(\d+)%\|critical damage\}\} damage against enemies \{\{as\|below (\d+)%",
            )?;

        self.constant("CINDERBLOOM_CRIT_DAMAGE", cinderbloom_crit_damage / 100.0)?
            .constant(
                "CINDERBLOOM_HEALTH_THRESHOLD",
                cinderbloom_health_threshold / 100.0,
            )?;

        Ok(())
    }
}
