use super::*;

impl Generator<ItemData> for ChemtechPutrifierArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
