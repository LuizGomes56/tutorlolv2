use super::*;

impl Generator for Akali {
    fn generate(&mut self) -> MayFail {
        self.attr(Area, [Q(Void), R(_1), R(_2Min), R(_2Max)])?
            .attr(Onhit, [P(Void)])?
            .combo([
                Ability(Q(Void)),
                Ability(E(_1Min)),
                Attack,
                Ability(P(Void)),
                Ability(E(_1Max)),
                Ability(R(_1)),
            ])?
            .progress(Stable)
            .end()
    }
}
