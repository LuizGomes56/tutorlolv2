use super::*;

impl Generator<Champion> for Zeri {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 1, _2)];
        ability![W, (0, 0, _1)];
        ability![E, (1, 0, _1), (1, 1, _2)];
        ability![R, (0, 0, _1)];
    }
}
