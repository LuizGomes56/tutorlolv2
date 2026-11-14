use super::*;

impl Generator for NightHarvester {
    fn generate(&mut self) -> MayFail {
        // Soulrend. The other scaling is the movement speed it grants
        self.damage_type(Magic).min(Named("Soulrend"))?.end()
    }
}
