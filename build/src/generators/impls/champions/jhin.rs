use super::*;

impl Generator for Jhin {
    fn generate(&mut self) -> MayFail {
        // 15% / 20% / 25% at levels 1 / 6 / 11
        let missing_health = stringify!(match ctx.level {
            ..6 => 0.15,
            6..11 => 0.2,
            11.. => 0.25,
        });

        self.ability(Key::P, [("Description 1 [1]", Void)])
            .modify(P(Void), |_| f![(missing_health) * EnemyMissingHealth])?
            .comment(P(Void), "Fourth shot bonus damage")?
            .ability(
                Key::Q,
                [
                    ("Physical Damage", Min),
                    ("Maximum Final Bounce Physical Damage", Max),
                ],
            )
            .ability(Key::W, [("Physical Damage", Void)])
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Minimum Physical Damage per Bullet", Min),
                    ("Maximum Physical Damage per Bullet", Max),
                    ("Minimum Fourth Shot Damage", _1Min),
                    ("Maximum Fourth Shot Damage", _1Max),
                ],
            )
            .combos([
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Ability(R(Max)),
                    Ability(R(Max)),
                    Ability(R(Max)),
                    Ability(R(Max)),
                ],
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[Ability(Q(Max)), Attack],
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[Attack, Attack, Attack, Ability(Q(Max)), Attack],
                &[
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
