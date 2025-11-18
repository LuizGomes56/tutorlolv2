use super::*;

impl Generator<ItemData> for CelestialOpposition {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
