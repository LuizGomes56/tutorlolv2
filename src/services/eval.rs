use crate::model::{
    base::{AdaptativeType, Stats},
    champions::ChampionCdnStats,
};

pub trait ConditionalAddition {
    fn add_if_some(&mut self, value: Option<f64>);
}

impl ConditionalAddition for f64 {
    fn add_if_some(&mut self, value: Option<Self>) {
        if let Some(v) = value {
            *self += v;
        }
    }
}

pub struct RiotFormulas;

impl RiotFormulas {
    /// Uses wiki's formula to return base stats for a given champion
    pub fn stat_growth(base: f64, growth_per_level: f64, level: usize) -> f64 {
        base + growth_per_level * (level as f64 - 1.0) * (0.7025 + 0.0175 * (level as f64 - 1.0))
    }
    /// Percentage values are entered in this section as a number in range 0-100
    ///
    /// 30% and 30% penetration should yield 49% penetration (0.51 true value)
    ///
    /// ```
    /// for x in vec yield x / 10.pow(len(x) << 1)
    /// let from_vec = [30, 30];
    /// return 0.51
    ///
    /// ```
    pub fn percent_value(from_vec: Vec<f64>) -> f64 {
        from_vec
            .iter()
            .map(|value: &f64| 100.0 - value)
            .product::<f64>()
            / 10f64.powi((from_vec.len() << 1) as i32)
    }

    pub fn adaptative_type(attack_damage: f64, ability_power: f64) -> AdaptativeType {
        if 0.35 * attack_damage >= 0.2 * ability_power {
            AdaptativeType::Physical
        } else {
            AdaptativeType::Magic
        }
    }

    pub fn full_base_stats(cdn: &ChampionCdnStats, level: usize) -> Stats {
        Stats {
            ability_power: 0.0,
            armor: Self::stat_growth(cdn.armor.flat, cdn.armor.per_level, level),
            armor_penetration_flat: 0.0,
            armor_penetration_percent: 0.0,
            attack_damage: Self::stat_growth(
                cdn.attack_damage.flat,
                cdn.attack_damage.per_level,
                level,
            ),
            attack_range: Self::stat_growth(
                cdn.attack_range.flat,
                cdn.attack_range.per_level,
                level,
            ),
            attack_speed: Self::stat_growth(
                cdn.attack_speed.flat,
                cdn.attack_speed.per_level,
                level,
            ),
            crit_chance: 0.0,
            crit_damage: Self::stat_growth(
                cdn.critical_strike_damage.flat,
                cdn.critical_strike_damage.per_level,
                level,
            ) * cdn.critical_strike_damage_modifier.flat,
            current_health: Self::stat_growth(cdn.health.flat, cdn.health.per_level, level),
            max_health: Self::stat_growth(cdn.health.flat, cdn.health.per_level, level),
            current_mana: Self::stat_growth(cdn.mana.flat, cdn.mana.per_level, level),
            max_mana: Self::stat_growth(cdn.mana.flat, cdn.mana.per_level, level),
            magic_penetration_flat: 0.0,
            magic_penetration_percent: 0.0,
            magic_resist: Self::stat_growth(
                cdn.magic_resistance.flat,
                cdn.magic_resistance.per_level,
                level,
            ),
        }
    }
}

pub trait MathEval {
    fn eval(&self) -> Result<f64, ()>;
}

impl<T: AsRef<str>> MathEval for T {
    fn eval(&self) -> Result<f64, ()> {
        eval_math_expr(self.as_ref())
    }
}

#[derive(Clone)]
enum Token {
    Number(f64),
    Operator(char),
    LParen,
    RParen,
}

/// Takes a string and evaluates it mathematically.
/// None values mean NaN as in JavaScript
/// Only parenthesis may be used. [] and {} are not supported
/// Exponentiation is done with the ^ operator, not with the ** operator
fn eval_math_expr(expr: &str) -> Result<f64, ()> {
    let tokens: Vec<Token> = tokenize(expr)?;
    let rpn: Vec<Token> = shunting_yard(&tokens);
    evaluate_rpn(&rpn)
}

fn tokenize(expr: &str) -> Result<Vec<Token>, ()> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut num_buf: String = String::new();
    let mut chars = expr.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' | '.' => {
                num_buf.push(ch);
                chars.next();
            }
            '+' | '-' | '*' | '/' | '^' => {
                if !num_buf.is_empty() {
                    tokens.push(Token::Number(num_buf.parse().map_err(|_| ())?));
                    num_buf.clear();
                }
                tokens.push(Token::Operator(ch));
                chars.next();
            }
            '(' => {
                if !num_buf.is_empty() {
                    tokens.push(Token::Number(num_buf.parse().map_err(|_| ())?));
                    num_buf.clear();
                }
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                if !num_buf.is_empty() {
                    tokens.push(Token::Number(num_buf.parse().map_err(|_| ())?));
                    num_buf.clear();
                }
                tokens.push(Token::RParen);
                chars.next();
            }
            ' ' => {
                chars.next();
            }
            _ => {
                return Err(());
            }
        }
    }

    if !num_buf.is_empty() {
        tokens.push(Token::Number(num_buf.parse().map_err(|_| ())?));
    }

    Ok(tokens)
}

fn precedence(op: char) -> u8 {
    match op {
        '^' => 4,
        '*' | '/' => 3,
        '+' | '-' => 2,
        _ => 0,
    }
}

fn is_right_associative(op: char) -> bool {
    op == '^'
}

fn shunting_yard(tokens: &[Token]) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut ops: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token.clone()),
            Token::Operator(op1) => {
                while let Some(Token::Operator(op2)) = ops.last() {
                    if (precedence(*op1) < precedence(*op2))
                        || (precedence(*op1) == precedence(*op2) && !is_right_associative(*op1))
                    {
                        output.push(ops.pop().unwrap());
                    } else {
                        break;
                    }
                }
                ops.push(token.clone());
            }
            Token::LParen => ops.push(token.clone()),
            Token::RParen => {
                while let Some(top) = ops.pop() {
                    if matches!(top, Token::LParen) {
                        break;
                    }
                    output.push(top);
                }
            }
        }
    }

    while let Some(token) = ops.pop() {
        output.push(token);
    }

    output
}

fn evaluate_rpn(rpn: &[Token]) -> Result<f64, ()> {
    let mut stack: Vec<f64> = Vec::new();

    for token in rpn {
        match token {
            Token::Number(n) => stack.push(*n),
            Token::Operator(op) => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                let res = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    '^' => a.powf(b),
                    _ => return Err(()),
                };
                stack.push(res);
            }
            _ => return Err(()),
        }
    }

    stack.pop().ok_or(())
}
