use super::*;

impl Generator for Janna {
    fn generate(&mut self) -> MayFail {
        self.attr(Area, [Q(Min), Q(Max)])?
            .combo([Ability(W(Void)), Attack, Ability(Q(Max)), Attack])?
            .progress(Stable)
            .end()
    }
}
