use super::*;

impl Generator<ItemData> for LegendaryFighterItem {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
