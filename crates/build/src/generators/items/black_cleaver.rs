use super::*;

impl Generator for BlackCleaver {
    fn generate(&mut self) -> MayFail {
        let description = self.description(Named("Carve"))?;
        let [max_stacks] = description.capture(regex!(r"stacking up to (\d+) times"))?;
        // Carve, in percent like the armor penetration of the engine
        let [carve] = description.capture(regex!(
            r"Each stack inflicts \{\{as\|(\d+)% armor reduction\}\}"
        ))?;

        self.constant("CARVE_ARMOR_REDUCTION_PER_STACK", carve)?;
        self.constant("CARVE_MAX_STACKS", max_stacks)?;

        Ok(())
    }
}
