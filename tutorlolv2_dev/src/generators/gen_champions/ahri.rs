use super::*;

// #![stable]
// #![allow_missing_offsets]

impl Generator<Champion> for Ahri {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, Min)]);
        self.ability(Key::W, [(1, 0, Min), (1, 1, _1), (1, 2, Max)]);
        self.ability(Key::E, [(0, 1, Void)]);
        self.ability(Key::R, [(0, 0, Min)]);

        self.clone_to(Q(Min), Q(Max))?.damage =
            self.merge_damage(|[q]| format!("({q}) * {MagicMultiplier} + ({q})"), [Q(Min)])?;

        self.clone_to(R(Min), R(Max))?.damage =
            self.merge_damage(|[r]| format!("3 * ({r})"), [R(Min)])?;

        self.damage_type(Q(Min), Magic)?;
        self.damage_type(Q(Max), Mixed)?;

        self.attr(Area, [Q(Min), Q(Max)])?;
        self.end()
    }
}
