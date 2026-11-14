use super::*;

impl Generator for Fizz {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Active Magic Damage", W(Void))?
            .take("Active On-Hit Magic Damage", W(_1))?
            .take("Total Passive Magic Damage", W(_2))?
            .take_in("Trickster", "Magic Damage", E(Void))?
            .take("Chomper Damage", R(_1))?
            .take("Guppy Damage", R(Min))?
            .take("Gigalodon Damage", R(Max))?
            .damage_type(Q(Void), Magic)
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Attack, Ability(W(Void)), Ability(Q(Void))],
                &[Ability(Q(Void)), Ability(R(Max))],
                &[Ability(E(Void)), Ability(W(Void)), Ability(Q(Void))],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                ],
                &[
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                ],
            ])?
            .end()
    }
}
