use super::*;

impl GeneratorV2 for Illaoi {
    fn generate(&mut self) -> MayFail {
        let increase = self.value("Damage Increase", Key::Q)?;

        self.take_nth("Innate", 0, P(Void))?
            .comment(P(Void), "Tentacle slam")
            .derive(Q(Void), P(Void), |p| f![p * (1 + increase / 100)])?
            .comment(Q(Void), "Tentacle slam increased by Tentacle Smash's rank")
            .take("Additional Physical Damage", W(Max))?
            .take("Minimum Physical Damage", W(Min))?
            .take("Physical Damage", R(Void))?
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(W(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Min)),
                    Attack,
                    Ability(W(Min)),
                ],
                &[
                    Attack,
                    Ability(W(Min)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(W(Min)),
                    Attack,
                    Ability(W(Min)),
                    Attack,
                    Ability(W(Min)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(W(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Min)),
                ],
                &[
                    Ability(W(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Ability(W(Min)),
                ],
                &[Attack, Ability(W(Max)), Ability(Q(Void)), Attack, Attack],
                &[Ability(W(Max)), Attack, Ability(Q(Void)), Attack],
                &[Attack, Ability(W(Max)), Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
