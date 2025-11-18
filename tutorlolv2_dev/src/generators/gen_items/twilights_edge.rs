use super::*;

impl Generator<ItemData> for TwilightsEdge {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
