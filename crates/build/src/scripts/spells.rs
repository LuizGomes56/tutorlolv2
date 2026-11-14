//! Generates the public Ignite function from the parsed summoner spell data.

use crate::MayFail;
use quote::quote;
use wiki::spells::SpellData;

pub fn spells_code(data: &SpellData) -> MayFail<String> {
    data.validate()?;

    let damage = &data.ignite;

    Ok(quote! {
        pub(crate) const IGNITE_DAMAGE_BY_LEVEL: &[i32] = &[#(#damage),*];

        /// Total true damage from the wiki's summoner spell module.
        /// Levels beyond its table continue the final published increment.
        pub const fn ignite(level: core::num::NonZeroU8) -> i32 {
            let index = level.get() as usize - 1;
            let last = IGNITE_DAMAGE_BY_LEVEL.len() - 1;

            if index <= last {
                IGNITE_DAMAGE_BY_LEVEL[index]
            } else {
                let step = IGNITE_DAMAGE_BY_LEVEL[last] - IGNITE_DAMAGE_BY_LEVEL[last - 1];
                IGNITE_DAMAGE_BY_LEVEL[last] + step * (index - last) as i32
            }
        }
    }
    .to_string())
}
