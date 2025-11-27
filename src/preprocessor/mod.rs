const INDENT_MARKER: &str = "⟨INDENT⟩";
const DEDENT_MARKER: &str = "⟨DEDENT⟩";

/// Keywords that continue a compound statement (should not trigger DEDENT)
const CONTINUATION_KEYWORDS: &[&str] = &["elif", "else"];

/// Preprocess source code to insert explicit INDENT/DEDENT markers
pub fn preprocess(source: &str) -> Result<String, String> {
    let mut result = String::new();
    let mut indent_stack: Vec<usize> = vec![0];
    let lines: Vec<&str> = source.lines().collect();

    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Handle empty lines and comment-only lines
        if trimmed.is_empty() {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        // Calculate current line's indentation
        let current_indent = count_leading_spaces(line);
        let prev_indent = *indent_stack.last().unwrap();

        // Check if this line starts with a continuation keyword (elif, else)
        CONTINUATION_KEYWORDS.iter().any(|kw| {
            trimmed.starts_with(kw)
                && trimmed[kw.len()..]
                    .chars()
                    .next()
                    .is_none_or(|c| !c.is_alphanumeric() && c != '_')
        });

        if current_indent > prev_indent {
            // Increased indentation - emit INDENT
            indent_stack.push(current_indent);
            result.push_str(INDENT_MARKER);
        } else if current_indent < prev_indent {
            // Decreased indentation - emit DEDENT(s)
            // But if this is a continuation keyword, we need to be careful
            while let Some(&top) = indent_stack.last() {
                if top > current_indent {
                    indent_stack.pop();
                    result.push_str(DEDENT_MARKER);
                } else if top == current_indent {
                    break;
                } else {
                    return Err(format!(
                        "Line {}: Inconsistent indentation (got {} spaces, expected {} or {})",
                        line_num + 1,
                        current_indent,
                        top,
                        indent_stack
                            .get(indent_stack.len().saturating_sub(2))
                            .unwrap_or(&0)
                    ));
                }
            }
        }
        // Same indentation level - no marker needed

        // Append the line content (without leading whitespace)
        result.push_str(trimmed);
        result.push('\n');
    }

    // Emit remaining DEDENTs at end of file
    while indent_stack.len() > 1 {
        indent_stack.pop();
        result.push_str(DEDENT_MARKER);
    }

    // Ensure file ends with newline
    if !result.ends_with('\n') {
        result.push('\n');
    }

    Ok(result)
}

/// Count leading spaces, converting tabs to 4 spaces
fn count_leading_spaces(line: &str) -> usize {
    let mut count = 0;
    for ch in line.chars() {
        match ch {
            ' ' => count += 1,
            '\t' => count += 4,
            _ => break,
        }
    }
    count
}
