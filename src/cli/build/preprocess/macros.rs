use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MacroDefinition {
    pub name: String,
    pub params: Vec<String>,
    pub body: String,
}

pub fn process_macros(code: &str) -> Result<String, String> {
    let (code_without_defs, macros) = extract_macro_definitions(code)?;
    let expanded = expand_macros(&code_without_defs, &macros)?;

    Ok(expanded)
}

pub fn extract_macro_definitions(
    code: &str,
) -> Result<(String, HashMap<String, MacroDefinition>), String> {
    let mut macros = HashMap::new();
    let mut result = String::new();
    let mut chars = code.chars().peekable();
    let mut pos = 0;

    while let Some(ch) = chars.next() {
        pos += ch.len_utf8();

        if ch == 'd' && code[pos - 1..].starts_with("define!") {
            for _ in 0..6 {
                chars.next();
                pos += 1;
            }

            skip_whitespace(&mut chars, &mut pos);

            let name = extract_identifier(&mut chars, &mut pos)?;
            if name.is_empty() {
                return Err("Macro definition missing name after 'define!'".to_string());
            }

            skip_whitespace(&mut chars, &mut pos);

            let params = if chars.peek() == Some(&'(') {
                chars.next();
                pos += 1;
                extract_parameters(&mut chars, &mut pos)?
            } else {
                Vec::new()
            };

            skip_whitespace(&mut chars, &mut pos);

            if chars.peek() != Some(&'{') {
                return Err(format!("Expected '{{' after macro definition '{}'", name));
            }
            chars.next();
            pos += 1;

            let body = extract_balanced_braces(&mut chars, &mut pos)?;

            let trimmed_body = body.trim().to_string();

            macros.insert(
                name.clone(),
                MacroDefinition {
                    name,
                    params,
                    body: trimmed_body,
                },
            );
        } else {
            result.push(ch);
        }
    }

    Ok((result, macros))
}

pub fn extract_parameters<I>(
    chars: &mut std::iter::Peekable<I>,
    pos: &mut usize,
) -> Result<Vec<String>, String>
where
    I: Iterator<Item = char>,
{
    let mut params = Vec::new();
    let mut current_param = String::new();

    loop {
        skip_whitespace(chars, pos);

        match chars.peek() {
            Some(&')') => {
                chars.next();
                *pos += 1;

                if !current_param.trim().is_empty() {
                    params.push(current_param.trim().to_string());
                }
                break;
            }
            Some(&',') => {
                chars.next();
                *pos += 1;
                if !current_param.trim().is_empty() {
                    params.push(current_param.trim().to_string());
                    current_param.clear();
                }
            }
            Some(&ch) if ch.is_alphanumeric() || ch == '_' => {
                current_param.push(ch);
                chars.next();
                *pos += ch.len_utf8();
            }
            Some(&ch) => {
                return Err(format!("Unexpected character '{}' in parameter list", ch));
            }
            None => {
                return Err("Unexpected end of input in parameter list".to_string());
            }
        }
    }

    Ok(params)
}

pub fn extract_identifier<I>(
    chars: &mut std::iter::Peekable<I>,
    pos: &mut usize,
) -> Result<String, String>
where
    I: Iterator<Item = char>,
{
    let mut name = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_alphanumeric() || ch == '_' {
            name.push(ch);
            chars.next();
            *pos += ch.len_utf8();
        } else {
            break;
        }
    }
    Ok(name)
}

pub fn skip_whitespace<I>(chars: &mut std::iter::Peekable<I>, pos: &mut usize)
where
    I: Iterator<Item = char>,
{
    while let Some(&ws) = chars.peek() {
        if ws.is_whitespace() {
            chars.next();
            *pos += ws.len_utf8();
        } else {
            break;
        }
    }
}

pub fn extract_balanced_braces<I>(
    chars: &mut std::iter::Peekable<I>,
    pos: &mut usize,
) -> Result<String, String>
where
    I: Iterator<Item = char>,
{
    let mut body = String::new();
    let mut depth = 1;

    while let Some(ch) = chars.next() {
        *pos += ch.len_utf8();

        match ch {
            '{' => {
                depth += 1;
                body.push(ch);
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(body);
                }
                body.push(ch);
            }
            _ => {
                body.push(ch);
            }
        }
    }

    Err("Unmatched braces in macro definition".to_string())
}

