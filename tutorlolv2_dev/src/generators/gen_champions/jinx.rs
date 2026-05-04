use super::*;

impl Generator for Jinx {
    fn generate(&mut self) -> MayFail {
        self.attr(AreaOnhitMax, [Q(Void)])?
            .attr(Area, [Q(Void), E(Void), R(_1Max), R(_1Min), R(Max), R(Min)])?
            .combo([
                Ability(E(Void)),
                Attack,
                Ability(R(Min)),
                Attack,
                Ability(W(Void)),
            ])?
            .progress(Stable)
            .end()
    }
}
