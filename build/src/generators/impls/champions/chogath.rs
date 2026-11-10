use super::*;

impl Generator for Chogath {
    fn generate(&mut self) -> MayFail {
        let feast_health = self.ranks(Key::R, "Bonus Health Per Stack")?;

        self.constant("FEAST_HEALTH_PER_STACK", feast_health)?
            .ability(Key::Q, [("Magic damage", Void)])
            .ability(Key::W, [("Magic damage", Void)])
            .ability(Key::E, [("Magic Damage", Min), ("Total Magic Damage", Max)])
            .ability(Key::R, [("Champion True Damage", Void)])
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[Ability(Q(Void)), Ability(W(Void)), Ability(R(Void))],
                &[
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(R(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
