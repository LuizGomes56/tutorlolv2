use highlight_ir::{Decoder, Dictionary, Encoder, TemplateRegistry};

#[test]
fn round_trip_preserves_rendering() {
    let source = "impl Generator for Struct {\n    fn generate(&mut self) -> MayFail {\n        let value = zero();\n    }\n}\n";
    let dictionary = Dictionary::new();
    let templates = TemplateRegistry::new();

    let (ir, aux) = Encoder::new(source, &dictionary, &templates).encode();
    let html = Decoder::new(&ir, &aux, &dictionary, &templates).render();

    assert!(html.contains(r#"<span class="Type">Struct</span>"#));
    assert!(html.contains(r#"<span class="Function">generate</span>"#));
    assert!(html.contains(r#"<span class="Keyword">let</span>"#));
    assert!(html.contains(r#"<span class="Function">zero</span>"#));
}

#[test]
fn unknown_identifier_falls_back_to_literal_text() {
    let source = "some_unknown_fn(x)";
    let dictionary = Dictionary::new();
    let templates = TemplateRegistry::new();

    let (ir, aux) = Encoder::new(source, &dictionary, &templates).encode();
    let html = Decoder::new(&ir, &aux, &dictionary, &templates).render();

    assert!(html.contains(r#"<span class="Function">some_unknown_fn</span>"#));
    // The unknown name must round-trip through the aux buffer, not vanish.
    assert!(aux.windows(15).any(|w| w == b"some_unknown_fn"));
}

#[test]
fn static_item_template_collapses_repeated_name() {
    let source = "static ITEM_SWORD: Item = Item {\n    name: \"Sword\",\n};\n";
    let dictionary = Dictionary::new();
    let templates = TemplateRegistry::new();

    let (ir, aux) = Encoder::new(source, &dictionary, &templates).encode();
    let html = Decoder::new(&ir, &aux, &dictionary, &templates).render();

    assert!(html.contains(r#"<span class="Constant">ITEM_SWORD</span>"#));
    assert!(html.contains(r#"<span class="String">"Sword"</span>"#));
    assert!(html.contains(r#"<span class="Type">Item</span>"#));
    // Neither "ITEM_SWORD" nor "Sword" should have been spelled out in the
    // aux buffer — both are reconstructed from the same 2-byte discriminant.
    assert!(!aux.windows(10).any(|w| w == b"ITEM_SWORD"));
    assert!(!aux.windows(5).any(|w| w == b"Sword"));
}

#[test]
fn mismatched_item_static_falls_back_instead_of_applying_template() {
    // The quoted name doesn't match the item id's real name, so the
    // template must not apply — this should still round-trip correctly
    // via the generic tokenizer, just without the compaction.
    let source = "static ITEM_SWORD: Item = Item {\n    name: \"NotASword\",\n};\n";
    let dictionary = Dictionary::new();
    let templates = TemplateRegistry::new();

    let (ir, aux) = Encoder::new(source, &dictionary, &templates).encode();
    let html = Decoder::new(&ir, &aux, &dictionary, &templates).render();

    assert!(html.contains("ITEM_SWORD"));
    assert!(html.contains("NotASword"));
}

#[test]
fn rainbow_brackets_track_nesting_depth() {
    let source = "(a(b)c)";
    let dictionary = Dictionary::new();
    let templates = TemplateRegistry::new();

    let (ir, aux) = Encoder::new(source, &dictionary, &templates).encode();
    let html = Decoder::new(&ir, &aux, &dictionary, &templates).render();

    assert!(html.contains(r#"<span class="Bracket1">(</span>"#));
    assert!(html.contains(r#"<span class="Bracket2">(</span>"#));
    assert!(html.contains(r#"<span class="Bracket2">)</span>"#));
    assert!(html.contains(r#"<span class="Bracket1">)</span>"#));
}
