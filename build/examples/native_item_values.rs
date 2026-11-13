use tutorlolv2_build_dep::{
    MayFail,
    generators::parser::{
        Parser,
        items::{Item, ItemParser, Source},
    },
};
use tutorlolv2_types::AttackType;

fn main() -> MayFail {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();

    let parser = ItemParser::from_root(root)?;

    for id in std::env::args().skip(1) {
        let item = Item::try_from(parser.data[&id].clone())?;

        for source in [Source::Passive, Source::Passive2, Source::Active] {
            if item.description(source).is_err() {
                continue;
            }

            println!(
                "{id} {source:?}: {}",
                serde_json::to_string_pretty(&item.native_document(
                    source,
                    AttackType::Melee,
                    false
                )?)?
            );
        }
    }

    Ok(())
}
