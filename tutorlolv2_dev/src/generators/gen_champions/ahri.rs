use super::*;

// #![stable]
// #![allow_missing_offsets]

impl Generator<Champion> for Ahri {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, Min)];
        ability![W, (1, 0, Min), (1, 1, _1), (1, 2, Max)];
        ability![E, (0, 1, Void)];
        ability![R, (0, 0, Min)];

        clone_to![Q::Min => Q::Max].damage = merge_damage!(
            |q| format!("({q}) * {mul} + ({q})", mul = EvalIdent::MagicMultiplier),
            Q::Min,
        );

        clone_to![R::Min => R::Max].damage = merge_damage!(|r| format!("3 * ({r})"), R::Min);

        damage_type![Q::Min, Magic];
        damage_type![Q::Max, Mixed];
        attr![Area => [Q::Min, Q::Max]];
    }
}
