use super::*;

impl Generator for GatheringStorm {
    fn generate(&mut self) -> MayFail {
        // Total adaptive force is `coefficient * m * (m - 1)`, where `m` grows by
        // one every 10 minutes of game time, written in ability power
        let [coefficient] = self
            .description("description")?
            .capture(regex!(r"or ([\d.]+)\*x\*\(x-1\)"))?;

        self.constant("ADAPTIVE_FORCE_COEFFICIENT", coefficient)?;

        Ok(())
    }
}
