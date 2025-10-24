use super::*;

impl Generator<Item> for BlackCleaver {
    #[item_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Item> {}
}
