use super::*;

impl Generator<Champion> for Taric {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![E, (0, 0, _1)];
    }
}
