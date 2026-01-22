use super::*;

impl Generator<ItemData> for NightHarvester {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
