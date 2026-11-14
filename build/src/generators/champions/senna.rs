use super::*;

impl Generator for Senna {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate - Relic Cannon", 0, P(_1))?
            .take("Innate - Weakened Soul", P(Void))?
            .take("Physical Damage", Q(Void))?
            .take("Physical Damage", W(Void))?
            .take("Physical Damage", R(Void))?
            .damage_type(P(Void), Physical)
            .damage_type(P(_1), Physical)
            .damage_type(R(Void), Physical)
            .combos([
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Attack,
                ],
                &[Ability(R(Void)), Ability(Q(Void))],
                &[Ability(W(Void)), Ability(Q(Void)), Ability(R(Void))],
                &[Ability(Q(Void)), Attack],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[Ability(W(Void)), Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
