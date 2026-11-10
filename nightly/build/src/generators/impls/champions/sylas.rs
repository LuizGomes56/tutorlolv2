use super::*;

impl Generator for Sylas {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Unshackled", Min)])
            .modify(P(Min), |_| f![0.4 * AttackDamage + 0.2 * AbilityPower])?
            .comment(P(Min), "Secondary Target Damage")?
            .clone_to(P(Min), P(Max), f![1.3 * AttackDamage + 0.3 * AbilityPower])?
            .comment(P(Max), "Primary Target Damage")?
            .ability(
                Key::Q,
                [
                    ("Magic Damage", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .ability(Key::W, [("Magic Damage", Void)])
            .ability(Key::E, [("Magic Damage", Void)])
            .combos([
                &[Ability(Q(Max)), Ability(W(Void)), Attack, Ability(E(Void))],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(E(Void)), Ability(E(Void)), Ability(W(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
