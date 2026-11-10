use super::*;

impl Generator for Brand {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                ("Ablaze", Void),
                ("Description 2", _1),
            ],
        )
        // The values based on stacks are the damage cap against monsters
        .modify(P(Void), |_| f![0.02 * EnemyMaxHealth])?
        .ability(Key::Q, [("Magic Damage", Void)])
        .ability(
            Key::W,
            [
                ("Increased Damage", Max),
                ("Magic Damage", Min),
            ],
        )
        .ability(Key::E, [("Magic Damage", Void)])
        .ability(
            Key::R,
            [
                ("Magic Damage", Min),
                ("Total Single-Target Damage", Max),
            ],
        )
        .combos([
            &[Ability(Q(Void)), Ability(E(Void)), Ability(W(Max))],
            &[
                Ability(R(Max)),
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(W(Max)),
                Attack,
            ],
            &[
                Ability(E(Void)),
                Ability(R(Max)),
                Ability(Q(Void)),
                Ability(W(Max)),
            ],
            &[
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(W(Max)),
                Ability(R(Max)),
                Attack,
                Attack,
            ],
            &[
                Ability(W(Max)),
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(R(Max)),
                Attack,
            ],
            &[
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(W(Max)),
                Ability(R(Max)),
                Attack,
            ],
            &[
                Ability(E(Void)),
                Ability(W(Max)),
                Ability(R(Max)),
                Ability(Q(Void)),
                Attack,
            ],
            &[Ability(E(Void)), Ability(Q(Void)), Ability(W(Max))],
            &[Ability(W(Max)), Ability(E(Void)), Ability(Q(Void))],
        ])?
        .end()
    }
}
