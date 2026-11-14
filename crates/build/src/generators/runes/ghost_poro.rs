use super::*;

impl Generator for GhostPoro {
    fn generate(&mut self) -> MayFail {
        // Adaptive force per stack, up to a maximum, and a bonus at the last stack
        let [per_stack, max, completion] = self
            .description("description3")?
            .capture(regex!(r"Gain (\d+) for every Ghost Poro.*?for a maximum of (\d+)\. After reaching \d+ stacks, additionally gain (\d+)"))?;

        if per_stack <= 0.0 {
            return Err("[GhostPoro] Expected positive adaptive force per stack".into());
        }

        self.constant("ADAPTIVE_FORCE_PER_STACK", per_stack)?
            .constant("MAX_STACKS", (max / per_stack).round())?
            .constant("COMPLETION_ADAPTIVE_FORCE", completion)?;

        Ok(())
    }
}
