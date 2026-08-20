use tutorlolv2::{AbilityId, AbilityName, ChampionId, ItemId, RuneId};
use tutorlolv2_rlib::render::{
    render_champion_function, render_champion_global, render_item_function, render_item_global,
    render_rune_global,
};

fn main() {
    tutorlolv2_rlib::packer::check_bin_packer().unwrap();
    let champion = render_champion_global(ChampionId::Neeko).unwrap();
    let cfn = render_champion_function(ChampionId::Neeko, AbilityId::Q(AbilityName::Max)).unwrap();
    let item = render_item_global(ItemId::BladeOfTheRuinedKing).unwrap();
    let ifn = render_item_function(ItemId::BladeOfTheRuinedKing).unwrap();
    let rune = render_rune_global(RuneId::Electrocute).unwrap();

    let html = champion + "<br>" + &cfn + "<br>" + &item + "<br>" + &ifn + "<br>" + &rune;

    std::fs::write("pk_render.html", html).unwrap()
}
