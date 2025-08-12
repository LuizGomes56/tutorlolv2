use super::*;

const ONHIT_EFFECT: &'static str = r#"<pre><span class="control">intrinsic</span> <span class="constant">ONHIT_EFFECT</span> = {
    <span class="variable">name</span>: <span class="string">"Onhit Effect"</span>,
    <span class="variable">damage_type</span>: <span class="type">DamageType</span>::<span class="constant">Mixed</span>,
    <span class="variable">damage</span>: ($($<span class="variable">dmg</span>:<span class="variable">expr</span>),*) => {
        <span class="variable">onhit_effects</span>.<span class="variable">minimum_damage</span>
            += <span class="constant">BASIC_ATTACK</span> + $($<span class="variable">dmg</span>),*;
        <span class="variable">onhit_effects</span>.<span class="variable">maximum_damage</span>
            += <span class="constant">CRITICAL_STRIKE</span> + $($<span class="variable">dmg</span>),*;
    },
};
</pre>"#;

pub fn export_code(_: &str) {
    let mut mega_block = String::with_capacity(1 << 24);
    export_champions(&mut mega_block);
    export_items(&mut mega_block);
    export_runes(&mut mega_block);
    let mut bytes = Vec::new();
    for file in ["champions", "items", "runes"] {
        let content = fs::read(format!("comptime_exports/{}.txt", file)).unwrap();
        bytes.extend_from_slice(&content);
    }

    let mut current_offset = mega_block.len();

    let basic_attack_start = current_offset;
    let highlighted_basic_attack = highlight(&invoke_rustfmt(&BASIC_ATTACK, 60));
    mega_block.push_str(&highlighted_basic_attack);
    let basic_attack_end = current_offset + highlighted_basic_attack.len();
    let basic_attack_offset = (basic_attack_start, basic_attack_end);
    current_offset = basic_attack_end;

    let critical_strike_start = current_offset;
    let highlighted_critical_strike = highlight(&invoke_rustfmt(&CRITICAL_STRIKE, 60));
    mega_block.push_str(&highlighted_critical_strike);
    let critical_strike_end = current_offset + highlighted_critical_strike.len();
    let critical_strike_offset = (critical_strike_start, critical_strike_end);
    current_offset = critical_strike_end;

    let onhit_effect_start = current_offset;
    mega_block.push_str(ONHIT_EFFECT);
    let onhit_effect_end = current_offset + ONHIT_EFFECT.len();
    let onhit_effect_offset = (onhit_effect_start, onhit_effect_end);
    current_offset = onhit_effect_end;

    let mega_block_len = current_offset;

    let mega_block_compressed = compress_bytes!(mega_block.as_bytes());
    bytes.extend_from_slice(
        format!(
            "
            pub static BASIC_ATTACK_OFFSET: (usize, usize) = {};
            pub static CRITICAL_STRIKE_OFFSET: (usize, usize) = {};
            pub static ONHIT_EFFECT_OFFSET: (usize, usize) = {};
            pub static UNCOMPRESSED_MEGA_BLOCK_SIZE: usize = {}; 
            pub static MEGA_BLOCK: [u8; {}] = [{}];",
            format!("({}, {})", basic_attack_offset.0, basic_attack_offset.1),
            format!(
                "({}, {})",
                critical_strike_offset.0, critical_strike_offset.1
            ),
            format!("({}, {})", onhit_effect_offset.0, onhit_effect_offset.1),
            mega_block_len,
            mega_block_compressed.len(),
            mega_block_compressed.join(",")
        )
        .as_bytes(),
    );
    let _ = fs::write("comptime_exports/export_code.txt", bytes);
}
