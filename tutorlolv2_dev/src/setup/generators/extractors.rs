use regex::Regex;

/// Takes a string with the match "{number} : {number}" and returns the numeric values
/// Might return nothing if no values are found, or a tuple is malformed
pub fn extract_range_values(input: &str) -> Option<(f64, f64)> {
    let re = Regex::new(r"(\d+(?:\.\d+)?)(%)?\s*[:\-–]\s*(\d+(?:\.\d+)?)(%)?").ok()?;
    let caps = re.captures(input)?;

    let first = caps.get(1)?.as_str().parse::<f64>().ok()?;
    let second = caps.get(3)?.as_str().parse::<f64>().ok()?;

    let first_is_percent = caps.get(2).is_some();
    let second_is_percent = caps.get(4).is_some();

    let denom1 = if first_is_percent { 100.0 } else { 1.0 };
    let denom2 = if second_is_percent { 100.0 } else { 1.0 };

    Some((first / denom1, second / denom2))
}

/// Gets the tuples that are in pattern (+ Scalling) and formats the string to the internal format.
pub fn extract_scaled_values(input: &str) -> String {
    let re = Regex::new(r"\(([^)]+)\)").unwrap();
    let mut result = Vec::<String>::new();
    for cap in re.captures_iter(input) {
        let content = cap[1].trim();
        if content.to_lowercase().contains("based on level") {
            continue;
        }
        let cleaned = content.trim_start_matches('+').trim();
        let parts = cleaned.split_whitespace().collect::<Vec<&str>>();
        if parts.len() >= 2 && parts[0].ends_with('%') {
            if let Ok(percent) = parts[0].trim_end_matches('%').parse::<f64>() {
                let decimal = percent / 100.0;
                let rest = parts[1..].join(" ");
                result.push(format!("({} * {})", decimal, rest));
                continue;
            }
        }
        result.push(format!("({})", cleaned));
    }
    result
        .iter()
        .map(|value| replace_keys(value))
        .collect::<Vec<String>>()
        .join(" + ")
}

pub fn process_scaled_string(input: &str) -> String {
    let re = Regex::new(r"\([^\)]*\)").unwrap();
    let paren_part = re.find(input).map(|m| m.as_str()).unwrap_or("");
    let base = input.replace(paren_part, "").trim().to_string();
    let scaled = extract_scaled_values(paren_part);
    if !scaled.is_empty() {
        format!("{} + {}", base, scaled)
    } else {
        base
    }
}

pub fn remove_parenthesized_additions(input: &str) -> String {
    let re = Regex::new(r"\(\+\s*[^)]*\)").unwrap();
    re.replace_all(input, "").to_string()
}

/// Replaces common keys found in the API with the corresponding ones used internally
pub fn replace_keys(s: &str) -> String {
    let replacements = [
        ("per 100", "0.01 *"),
        ("of damage dealt", "100.0"),
        ("of damage stored", "100.0"),
        ("per Soul collected", " * THRESH_STACKS"),
        ("of expended Grit", "0.0"),
        ("of primary target's bonus health", "ENEMY_BONUS_HEALTH"),
        ("of his bonus health", "BONUS_HEALTH"),
        ("Pantheon's bonus health", "BONUS_HEALTH"),
        ("critical strike chance", "CRIT_CHANCE"),
        ("of the original damage", "100.0"),
        ("per Overwhelm stack on the target", "1.0"),
        ("of Ivern's AP", "AP"),
        ("of Sona's AP", "AP"),
        ("per Feast stack", "CHOGATH_STACKS"),
        ("of Siphoning Strike stacks", "NASUS_STACKS"),
        ("Stardust", "AURELION_SOL_STACKS"),
        ("per mark", "KINDRED_STACKS"),
        ("bonus armor", "BONUS_ARMOR"),
        ("bonus mana", "BONUS_MANA"),
        ("bonus AD", "BONUS_AD"),
        ("bonus armor", "BONUS_ARMOR"),
        ("bonus magic resistance", "BONUS_MAGIC_RESIST"),
        ("bonus health", "BONUS_HEALTH"),
        ("bonus movement speed", "BONUS_MOVE_SPEED"),
        ("armor", "ARMOR"),
        ("of the target's maximum health", "ENEMY_MAX_HEALTH"),
        ("of target's maximum health", "ENEMY_MAX_HEALTH"),
        ("of Zac's maximum health", "MAX_HEALTH"),
        ("of Braum's maximum health", "MAX_HEALTH"),
        ("of her maximum health", "MAX_HEALTH"),
        ("of his maximum health", "MAX_HEALTH"),
        ("of maximum health", "MAX_HEALTH"),
        ("maximum health", "MAX_HEALTH"),
        ("of the target's current health", "ENEMY_CURRENT_HEALTH"),
        ("of target's current health", "ENEMY_CURRENT_HEALTH"),
        ("target's current health", "ENEMY_CURRENT_HEALTH"),
        ("of the target's missing health", "ENEMY_MISSING_HEALTH"),
        ("of target's missing health", "ENEMY_MISSING_HEALTH"),
        ("target's missing health", "ENEMY_MISSING_HEALTH"),
        ("maximum mana", "MAX_MANA"),
    ];

    replacements
        .iter()
        .fold(s.to_string(), |acc, (old, new)| acc.replace(old, new))
}

pub fn extract_damagelike_expr(input: &str) -> String {
    let re = Regex::new(r"\{\{as\|([^\}]+)\}\}").unwrap();
    let mut results = Vec::<String>::new();
    for cap in re.captures_iter(input) {
        let mut content = cap[1].to_string();
        let nested = Regex::new(r"\{\{[^}]+\|([^}]+)\}\}").unwrap();
        content = nested.replace_all(&content, "$1").to_string();
        results.push(content);
    }
    process_scaled_string(&results.join(" ").replace("{{as|", ""))
}
