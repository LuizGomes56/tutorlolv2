use super::*;

impl Generator<Champion> for Zac {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 1, _2)];
        ability![W, (0, 0, _1), (0, 1, _2)];
        ability![E, (2, 0, _1)];
        ability![R, (1, 0, _1), (1, 1, _2), (2, 0, _3)];
    }
}
