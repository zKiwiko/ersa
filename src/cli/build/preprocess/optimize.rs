pub fn optimize(code: &str) -> Result<String, String> {
    constant_fold(code)
}

pub fn constant_fold(code: &str) -> Result<String, String> {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = code.chars().collect();

    while i < chars.len() {
        if let Some((expr_start, expr_end)) = find_foldable_expression(&chars, i) {
            result.push_str(&chars[i..expr_start].iter().collect::<String>());

            let expr: String = chars[expr_start..expr_end].iter().collect();

            if let Ok(value) = evaluate_expression(&expr) {
                result.push_str(&value.to_string());
            } else {
                result.push_str(&expr);
            }

            i = expr_end;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    Ok(result)
}

fn find_foldable_expression(chars: &[char], start: usize) -> Option<(usize, usize)> {
    if start >= chars.len() {
        return None;
    }

    let mut i = start;
    while i < chars.len() && chars[i].is_whitespace() {
        i += 1;
    }

    if i >= chars.len() {
        return None;
    }

    if !chars[i].is_ascii_digit() && chars[i] != '(' && chars[i] != '-' {
        return None;
    }

    let expr_start = i;
    let mut depth = 0;
    let mut has_operator = false;
    let mut last_was_number = false;

    while i < chars.len() {
        let ch = chars[i];

        match ch {
            '0'..='9' => {
                last_was_number = true;
                i += 1;
            }
            '+' | '*' | '/' | '%' | '&' | '|' | '^' => {
                has_operator = true;
                last_was_number = false;
                i += 1;
            }
            '-' => {
                has_operator = true;
                last_was_number = false;
                i += 1;
            }
            '<' | '>' => {
                if i + 1 < chars.len() && chars[i + 1] == ch {
                    has_operator = true;
                    i += 2;
                    last_was_number = false;
                } else {
                    break;
                }
            }
            '(' => {
                depth += 1;
                last_was_number = false;
                i += 1;
            }
            ')' => {
                if depth > 0 {
                    depth -= 1;
                    last_was_number = true;
                    i += 1;
                } else {
                    break;
                }
            }
            ' ' | '\t' => {
                i += 1;
            }
            ',' | ';' | '{' | '}' | '=' | '!' | '\n' => {
                break;
            }
            _ => {
                break;
            }
        }
    }

    if has_operator && (last_was_number || depth == 0) && i > expr_start {
        Some((expr_start, i))
    } else {
        None
    }
}

fn evaluate_expression(expr: &str) -> Result<i64, String> {
    let tokens = tokenize(expr)?;
    if !is_constant_expression(&tokens) {
        return Err("Not a constant expression".to_string());
    }
    parse_expression(&tokens, 0).map(|(val, _)| val)
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum ExprToken {
    Number(i64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    BitAnd,
    BitOr,
    BitXor,
    LeftShift,
    RightShift,
    And,
    Or,
    Xor,
    LParen,
    RParen,
}

fn tokenize(expr: &str) -> Result<Vec<ExprToken>, String> {
    let mut tokens = Vec::new();
    let mut chars = expr.trim().chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' => {
                chars.next();
            }
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_ascii_digit() {
                        num.push(digit);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(ExprToken::Number(
                    num.parse().map_err(|_| "Invalid number")?,
                ));
            }
            '+' => {
                chars.next();
                tokens.push(ExprToken::Plus);
            }
            '-' => {
                chars.next();

                if tokens.is_empty()
                    || matches!(
                        tokens.last(),
                        Some(ExprToken::LParen)
                            | Some(ExprToken::Plus)
                            | Some(ExprToken::Minus)
                            | Some(ExprToken::Multiply)
                            | Some(ExprToken::Divide)
                            | Some(ExprToken::Modulo)
                    )
                {
                    let mut num = String::from("-");
                    while let Some(&digit) = chars.peek() {
                        if digit.is_ascii_digit() {
                            num.push(digit);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(ExprToken::Number(
                        num.parse().map_err(|_| "Invalid number")?,
                    ));
                } else {
                    tokens.push(ExprToken::Minus);
                }
            }
            '*' => {
                chars.next();
                tokens.push(ExprToken::Multiply);
            }
            '/' => {
                chars.next();
                tokens.push(ExprToken::Divide);
            }
            '%' => {
                chars.next();
                tokens.push(ExprToken::Modulo);
            }
            '&' => {
                chars.next();
                if chars.peek() == Some(&'&') {
                    chars.next();
                    tokens.push(ExprToken::And);
                } else {
                    tokens.push(ExprToken::BitAnd);
                }
            }
            '|' => {
                chars.next();
                if chars.peek() == Some(&'|') {
                    chars.next();
                    tokens.push(ExprToken::Or);
                } else {
                    tokens.push(ExprToken::BitOr);
                }
            }
            '^' => {
                chars.next();
                tokens.push(ExprToken::BitXor);
            }
            '<' => {
                chars.next();
                if chars.peek() == Some(&'<') {
                    chars.next();
                    tokens.push(ExprToken::LeftShift);
                } else {
                    return Err(
                        "Comparison operators not supported in constant expressions".to_string()
                    );
                }
            }
            '>' => {
                chars.next();
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(ExprToken::RightShift);
                } else {
                    return Err(
                        "Comparison operators not supported in constant expressions".to_string()
                    );
                }
            }
            '(' => {
                chars.next();
                tokens.push(ExprToken::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(ExprToken::RParen);
            }
            _ => {
                return Err(format!("Unexpected character in expression: '{}'", ch));
            }
        }
    }

    Ok(tokens)
}

fn is_constant_expression(tokens: &[ExprToken]) -> bool {
    !tokens.is_empty()
        && tokens.iter().all(|t| {
            matches!(
                t,
                ExprToken::Number(_)
                    | ExprToken::Plus
                    | ExprToken::Minus
                    | ExprToken::Multiply
                    | ExprToken::Divide
                    | ExprToken::Modulo
                    | ExprToken::BitAnd
                    | ExprToken::BitOr
                    | ExprToken::BitXor
                    | ExprToken::LeftShift
                    | ExprToken::RightShift
                    | ExprToken::And
                    | ExprToken::Or
                    | ExprToken::Xor
                    | ExprToken::LParen
                    | ExprToken::RParen
            )
        })
}

fn precedence(token: &ExprToken) -> u8 {
    match token {
        ExprToken::Or => 1,
        ExprToken::Xor => 2,
        ExprToken::And => 3,
        ExprToken::BitOr => 4,
        ExprToken::BitXor => 5,
        ExprToken::BitAnd => 6,
        ExprToken::LeftShift | ExprToken::RightShift => 9,
        ExprToken::Plus | ExprToken::Minus => 10,
        ExprToken::Multiply | ExprToken::Divide | ExprToken::Modulo => 11,
        _ => 0,
    }
}

fn parse_expression(tokens: &[ExprToken], pos: usize) -> Result<(i64, usize), String> {
    parse_binary_expression(tokens, pos, 0)
}

fn parse_binary_expression(
    tokens: &[ExprToken],
    mut pos: usize,
    min_prec: u8,
) -> Result<(i64, usize), String> {
    let (mut left, new_pos) = parse_primary(tokens, pos)?;
    pos = new_pos;

    while pos < tokens.len() {
        let op = &tokens[pos];
        let prec = precedence(op);

        if prec < min_prec {
            break;
        }

        if matches!(op, ExprToken::RParen) {
            break;
        }

        pos += 1;

        let (right, new_pos) = parse_binary_expression(tokens, pos, prec + 1)?;
        pos = new_pos;

        left = apply_operator(left, op, right)?;
    }

    Ok((left, pos))
}

fn parse_primary(tokens: &[ExprToken], pos: usize) -> Result<(i64, usize), String> {
    if pos >= tokens.len() {
        return Err("Unexpected end of expression".to_string());
    }

    match &tokens[pos] {
        ExprToken::Number(n) => Ok((*n, pos + 1)),
        ExprToken::LParen => {
            let (value, new_pos) = parse_expression(tokens, pos + 1)?;
            if new_pos >= tokens.len() || !matches!(tokens[new_pos], ExprToken::RParen) {
                return Err("Missing closing parenthesis".to_string());
            }
            Ok((value, new_pos + 1))
        }
        ExprToken::Minus => {
            let (value, new_pos) = parse_primary(tokens, pos + 1)?;
            Ok((-value, new_pos))
        }
        _ => Err(format!("Unexpected token in expression: {:?}", tokens[pos])),
    }
}

fn apply_operator(left: i64, op: &ExprToken, right: i64) -> Result<i64, String> {
    match op {
        ExprToken::Plus => Ok(left.wrapping_add(right)),
        ExprToken::Minus => Ok(left.wrapping_sub(right)),
        ExprToken::Multiply => Ok(left.wrapping_mul(right)),
        ExprToken::Divide => {
            if right == 0 {
                Err("Division by zero".to_string())
            } else {
                Ok(left / right)
            }
        }
        ExprToken::Modulo => {
            if right == 0 {
                Err("Modulo by zero".to_string())
            } else {
                Ok(left % right)
            }
        }
        ExprToken::BitAnd => Ok(left & right),
        ExprToken::BitOr => Ok(left | right),
        ExprToken::BitXor => Ok(left ^ right),
        ExprToken::LeftShift => Ok(left << right),
        ExprToken::RightShift => Ok(left >> right),
        ExprToken::And => Ok(if left != 0 && right != 0 { 1 } else { 0 }),
        ExprToken::Or => Ok(if left != 0 || right != 0 { 1 } else { 0 }),
        ExprToken::Xor => Ok(if (left != 0) != (right != 0) { 1 } else { 0 }),
        _ => Err(format!("Invalid operator: {:?}", op)),
    }
}
