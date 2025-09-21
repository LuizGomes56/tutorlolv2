use std::{
    io::Write,
    process::{Command, Stdio},
};
use synoptic::{Highlighter, TokOpt};

pub fn invoke_rustfmt(src: &str, width: usize) -> String {
    let mut child = Command::new("rustfmt")
        .args(&[
            "--emit",
            "stdout",
            "--config",
            &format!("max_width={width}"),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(src.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    String::from_utf8_lossy(&output.stdout).into_owned()
}

pub fn highlight(code_string: &str) -> String {
    let mut h = Highlighter::new(4);
    h.bounded("comment", r"/\*", r"\*/", false);
    h.keyword("comment", r"//.*$");
    h.bounded_interp("string", "\"", "\"", "\\{", "\\}", true);
    h.keyword("lifetime", r"'\w+");
    h.keyword(
        "keyword",
        r"\b(pub|use|crate|mut|static|ref|dyn|unsafe|extern|type|super|mod|struct|const|enum|fn|let|impl|trait|where|loop|Self|self)\b",
    );
    h.keyword(
        "control",
        r"\b(break|continue|intrinsic|match|return|yield|for|while|match|if|else|as|in)\b",
    );

    h.keyword("constant", r"\b[A-Z]+\b");
    h.keyword("constant", r"\b(AbilityHaste|AbilityPower|Armor|ArmorPenetration|MagicPenetration|AttackDamage|AttackSpeed|GoldPer10Seconds|AdaptiveForce|CriticalStrikeChance|CriticalStrikeDamage|Health|LifeSteal|MagicResist|Mana|MoveSpeed|Omnivamp|BaseHealthRegen|BaseManaRegen|Tenacity|HealAndShieldPower|_1|_2|_3|_4|_5|_6|_7|_8|Mega|Max|Min|Minion|Minion1|Minion2|Minion3|MinionMax|Monster|Monster1|Monster2|Monster3|Monster4|MonsterMax|Void|_1Max|_2Max|_3Max|_4Max|_5Max|_6Max|_7Max|_8Max|_1Min|_2Min|_3Min|_4Min|_5Min|_6Min|_7Min|_8Min|Some|None|Top|Jungle|Middle|Bottom|Support|Melee|Ranged|BASIC_ATTACK|CRITICAL_STRIKE|Physical|Magic|Mixed|True|Adaptative|Unknown|Onhit|OnhitMin|OnhitMax)\b");
    h.keyword("type", r"\b[A-Z][a-zA-Z0-9_]*\b");
    h.keyword(
        "primitive",
        r"\b(bool|usize|u8|u16|u32|u64|isize|i8|i16|i32|i64|f32|f64|char|str|tutorlolv2_macros)\b",
    );
    h.keyword("float", r"\b\d+\.?\d*f32\b");
    h.keyword("number", r"\b\d+\.?\d*\b");
    h.keyword("boolean", r"\b(true|false)\b");
    h.keyword("macro", r"[a-zA-Z_][a-zA-Z0-9_]*!");
    h.keyword("function", r"\b[a-z][a-zA-Z0-9_]*\(");
    h.keyword("function", r"\b(zero|generator)\b");
    h.keyword("variable", r"\b[a-z][a-zA-Z0-9_]*\b");

    let code = code_string
        .lines()
        .map(str::to_string)
        .collect::<Vec<String>>();

    h.run(&code);

    let mut out = String::new();
    for (i, line) in code.iter().enumerate() {
        let mut line_html = String::new();
        for token in h.line(i, line) {
            match token {
                TokOpt::Some(text, kind) => {
                    line_html.push_str(&format!("<span class=\"{kind}\">{text}</span>"));
                }
                TokOpt::None(text) => {
                    line_html.push_str(&text);
                }
            }
        }
        out.push_str(&line_html);
        out.push_str("\n");
    }
    format!("<pre>{}</pre>", out)
}
