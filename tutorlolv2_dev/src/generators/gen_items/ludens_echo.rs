use super::*;

impl Generator<ItemData> for LudensEcho {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