pub fn expand_macros(
    code: &str,
    macros: &HashMap<String, MacroDefinition>,
) -> Result<String, String> {
    let mut result = String::new();
    let mut chars = code.chars().peekable();
    let mut pos = 0;

    while let Some(ch) = chars.next() {
        pos += ch.len_utf8();

        if ch.is_alphabetic() || ch == '_' {
            let mut name = String::new();
            name.push(ch);

            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_alphanumeric() || next_ch == '_' {
                    name.push(next_ch);
                    chars.next();
                    pos += next_ch.len_utf8();
                } else {
                    break;
                }
            }

            skip_whitespace(&mut chars, &mut pos);

            let args = if chars.peek() == Some(&'(') {
                chars.next();
                pos += 1;
                Some(extract_arguments(&mut chars, &mut pos)?)
            } else {
                None
            };

            skip_whitespace(&mut chars, &mut pos);

            if chars.peek() == Some(&'!') {
                chars.next();
                pos += 1;

                skip_whitespace(&mut chars, &mut pos);

                if chars.peek() != Some(&'{') {
                    return Err(format!("Expected '{{' after macro call '{}!'", name));
                }
                chars.next();
                pos += 1;

                let body = extract_balanced_braces(&mut chars, &mut pos)?;

                if let Some(macro_def) = macros.get(&name) {
                    let expanded = substitute_macro(macro_def, args.as_deref(), &body)?;

                    let fully_expanded = expand_macros(&expanded, macros)?;
                    result.push_str(&fully_expanded);
                } else {
                    return Err(format!("Undefined macro: '{}'", name));
                }
            } else {
                result.push_str(&name);
                if let Some(ref arg_list) = args {
                    result.push('(');

                    let expanded_args = expand_macros(arg_list, macros)?;
                    result.push_str(&expanded_args);
                    result.push(')');
                }
            }
        } else {
            result.push(ch);
        }
    }

    Ok(result)
}

pub fn extract_arguments<I>(
    chars: &mut std::iter::Peekable<I>,
    pos: &mut usize,
) -> Result<String, String>
where
    I: Iterator<Item = char>,
{
    let mut depth = 1;
    let mut args = String::new();

    while let Some(ch) = chars.next() {
        *pos += ch.len_utf8();

        match ch {
            '(' => {
                depth += 1;
                args.push(ch);
            }
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(args);
                }
                args.push(ch);
            }
            _ => {
                args.push(ch);
            }
        }
    }

    Err("Unmatched parentheses in macro arguments".to_string())
}

pub fn substitute_macro(
    macro_def: &MacroDefinition,
    args: Option<&str>,
    body: &str,
) -> Result<String, String> {
    let mut result = macro_def.body.clone();

    if !macro_def.params.is_empty() {
        let arg_values = if let Some(args_str) = args {
            parse_argument_values(args_str)?
        } else {
            return Err(format!(
                "Macro '{}' expects {} arguments, but none were provided",
                macro_def.name,
                macro_def.params.len()
            ));
        };

        if arg_values.len() != macro_def.params.len() {
            return Err(format!(
                "Macro '{}' expects {} arguments, but {} were provided",
                macro_def.name,
                macro_def.params.len(),
                arg_values.len()
            ));
        }

        for (param, value) in macro_def.params.iter().zip(arg_values.iter()) {
            result = result.replace(param, value.trim());
        }
    }

    result = result.replace("%0", body.trim());

    Ok(result)
}

pub fn parse_argument_values(args: &str) -> Result<Vec<String>, String> {
    let mut values = Vec::new();
    let mut current = String::new();
    let mut depth = 0;

    for ch in args.chars() {
        match ch {
            '(' | '{' | '[' => {
                depth += 1;
                current.push(ch);
            }
            ')' | '}' | ']' => {
                depth -= 1;
                current.push(ch);
            }
            ',' if depth == 0 => {
                values.push(current.trim().to_string());
                current.clear();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.trim().is_empty() {
        values.push(current.trim().to_string());
    }

    Ok(values)
}
