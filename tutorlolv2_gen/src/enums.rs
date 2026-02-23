use core::{convert::Infallible, str::FromStr};

/// Creates an enum and associates constants that represents each of its
/// variants, using the same name and ignores `upper_case` lints
macro_rules! const_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $(
                $(#[$vmeta:meta])*
                $Variant:ident,
            )+
        }
    ) => {
        $(#[$meta])*
        pub enum $name {
            $(
                $(#[$vmeta])*
                $Variant,
            )+
        }
    };
}

const_enum! {
    /// Defines what is the damage type of some entity.
    /// - [`DamageType::Physical`] and [`DamageType::Magic`] Represents any damage related
    /// to how much (armor or magic resistence) the enemy player has, and is affected by the
    /// percent and flat values or (armor or magic) penetration of the current player
    /// - [`DamageType::Mixed`] Damages of this type are treated as a special case of
    /// [`DamageType::True`], where the closure has to multiply on its own the `physical_mod`
    /// and `magic_mod` modifiers of the [`tutorlolv2_math::DamageModifiers`] struct. It is
    /// usually used when a single ability or item deals both physical and magic damage in the
    /// same hit.
    /// - [`DamageType::True`] Damages of this type are not affected by armor or magic resistence,
    /// their values are in general irreducible.
    /// - [`DamageType::Adaptive`] Damages of this type will vary into the [`DamageType::Physical`]
    /// or [`DamageType::Magic`] depending on how much bonus armor or ability power the current player
    /// has.
    /// - [`DamageType::Unknown`] is the default value when no damage type is set
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[derive(bincode::Encode, bincode::Decode)]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum DamageType {
        Physical,
        Magic,
        Mixed,
        True,
        Adaptive,
        #[default]
        Unknown,
    }
}

const_enum! {
    /// An enum with several variants that can be used to add up to `255` attributes
    /// to some ability, item or rune. It is mostly used to determine if the current
    /// instance damages onhit only for the `maximum`, `minimum` or both damage kinds.
    /// [`Attrs::Undefined`] is set to be the default variant, representing no extra data. This
    /// is also used to determine if some ability has area damage
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[derive(bincode::Encode, bincode::Decode)]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum Attrs {
        #[default]
        Undefined,
        Onhit,
        OnhitMin,
        OnhitMax,
        Area,
        AreaOnhit,
        AreaOnhitMin,
        AreaOnhitMax,
    }
}

impl FromStr for DamageType {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PHYSICAL_DAMAGE" => Ok(DamageType::Physical),
            "MAGIC_DAMAGE" => Ok(DamageType::Magic),
            "MIXED_DAMAGE" => Ok(DamageType::Mixed),
            "TRUE_DAMAGE" => Ok(DamageType::True),
            "ADAPTIVE_DAMAGE" => Ok(DamageType::Adaptive),
            _ => Ok(DamageType::Unknown),
        }
    }
}
