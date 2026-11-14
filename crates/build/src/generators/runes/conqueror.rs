use super::*;

impl Generator for Conqueror {
    fn generate(&mut self) -> MayFail {
        // Adaptive force per stack, from level 1 to level 18
        let [min, max] = self.description("description2")?.capture(regex!(
            r"Each stack of Conqueror grants ([\d.]+) \+ \(([\d.]+)-[\d.]+\)/17\*\(x-1\)"
        ))?;

        self.constant("MIN_ADAPTIVE_FORCE_PER_STACK", min)?
            .constant("MAX_ADAPTIVE_FORCE_PER_STACK", max)?;

        Ok(())
    }
}
