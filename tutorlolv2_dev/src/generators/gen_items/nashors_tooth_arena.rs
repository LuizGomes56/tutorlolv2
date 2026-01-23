use super::*;

// #![stable]

impl Generator<ItemData> for NashorsToothArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        ItemFactory::run(ItemId::NashorsTooth)
    }
}
