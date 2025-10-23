use super::*;

impl Generator<Champion> for Sett {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 1, _2)];
        ability![W, (1, 0, _1)];
        ability![E, (0, 0, _1)];
        ability![R, (1, 0, _1), (1, 1, _2)];
    }
}
