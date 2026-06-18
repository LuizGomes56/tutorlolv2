use {
    crate::{ChampionId, ItemId, RuneId},
    core::ops::Range,
    tutorlolv2_types::CtxVar,
};

include!(concat!(env!("OUT_DIR"), "/docs.rs"));

pub const DOCS: &str = include_str!(concat!(env!("OUT_DIR"), "/docs.txt"));
pub const DOCS_LEN: usize = DOCS.len();

pub const DOCS_BR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/docs.br"));
pub const DOCS_BR_LEN: usize = DOCS_BR.len();

impl ChampionId {
    pub const FUNCTIONS_DOCS: &[&[Range<usize>]; Self::VARIANTS] = &CHAMPION_CLOSURES;
    pub const ABILITIES_DOCS: &[&[Range<usize>]; Self::VARIANTS] = &ABILITY_FORMULAS;

    pub const fn functions_docs(&self) -> &'static [Range<usize>] {
        &Self::FUNCTIONS_DOCS[self.index()]
    }

    pub const fn abilities_docs(&self) -> &'static [Range<usize>] {
        &Self::ABILITIES_DOCS[self.index()]
    }

    pub const fn identifiers(&self) -> &'static [&'static [CtxVar]] {
        self.data().identifiers
    }
}

impl ItemId {
    pub const FUNCTIONS_DOCS: &[[[Range<usize>; 2]; 2]; Self::VARIANTS] = &ITEM_CLOSURES;

    pub const fn functions_docs(&self) -> &'static [[Range<usize>; 2]; 2] {
        &Self::FUNCTIONS_DOCS[self.index()]
    }

    pub const fn identifiers(&self) -> &'static [CtxVar] {
        self.data().identifiers
    }
}

impl RuneId {
    pub const FUNCTIONS_DOCS: &[[[Range<usize>; 2]; 2]; Self::VARIANTS] = &RUNE_CLOSURES;

    pub const fn functions_docs(&self) -> &'static [[Range<usize>; 2]; 2] {
        &Self::FUNCTIONS_DOCS[self.index()]
    }

    pub const fn identifiers(&self) -> &'static [CtxVar] {
        self.data().identifiers
    }
}
