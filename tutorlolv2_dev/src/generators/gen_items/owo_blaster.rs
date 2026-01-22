use super::*;

impl Generator<ItemData> for OwoBlaster {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
