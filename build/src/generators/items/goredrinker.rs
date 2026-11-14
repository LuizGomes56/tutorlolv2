use super::*;

impl Generator for Goredrinker {
    fn generate(&mut self) -> MayFail {
        // Thirsting Slash. The other scalings are its healing
        self.damage_type(Physical)
            .min(Named("Thirsting Slash"))?
            .end()
    }
}
