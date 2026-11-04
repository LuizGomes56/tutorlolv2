use super::*;

impl Generator for Anivia {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Magic Damage", Min),
                ("Total Magic Damage", Max),
            ],
        )
        .ability(
            Key::E,
            [
                ("Enhanced Damage", Max),
                ("Magic Damage", Min),
            ],
        )
        .ability(
            Key::R,
            [
                ("Empowered Damage per Tick", Max),
                ("Magic Damage per Tick", Min),
            ],
        )
        .combos([
            &[
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(E(Min)),
                Ability(R(Max)),
                Attack,
                Attack,
                Ability(E(Min)),
                Attack,
                Ability(R(Max)),
            ],
            &[
                Ability(R(Max)),
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
                Ability(E(Max)),
                Attack,
                Ability(R(Max)),
            ],
            &[
                Ability(Q(Min)),
                Ability(R(Max)),
                Attack,
                Ability(Q(Min)),
                Ability(E(Max)),
                Attack,
                Ability(R(Max)),
            ],
            &[
                Ability(R(Max)),
                Attack,
                Ability(E(Max)),
                Ability(Q(Min)),
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(E(Max)),
                Ability(R(Max)),
            ],
            &[Ability(Q(Min)), Ability(E(Max)), Ability(Q(Min))],
            &[
                Ability(R(Max)),
                Attack,
                Ability(E(Max)),
                Ability(R(Max)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
                Ability(E(Max)),
                Attack,
            ],
        ])?
        .end()
    }
}
