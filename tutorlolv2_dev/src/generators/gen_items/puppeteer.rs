use super::*;

impl Generator<ItemData> for Puppeteer {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
