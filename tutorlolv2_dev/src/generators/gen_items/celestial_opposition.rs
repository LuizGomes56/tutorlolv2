use super::*;

impl Generator for CelestialOpposition {
    fn generate(&mut self) -> MayFail {
        self.min(Active).end()
    }
}
