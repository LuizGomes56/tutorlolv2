use crate::const_bytes;
use actix_web::{HttpResponse, Responder, get};

static FORMULAS_CHAMPIONS_BYTES: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/champion_formulas.br"));
static FORMULAS_ITEMS_BYTES: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/item_formulas.br"));
static FORMULAS_RUNES_BYTES: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/rune_formulas.br"));
static FORMULAS_ABILITIES_BYTES: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/ability_formulas.br"));
static FORMULAS_CHAMPION_GENERATOR_BYTES: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/champion_generator.br"));

#[get("/champions")]
pub async fn formulas_champions() -> impl Responder {
    const_bytes!(FORMULAS_CHAMPIONS_BYTES)
}

#[get("/champion_generator")]
pub async fn formulas_champion_generator() -> impl Responder {
    const_bytes!(FORMULAS_CHAMPION_GENERATOR_BYTES)
}

#[get("/abilities")]
pub async fn formulas_abilities() -> impl Responder {
    const_bytes!(FORMULAS_ABILITIES_BYTES)
}

#[get("/items")]
pub async fn formulas_items() -> impl Responder {
    const_bytes!(FORMULAS_ITEMS_BYTES)
}

#[get("/runes")]
pub async fn formulas_runes() -> impl Responder {
    const_bytes!(FORMULAS_RUNES_BYTES)
}
