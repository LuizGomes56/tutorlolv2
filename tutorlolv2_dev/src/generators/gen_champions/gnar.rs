use super::*;

impl Generator<Champion> for Gnar {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 1, _2), (0, 0, _3)];
        ability![W, (2, 0, _1), (0, 0, _2)];
        ability![E, (4, 0, _1), (0, 0, _2)];
        ability![R, (0, 0, _1), (1, 1, _2)];
    }
}
