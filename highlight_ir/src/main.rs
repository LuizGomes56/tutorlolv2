use highlight_ir::{Decoder, Dictionary, Encoder, TemplateRegistry};

fn main() {
    let source = concat!(
        "impl Generator for Struct {\n",
        "    fn generate(&mut self) -> MayFail {\n",
        "        let value = zero();\n",
        "        static ITEM_SWORD: Item = Item {\n",
        "            name: \"Sword\",\n",
        "        };\n",
        "        return value;\n",
        "    }\n",
        "}\n",
    );

    let dictionary = Dictionary::new();
    let templates = TemplateRegistry::new();

    let (ir, aux) = Encoder::new(source, &dictionary, &templates).encode();
    let html = Decoder::new(&ir, &aux, &dictionary, &templates).render();

    println!("source bytes: {}", source.len());
    println!(".ir bytes:    {}", ir.len());
    println!(".txt bytes:   {}", aux.len());
    println!(
        "combined:     {} ({:.0}% of source)",
        ir.len() + aux.len(),
        (ir.len() + aux.len()) as f64 / source.len() as f64 * 100.0
    );
    println!("\n--- rendered HTML ---\n{html}");
}
