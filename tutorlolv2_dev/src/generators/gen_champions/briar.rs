use super::*;

impl Generator<Champion> for Briar {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![W, (2, 2, _1), (0, 0, _2), (1, 0, _3)];
        ability![E, (2, 0, _1), (2, 1, _2), (3, 0, _3), (3, 1, _4)];
        ability![R, (3, 0, _1)];
    }
}
