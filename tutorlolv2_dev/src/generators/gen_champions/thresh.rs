use super::*;

impl Generator<Champion> for Thresh {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (2, 0, _1)];
        ability![E, (0, 0, _1), (1, 0, _2), (1, 1, _3)];
        ability![R, (0, 0, _1)];
    }
}
