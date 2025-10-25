use super::*;

impl Generator<Champion> for Fizz {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![W, (0, 0, _1), (1, 0, _2), (2, 0, _3), (2, 1, _4)];
        ability![E, (1, 0, _1), (0, 0, _2)];
        ability![R, (1, 0, _1), (3, 0, _2), (4, 0, _3)];
    }
}
