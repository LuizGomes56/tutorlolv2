use super::*;

impl Generator for Amumu {
    fn generate(&mut self) -> MayFail {
        self.combo([
            Ability(Q(Void)),
            Ability(W(Void)),
            Ability(E(Void)),
            Ability(W(Void)),
            Ability(R(Void)),
            Attack,
            Ability(W(Void)),
            Attack,
            Ability(W(Void)),
            Ability(E(Void)),
        ])?
        .progress(Preserve)
        .end()
    }
}
