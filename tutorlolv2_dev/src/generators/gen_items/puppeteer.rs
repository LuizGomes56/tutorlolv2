use super::*;

impl Generator<ItemData> for Puppeteer {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
