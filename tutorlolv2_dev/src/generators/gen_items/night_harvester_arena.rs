use super::*;

impl Generator<ItemData> for NightHarvesterArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
