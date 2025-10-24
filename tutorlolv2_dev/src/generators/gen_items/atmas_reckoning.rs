use super::*;

impl Generator<Item> for AtmasReckoning {
    #[item_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Item> {}
}
