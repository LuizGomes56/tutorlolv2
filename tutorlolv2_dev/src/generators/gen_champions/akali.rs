use super::*;

// #![stable]

impl Generator<Champion> for Akali {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![P::Void, (0, 1)];
        ability![Q, (0, 0, Void)];
        ability![E, (0, 0, Min), (2, 0, Max), (2, 1, Max)];
        ability![R, (0, 0, _1), (2, 0, _2Min), (2, 1, _2Max)];
        merge![E::Min - E::Max, R::_2Min - R::_2Max];
    }
}
