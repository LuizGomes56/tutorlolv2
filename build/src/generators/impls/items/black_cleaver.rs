use super::*;

impl Generator for BlackCleaver {
    fn generate(&mut self) -> MayFail {
        // Carve, in percent like the armor penetration of the engine
        let [carve] = self
            .description(Passive)?
            .capture(regex!(r"Each stack inflicts \{\{as\|(\d+)% armor reduction\}\}"))?;

        self.constant("CARVE_ARMOR_REDUCTION_PER_STACK", carve)?;

        Ok(())
    }
}
