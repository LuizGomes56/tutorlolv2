//! Evaluator of the arithmetic accepted by the `{{#expr:}}` parser function, which
//! the wiki also applies to every argument of `{{ap}}` before reading it

/// Arithmetic expression as written in the wiki, such as `40*(1+0.4*2)` or `70/10`.
/// Supports `+ - * / ^ mod round`, the unary `abs floor ceil trunc` and parentheses.
/// As in the wiki, unary operators bind tighter than `^`, so `-2^2` is 4
pub struct Expr<'a> {
    src: &'a [u8],
    pos: usize,
}

impl<'a> Expr<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.as_bytes(),
            pos: 0,
        }
    }

    /// Value of the whole expression, or `None` if it is not valid arithmetic
    pub fn eval(mut self) -> Option<f64> {
        let value = self.binary(0)?;
        self.skip_spaces();
        (self.pos == self.src.len() && value.is_finite()).then_some(value)
    }

    fn skip_spaces(&mut self) {
        while self.src.get(self.pos).is_some_and(u8::is_ascii_whitespace) {
            self.pos += 1;
        }
    }

    fn eat(&mut self, token: &str) -> bool {
        self.skip_spaces();

        let matches = self.src[self.pos..].starts_with(token.as_bytes());
        // Words must not be the prefix of a longer word, as in `modifier`
        let is_word_end = !token.as_bytes()[0].is_ascii_alphabetic()
            || !self
                .src
                .get(self.pos + token.len())
                .is_some_and(u8::is_ascii_alphabetic);

        if matches && is_word_end {
            self.pos += token.len();
        }

        matches && is_word_end
    }

    /// Binary operators, from the lowest precedence to the highest
    const LEVELS: [&[&str]; 4] = [&["round"], &["+", "-"], &["*", "/", "mod"], &["^"]];

    fn binary(&mut self, level: usize) -> Option<f64> {
        let Some(operators) = Self::LEVELS.get(level) else {
            return self.unary();
        };

        let mut lhs = self.binary(level + 1)?;

        'outer: loop {
            for &operator in *operators {
                if self.eat(operator) {
                    // Every operator is left associative, `2^3^2` is 64 as in the wiki
                    let rhs = self.binary(level + 1)?;

                    lhs = match operator {
                        "round" => {
                            let factor = 10f64.powf(rhs.trunc());
                            (lhs * factor).round() / factor
                        }
                        "+" => lhs + rhs,
                        "-" => lhs - rhs,
                        "*" => lhs * rhs,
                        "/" => lhs / rhs,
                        "mod" => lhs.trunc() % rhs.trunc(),
                        _ => lhs.powf(rhs),
                    };

                    continue 'outer;
                }
            }

            return Some(lhs);
        }
    }

    fn unary(&mut self) -> Option<f64> {
        if self.eat("-") {
            return Some(-self.unary()?);
        }

        if self.eat("+") {
            return self.unary();
        }

        for (name, function) in [
            ("abs", f64::abs as fn(f64) -> f64),
            ("floor", f64::floor),
            ("ceil", f64::ceil),
            ("trunc", f64::trunc),
        ] {
            if self.eat(name) {
                return Some(function(self.unary()?));
            }
        }

        self.primary()
    }

    fn primary(&mut self) -> Option<f64> {
        if self.eat("(") {
            let value = self.binary(0)?;
            return self.eat(")").then_some(value);
        }

        self.skip_spaces();

        let start = self.pos;

        while self
            .src
            .get(self.pos)
            .is_some_and(|c| c.is_ascii_digit() || *c == b'.')
        {
            self.pos += 1;
        }

        std::str::from_utf8(&self.src[start..self.pos])
            .ok()?
            .parse()
            .ok()
    }
}
