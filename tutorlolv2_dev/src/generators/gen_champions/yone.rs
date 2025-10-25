use super::*;

impl Generator<Champion> for Yone {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![W, (0, 0, _1), (0, 1, _2), (0, 2, _3)];
        ability![E, (3, 0, _1)];
        ability![R, (1, 0, _1), (1, 1, _2), (1, 2, _3)];
    }
}
