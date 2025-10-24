use super::*;

impl Generator<Item> for AmplifyingTome {
    #[item_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Item> {}
}
