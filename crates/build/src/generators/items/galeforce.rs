use super::*;

impl Generator for Galeforce {
    fn generate(&mut self) -> MayFail {
        // Cloudburst: total damage of the 3 missiles, increased by up to 50%
        // against targets with missing health
        let total = self.total_formula(Named("Cloudburst II"))?;
        let amplification = self.maximum_amplification(Named("Cloudburst II"))?;

        self.damage_type(Physical)
            .set_min(&total)
            .set_max(f![(1 + amplification) * total])
            .end()
    }
}
