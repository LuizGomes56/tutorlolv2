use super::*;

impl Generator<ItemData> for ElixirofIron {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
