use super::*;

impl Generator<ItemData> for TheCollectorU66 {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
