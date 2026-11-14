//! Delete redundant delimiters without rewriting literals, comments or operators.
use syn::{BinOp, Expr, Stmt, visit_mut::VisitMut};

/// Preserve the expression tree, including right-hand groups of equal precedence
/// (floating-point arithmetic is not associative). Unknown syntax is conservative.
pub fn remove_redundant_parentheses(source: &str) -> String {
    const PREFIX: &str = "fn formula() {\n";

    let wrapped = format!("{PREFIX}{source}\n}}");
    let Ok(mut file) = syn::parse_file(&wrapped) else {
        return source.into();
    };

    let mut visitor = Parentheses {
        minimum: 0,
        removed: Vec::new(),
    };

    visitor.visit_file_mut(&mut file);

    let mut bytes = source.as_bytes().to_vec();

    visitor.removed.sort_unstable();
    visitor.removed.dedup();

    for index in visitor.removed.into_iter().rev() {
        if let Some(index) = index.checked_sub(PREFIX.len())
            && index < bytes.len()
        {
            if index > 0
                && index + 1 < bytes.len()
                && needs_separator(bytes[index - 1], bytes[index + 1])
            {
                bytes[index] = b' ';
            } else {
                bytes.remove(index);
            }
        }
    }

    // Casts followed by `<`/`<<`, for example, need groups beyond arithmetic
    // precedence because Rust otherwise reads generic type arguments. Keep the
    // original source whenever a proposed removal violates Rust's grammar.
    let result = String::from_utf8(bytes).expect("Parenthesis removal preserves UTF-8");

    if syn::parse_file(&format!("{PREFIX}{result}\n}}")).is_ok() {
        result
    } else {
        source.into()
    }
}

fn needs_separator(left: u8, right: u8) -> bool {
    let word = |byte: u8| byte.is_ascii_alphanumeric() || byte == b'_' || byte >= 128;

    (word(left) && word(right))
        || (left.is_ascii_digit() && right == b'.')
        || matches!(
            (left, right),
            (b'/', b'*' | b'/')
                | (b'.', b'.')
                | (b'<', b'<' | b'=')
                | (b'>', b'>' | b'=')
                | (b'=', b'=')
                | (b'!', b'=')
                | (b'&', b'&' | b'=')
                | (b'|', b'|' | b'=')
                | (b'+' | b'-' | b'*' | b'/' | b'%' | b'^', b'=')
        )
}

struct Parentheses {
    minimum: u8,
    removed: Vec<usize>,
}

fn precedence(expr: &Expr) -> Option<u8> {
    Some(match expr {
        Expr::Paren(expr) => return precedence(&expr.expr),
        Expr::Group(expr) => return precedence(&expr.expr),
        Expr::Binary(expr) => match expr.op {
            BinOp::Or(_) => 1,
            BinOp::And(_) => 2,
            BinOp::Eq(_)
            | BinOp::Ne(_)
            | BinOp::Lt(_)
            | BinOp::Le(_)
            | BinOp::Gt(_)
            | BinOp::Ge(_) => 3,
            BinOp::BitOr(_) => 4,
            BinOp::BitXor(_) => 5,
            BinOp::BitAnd(_) => 6,
            BinOp::Shl(_) | BinOp::Shr(_) => 7,
            BinOp::Add(_) | BinOp::Sub(_) => 8,
            BinOp::Mul(_) | BinOp::Div(_) | BinOp::Rem(_) => 9,
            _ => return None,
        },
        Expr::Cast(_) => 10,
        Expr::Unary(_) => 11,
        Expr::Lit(_)
        | Expr::Path(_)
        | Expr::Field(_)
        | Expr::Call(_)
        | Expr::MethodCall(_)
        | Expr::Index(_) => 13,
        // Keep block-like operands grouped to avoid Rust statement ambiguity.
        Expr::Match(_) | Expr::Block(_) | Expr::If(_) => 0,
        _ => return None,
    })
}

impl Parentheses {
    fn expression(&mut self, expr: &mut Expr, minimum: u8) {
        let previous = std::mem::replace(&mut self.minimum, minimum);

        self.visit_expr_mut(expr);
        self.minimum = previous;
    }
}

impl VisitMut for Parentheses {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Paren(paren) => {
                if paren.attrs.is_empty()
                    && precedence(&paren.expr).is_some_and(|value| value >= self.minimum)
                {
                    self.removed
                        .push(paren.paren_token.span.open().byte_range().start);

                    self.removed
                        .push(paren.paren_token.span.close().byte_range().start);

                    self.visit_expr_mut(&mut paren.expr);
                } else {
                    // One retained pair already groups all of its contents.
                    self.expression(&mut paren.expr, 0);
                }
            }
            Expr::Group(group) => self.visit_expr_mut(&mut group.expr),
            Expr::Binary(_) => {
                let minimum = precedence(expr).unwrap_or(u8::MAX - 1);
                let Expr::Binary(binary) = expr else {
                    unreachable!()
                };

                let left = minimum + u8::from(minimum == 3);

                self.expression(&mut binary.left, left);
                self.expression(&mut binary.right, minimum + 1);
            }
            Expr::Unary(unary) => self.expression(&mut unary.expr, 11),
            Expr::Cast(cast) => self.expression(&mut cast.expr, 10),
            Expr::Field(field) => self.expression(&mut field.base, 13),
            Expr::Call(call) => {
                self.expression(&mut call.func, 13);

                for arg in &mut call.args {
                    self.expression(arg, 0);
                }
            }
            Expr::MethodCall(call) => {
                self.expression(&mut call.receiver, 13);

                for arg in &mut call.args {
                    self.expression(arg, 0);
                }
            }
            Expr::Index(index) => {
                self.expression(&mut index.expr, 13);
                self.expression(&mut index.index, 0);
            }
            Expr::Match(value) => {
                self.expression(&mut value.expr, 0);

                for arm in &mut value.arms {
                    self.expression(&mut arm.body, 0);
                }
            }
            Expr::Block(block) => self.visit_block_mut(&mut block.block),
            Expr::If(value) => {
                self.expression(&mut value.cond, 0);
                self.visit_block_mut(&mut value.then_branch);

                if let Some((_, value)) = &mut value.else_branch {
                    self.expression(value, 0);
                }
            }
            _ => {
                let previous = core::mem::replace(&mut self.minimum, u8::MAX);

                syn::visit_mut::visit_expr_mut(self, expr);
                self.minimum = previous;
            }
        }
    }

    fn visit_block_mut(&mut self, block: &mut syn::Block) {
        for statement in &mut block.stmts {
            match statement {
                Stmt::Expr(expr, _) => self.expression(expr, 0),
                Stmt::Local(local) => {
                    if let Some(init) = &mut local.init {
                        self.expression(&mut init.expr, 0);

                        if let Some((_, value)) = &mut init.diverge {
                            self.expression(value, 0);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
