use super::*;

impl Generator<Champion> for Illaoi {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 0, _1)];
        ability![W, (3, 0, _1), (3, 1, _2)];
        ability![E, (3, 0, _1)];
        ability![R, (0, 0, _1)];
    }
}
