use super::*;

impl Generator for Elise {
    fn generate(&mut self) -> MayFail {
        // Spider Queen's innates were removed from the game
        self.take_in("Venomous Bite", "Magic Damage", Q(_1))?
            .take_in("Neurotoxin", "Magic Damage", Q(_2))?
            .take("Magic Damage", W(Void))?
            .insert_pet(R(Void), "Spiderling")?
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(_1)),
                    Ability(R(Void)),
                    Ability(Q(_1)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(_1)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(_1)),
                    Ability(R(Void)),
                    Ability(Q(_1)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(_1)),
                    Ability(R(Void)),
                    Ability(Q(_1)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(_1)),
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Ability(Q(_1)),
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(_1)),
                    Attack,
                    Ability(R(Void)),
                    Ability(Q(_1)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[Ability(R(Void)), Ability(W(Void)), Ability(Q(_1)), Attack],
            ])?
            .end()
    }
}
