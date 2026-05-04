use super::*;

impl Generator for Ashe {
    fn generate(&mut self) -> MayFail {
        self.attr(Area, [R(Void), W(Void)])?
            .combo([Attack, Ability(W(Void)), Ability(R(Void)), Attack])?
            .progress(Stable)
            .end()
    }
}
