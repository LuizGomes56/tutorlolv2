use super::*;

impl Generator<Champion> for RekSai {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 0, _2)];
        ability![W, (0, 0, _1)];
        ability![E, (0, 0, _1), (1, 0, _2)];
        ability![R, (0, 0, _1)];
    }
}
