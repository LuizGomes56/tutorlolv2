use super::*;

impl Generator for Kayn {
    fn generate(&mut self) -> MayFail {
        self.take("Physical Damage", Q(Min))?
            .map_change(Q(Min), "Bonus AD ratio per hit")?
            .take("Total Physical Damage", Q(Max))?
            .map_change(Q(Max), "Total bonus AD ratio")?
            .take("Physical Damage", W(Void))?
            .take("Physical Damage", R(Void))?
            .combos([
                &[Ability(W(Void)), Ability(Q(Max))],
                &[
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(R(Void)),
                    Ability(R(Void)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(R(Void)),
                    Ability(R(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(R(Void)),
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(R(Void)),
                    Ability(R(Void)),
                ],
                &[Ability(W(Void)), Ability(Q(Max)), Attack],
                &[Ability(Q(Max)), Attack, Ability(W(Void)), Attack],
            ])?
            .end()
    }
}
