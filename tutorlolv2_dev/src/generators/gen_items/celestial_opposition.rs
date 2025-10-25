use super::*;

impl Generator<ItemData> for CelestialOpposition {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
