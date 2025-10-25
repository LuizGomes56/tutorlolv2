use super::*;

impl Generator<Champion> for Jayce {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 0, _2), (1, 0, _3)];
        ability![W, (0, 0, _1), (0, 1, _2), (0, 0, _3)];
        ability![E, (0, 0, _1), (0, 1, _2)];
    }
}
