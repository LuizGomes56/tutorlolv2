use super::*;

impl Generator<ItemData> for RapidRabbitRaindown {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
