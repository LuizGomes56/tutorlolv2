use super::*;

impl Generator<ItemData> for MinionDematerializer {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
