use super::*;

impl Generator for Akshan {
    fn generate(&mut self) -> MayFail {
        // 15 / 40 / 80 / 150 at levels 1 / 6 / 11 / 16
        let dirty_fighting = stringify!(match ctx.level {
            ..6 => 15,
            6..11 => 40,
            11..16 => 80,
            16.. => 150,
        });

        self.ability(Key::P, [("Innate", Void)])
            .modify(P(Void), |_| f![(dirty_fighting) + 0.6 * AbilityPower])?
            .damage_type(P(Void), Magic)?
            .comment(P(Void), "Dirty Fighting third stack bonus magic damage")?
            .ability(
                Key::Q,
                [
                    ("Physical Damage", Min),
                    ("Total Physical Damage", Max),
                ],
            )
            .ability(Key::E, [("Physical Damage per Shot", Void)])
            .ability(
                Key::R,
                [
                    ("Damage to target on 67% missing hp", _1Max),
                    ("Maximum Physical Damage per Bullet", Max),
                    ("Minimum Charged Physical Damage", _1Min),
                    ("Minimum Physical Damage per Bullet", Min),
                ],
            )
            .combos([
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Ability(R(Max)),
                ],
                &[Attack, Attack, Ability(Q(Max))],
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(R(Max)),
                    Ability(E(Void)),
                ],
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                ],
            ])?
            .end()
    }
}
