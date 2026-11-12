use super::*;

impl GeneratorV2 for Khazix {
    fn generate(&mut self) -> MayFail {
        self.take("Innate", P(Void))?
            .take("Physical Damage", Q(Min))?
            .take("Isolated Target Physical Damage", Q(Max))?
            .take("Physical Damage", W(Void))?
            .take("Physical Damage", E(Void))?
            .damage_type(W(Void), Physical)
            .damage_type(E(Void), Physical)
            .combos([
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(E(Void)),
                ],
                &[Attack, Ability(W(Void)), Ability(E(Void)), Ability(Q(Max))],
                &[Ability(E(Void)), Ability(Q(Max)), Ability(E(Void))],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[Ability(W(Void)), Ability(E(Void)), Ability(Q(Max)), Attack],
                &[Ability(E(Void)), Ability(Q(Max)), Attack, Ability(W(Void))],
                &[
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[Ability(W(Void)), Attack, Ability(Q(Max))],
                &[Ability(W(Void)), Ability(Q(Max)), Attack],
                &[Attack, Ability(W(Void)), Ability(Q(Max))],
            ])?
            .end()
    }
}
