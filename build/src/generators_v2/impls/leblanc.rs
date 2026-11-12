use super::*;

impl GeneratorV2 for Leblanc {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Min))?
            .take("Total Magic Damage", Q(Max))?
            .take("Magic Damage", W(Void))?
            .take("Fracture Magic Damage", E(_1))?
            .take("Magic Damage", E(Min))?
            .take("Total Damage", E(Max))?
            .take("Application Magic Damage", R(_1))?
            .take("Mark Magic Damage", R(_2))?
            .take("Magic Damage", R(_3))?
            .take("Orb Magic Damage", R(_4))?
            .take("Fracture Magic Damage", R(_5))?
            .take_nth("Total Magic Damage", 1, R(_6))?
            .combos([
                &[
                    Ability(R(_1)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(R(_1)),
                    Ability(E(Max)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Ability(R(_1)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(R(_1)),
                    Ability(W(Void)),
                    Ability(R(_1)),
                    Ability(Q(Max)),
                    Ability(E(Max)),
                ],
                &[
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Attack,
                    Ability(R(_1)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(R(_1)),
                    Ability(Q(Max)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                ],
                &[
                    Ability(E(Max)),
                    Ability(Q(Max)),
                    Ability(R(_1)),
                    Ability(W(Void)),
                    Ability(W(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(R(_1)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Ability(R(_1)),
                    Ability(Q(Max)),
                ],
                &[
                    Ability(R(_1)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Ability(R(_1)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(R(_1)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                ],
                &[Ability(W(Void)), Attack, Ability(Q(Max)), Ability(W(Void))],
                &[Ability(Q(Max)), Ability(W(Void)), Attack, Ability(W(Void))],
            ])?
            .end()
    }
}
