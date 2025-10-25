use super::*;

impl Generator<Champion> for Azir {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 0, _1)];
        ability![W, (3, 0, _1)];
        ability![E, (1, 0, _1)];
        ability![R, (1, 0, _1)];
    }
}
