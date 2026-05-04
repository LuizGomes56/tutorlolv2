use super::*;

impl Generator for Alistar {
    fn generate(&mut self) -> MayFail {
        self.combo([Ability(W(Void)), Ability(Q(Void)), Attack, Ability(E(Max))])?
            .progress(Preserve)
            .end()
    }
}
