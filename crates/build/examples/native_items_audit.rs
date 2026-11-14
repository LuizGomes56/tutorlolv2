use build::{
    MayFail,
    generators::parser::{Parser, items::ItemParser},
};

fn main() -> MayFail {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap();

    let parser = ItemParser::from_root(root)?;
    let mut failures = Vec::new();

    for id in parser.data.keys() {
        if build::generators::items::item_gen_fn(id).is_some() {
            match parser.run_fn(id) {
                Ok(_) => println!("OK {id}"),
                Err(e) => {
                    println!("FAIL {id}: {e}");
                    failures.push(format!("{id}: {e}"));
                }
            }
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("\n").into())
    }
}
