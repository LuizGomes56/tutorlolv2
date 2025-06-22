use crate::model::{
    base::{AdaptativeType, Stats},
    champions::ChampionCdnStats,
};

pub(super) trait ConditionalAddition {
    fn add_if_some(&mut self, value: Option<f64>);
}

impl ConditionalAddition for f64 {
    fn add_if_some(&mut self, value: Option<Self>) {
        if let Some(v) = value {
            *self += v;
        }
    }
}

pub(super) struct RiotFormulas;

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
        macro_rules! assign_value {
            ($field:ident) => {
                Self::stat_growth(cdn.$field.flat, cdn.$field.per_level, level)
            };
        }

        Stats {
            ability_power: 0.0,
            armor: assign_value!(armor),
            armor_penetration_flat: 0.0,
            armor_penetration_percent: 0.0,
            attack_damage: assign_value!(attack_damage),
            attack_range: assign_value!(attack_range),
            attack_speed: assign_value!(attack_speed),
            crit_chance: 0.0,
            crit_damage: assign_value!(critical_strike_damage)
                * cdn.critical_strike_damage_modifier.flat,
            current_health: assign_value!(health),
            max_health: assign_value!(health),
            current_mana: assign_value!(mana),
            max_mana: assign_value!(mana),
            magic_penetration_flat: 0.0,
            magic_penetration_percent: 0.0,
            magic_resist: assign_value!(magic_resistance),
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
    let tokens = tokenize(expr)?;
    let rpn = shunting_yard(&tokens);
    evaluate_rpn(&rpn)
}

fn tokenize(expr: &str) -> Result<Vec<Token>, ()> {
    let mut tokens = Vec::new();
    let mut num_buf = String::new();
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
    let mut output = Vec::new();
    let mut ops = Vec::new();

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
    let mut stack = Vec::<f64>::new();

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
