use super::*;

impl Generator for XinZhao {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1), (0, 1, _2));
        ability!(W, (0, 0, _1), (0, 1, _2), (0, 2, _3), (0, 3, _4));
        ability!(E, (0, 0, _1));
        ability!(R, (1, 0, _1));
    }
}