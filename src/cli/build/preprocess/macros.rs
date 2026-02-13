use std::collections::HashMap;

/// Macro definition structure
#[derive(Debug, Clone)]
pub struct MacroDefinition {
    pub name: String,
    pub params: Vec<String>,
    pub body: String,
}

/// Process macro definitions and expansions
/// Supports: define! name(param1, param2) { body } and name(arg1, arg2)! { body }
pub fn process_macros(code: &str) -> Result<String, String> {
    let (code_without_defs, macros) = extract_macro_definitions(code)?;
    let expanded = expand_macros(&code_without_defs, &macros)?;

    Ok(expanded)
}

/// Extract macro definitions from code
/// Returns (code_without_definitions, macro_map)
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

            // Extract macro name
            let name = extract_identifier(&mut chars, &mut pos)?;
            if name.is_empty() {
                return Err("Macro definition missing name after 'define!'".to_string());
            }

            skip_whitespace(&mut chars, &mut pos);

            // Check for parameters
            let params = if chars.peek() == Some(&'(') {
                chars.next(); // consume '('
                pos += 1;
                extract_parameters(&mut chars, &mut pos)?
            } else {
                Vec::new()
            };

            skip_whitespace(&mut chars, &mut pos);

            // Expect '{'
            if chars.peek() != Some(&'{') {
                return Err(format!("Expected '{{' after macro definition '{}'", name));
            }
            chars.next(); // consume '{'
            pos += 1;

            // Extract balanced body
            let body = extract_balanced_braces(&mut chars, &mut pos)?;

            // Trim the body to remove leading/trailing whitespace
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

/// Extract a list of parameters from parentheses
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
                chars.next(); // consume ')'
                *pos += 1;
                // Add last parameter if not empty
                if !current_param.trim().is_empty() {
                    params.push(current_param.trim().to_string());
                }
                break;
            }
            Some(&',') => {
                chars.next(); // consume ','
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

/// Extract an identifier (alphanumeric + underscore)
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

/// Skip whitespace characters
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

/// Extract content between balanced braces
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

/// Expand macro usages in code
pub fn expand_macros(
    code: &str,
    macros: &HashMap<String, MacroDefinition>,
) -> Result<String, String> {
    let mut result = String::new();
    let mut chars = code.chars().peekable();
    let mut pos = 0;

    while let Some(ch) = chars.next() {
        pos += ch.len_utf8();

        // Check if this could be a macro call (alphanumeric/underscore)
        if ch.is_alphabetic() || ch == '_' {
            let mut name = String::new();
            name.push(ch);

            // Collect rest of identifier
            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_alphanumeric() || next_ch == '_' {
                    name.push(next_ch);
                    chars.next();
                    pos += next_ch.len_utf8();
                } else {
                    break;
                }
            }

            // Check for macro call pattern: name(args)! or name!
            skip_whitespace(&mut chars, &mut pos);

            let args = if chars.peek() == Some(&'(') {
                chars.next(); // consume '('
                pos += 1;
                Some(extract_arguments(&mut chars, &mut pos)?)
            } else {
                None
            };

            skip_whitespace(&mut chars, &mut pos);

            // Check for '!'
            if chars.peek() == Some(&'!') {
                chars.next(); // consume '!'
                pos += 1;

                skip_whitespace(&mut chars, &mut pos);

                // Expect '{'
                if chars.peek() != Some(&'{') {
                    return Err(format!("Expected '{{' after macro call '{}!'", name));
                }
                chars.next(); // consume '{'
                pos += 1;

                // Extract body (what replaces %0)
                let body = extract_balanced_braces(&mut chars, &mut pos)?;

                // Look up macro
                if let Some(macro_def) = macros.get(&name) {
                    let expanded = substitute_macro(macro_def, args.as_deref(), &body)?;
                    // Recursively expand any macros in the substituted result
                    let fully_expanded = expand_macros(&expanded, macros)?;
                    result.push_str(&fully_expanded);
                } else {
                    return Err(format!("Undefined macro: '{}'", name));
                }
            } else {
                // Not a macro call, just a regular identifier (possibly with parens)
                result.push_str(&name);
                if let Some(ref arg_list) = args {
                    result.push('(');
                    // Recursively expand macros inside the arguments
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

/// Extract arguments from parentheses
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

/// Substitute macro parameters and body
pub fn substitute_macro(
    macro_def: &MacroDefinition,
    args: Option<&str>,
    body: &str,
) -> Result<String, String> {
    let mut result = macro_def.body.clone();

    // Substitute named parameters if provided
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

        // Substitute each parameter
        for (param, value) in macro_def.params.iter().zip(arg_values.iter()) {
            result = result.replace(param, value.trim());
        }
    }

    // Substitute %0 with the body
    result = result.replace("%0", body.trim());

    Ok(result)
}

/// Parse comma-separated argument values
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
