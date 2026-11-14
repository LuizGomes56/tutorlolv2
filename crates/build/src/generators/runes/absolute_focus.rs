use super::*;

impl Generator for AbsoluteFocus {
    fn generate(&mut self) -> MayFail {
        // Adaptive force from level 1 to level 18
        let [min, max] = self
            .description("description")?
            .capture(regex!(r"Gain ([\d.]+) \+ \(([\d.]+)-[\d.]+\)/17\*\(x-1\)"))?;

        self.constant("MIN_ADAPTIVE_FORCE", min)?
            .constant("MAX_ADAPTIVE_FORCE", max)?;

        Ok(())
    }
}
