use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

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
        $(
            #[allow(non_upper_case_globals)]
            pub const $Variant: $name = $name::$Variant;
        )+
    };
}

const_enum! {
    #[derive(Default, Copy, Serialize, Deserialize, Clone, Encode, Decode, PartialEq)]
    pub enum DamageType {
        Physical,
        Magic,
        Mixed,
        True,
        Adaptative,
        #[default]
        Unknown,
    }
}

const_enum! {
    #[derive(Copy, Clone, Encode, Serialize, Deserialize)]
    pub enum Attrs {
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

impl<T: AsRef<str>> From<T> for DamageType {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "PHYSICAL_DAMAGE" => DamageType::Physical,
            "MAGIC_DAMAGE" => DamageType::Magic,
            "MIXED_DAMAGE" => DamageType::Mixed,
            "TRUE_DAMAGE" => DamageType::True,
            "ADAPTATIVE_DAMAGE" => DamageType::Adaptative,
            _ => DamageType::Unknown,
        }
    }
}
