use super::*;

impl Generator for Reverberation {
    fn generate(&mut self) -> MayFail {
        // Resonate
        self.damage_type(Magic)
            .attr(Onhit)
            .min(Named("Resonate"))?
            .end()
    }
}
