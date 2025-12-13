/// Creates the `EvalIdent` and `EvalContext` structs, associating
/// the appropriate names and numeric types that it will hold. This struct
/// is essential to the application since it is used to evaluate all the
/// generated closures contained in cache static variables
macro_rules! create_eval_struct {
    ($($type:ident($($value:ident),*$(,)?)),+$(,)?) => {
        paste::paste! {
            /// Defines a standard type that implements trait [`core::fmt::Display`]
            /// and is used to create constant closures in the static variables of
            /// this module. For example:
            /// [`EvalIdent::QLevel`] is converted to: `ctx.q_level`
            #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
            #[repr(u8)]
            pub enum EvalIdent {
                $($([<$value:camel>],)*)*
            }

            $($(
                #[allow(non_upper_case_globals)]
                pub const [<$value:camel>]: EvalIdent = EvalIdent::[<$value:camel>];
            )*)*

            /// General struct that holds all the possible values that a constant
            /// closure can access when calculating the damage of some item, ability,
            /// passive, or rune. Those closures are created with the help of generators
            /// and the struct [`EvalIdent`].
            /// Closures have the following signature: `fn(ctx: &EvalContext) -> f32`.
            /// - The following code has an example of usage. For the complete details
            /// of the actual data that the static variable in the example holds, see
            /// [`crate::data::champions::NEEKO`]
            /// ```rs
            /// pub static NEEKO: CachedChampion {
            ///     .. // other fields
            ///     metadata: [
            ///         // example metadata
            ///         TypeMetadata<AbilityId> {
            ///             kind: AbilityId::Q(AbilityName::_1),
            ///             damage_type: DamageType::Magic,
            ///             attributes: Attrs::Undefined,
            ///         },
            ///         ..
            ///     ]
            ///     closures: [
            ///         // The first value in the `metadata` array is Q::_1,
            ///         // so this closure refers to that ability. Note that
            ///         // this is just an example
            ///         |ctx: &EvalContext| match ctx.q_level {
            ///             1 => 80f32 + 0.8f32 * ctx.ap,
            ///             2 => 160f32 + 0.8f32 * ctx.ap,
            ///             3 => 240f32 + 0.8f32 * ctx.ap,
            ///             ..
            ///         }
            ///     ]
            /// }
            /// ```
            #[derive(Default, Debug, Copy, Clone)]
            pub struct EvalContext {
                $($(pub $value: $type,)*)*
            }

            impl ::core::fmt::Display for EvalIdent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        $($(
                            Self::[<$value:camel>] => write!(f, concat!("ctx.", stringify!($value))),
                        )*)*
                    }
                }
            }
        }
    };
}

create_eval_struct!(
    u8(q_level, w_level, e_level, r_level),
    f32(
        level,
        chogath_stacks,
        veigar_stacks,
        nasus_stacks,
        smolder_stacks,
        aurelion_sol_stacks,
        thresh_stacks,
        kindred_stacks,
        belveth_stacks,
        adaptative_damage,
        physical_multiplier,
        magic_multiplier,
        steelcaps_effect,
        randuin_effect,
        rocksolid_effect,
        enemy_bonus_health,
        enemy_armor,
        enemy_max_health,
        enemy_health,
        enemy_current_health,
        enemy_missing_health,
        enemy_magic_resist,
        base_health,
        base_ad,
        base_armor,
        base_magic_resist,
        base_mana,
        bonus_ad,
        bonus_armor,
        bonus_magic_resist,
        bonus_health,
        bonus_mana,
        bonus_move_speed,
        armor_penetration_flat,
        armor_penetration_percent,
        magic_penetration_flat,
        magic_penetration_percent,
        max_mana,
        current_mana,
        max_health,
        current_health,
        armor,
        magic_resist,
        crit_chance,
        crit_damage,
        attack_speed,
        missing_health,
        ap,
        ad
    )
);
