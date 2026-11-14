use super::*;

impl Generator for Tristana {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .take("Minimum Physical Damage", E(Min))?
            .take("Full Stack Physical Damage", E(Max))?
            .take("Magic Damage", R(Void))?
            .comment(E(Void), "Passive explosion damage on kill")
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[Attack, Ability(E(Void)), Attack, Attack, Attack, Attack],
            ])?
            .end()
    }
}
