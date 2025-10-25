use super::*;

impl Generator<Champion> for Milio {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (3, 0, _1)];
    }
}
