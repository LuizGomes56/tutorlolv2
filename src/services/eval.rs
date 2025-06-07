#[derive(Clone)]
enum Token {
    Number(f64),
    Operator(char),
    LParen,
    RParen,
}

// Takes a string and evaluates it mathematically.
// None values mean NaN as in JavaScript
// Only parenthesis may be used. [] and {} are not supported
// Exponentiation is done with the ^ operator, not with the ** operator
pub fn eval_math_expr(expr: &str) -> Result<f64, ()> {
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
