use similar::{ChangeTag, TextDiff};

/// Generate unified diff between two texts.
pub fn generate_diff(old: &str, new: &str) -> String {
    let diff = TextDiff::from_lines(old, new);
    let mut output = String::new();

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        output.push_str(&format!("{}{}", sign, change));
    }

    output
}

/// Apply a unified diff to text.
/// Expects diff in the format produced by generate_diff.
pub fn apply_diff(content: &str, diff: &str) -> Result<String, String> {
    let lines: Vec<&str> = content.lines().collect();
    let diff_lines: Vec<&str> = diff.lines().collect();

    let mut result = Vec::new();
    let mut content_idx = 0;

    for diff_line in &diff_lines {
        if diff_line.is_empty() {
            continue;
        }

        let (prefix, text) = diff_line.split_at(1);
        match prefix {
            " " => {
                // Context line - must match current content line
                if content_idx >= lines.len() {
                    return Err("Diff references beyond end of content".to_string());
                }
                if lines[content_idx] != text {
                    return Err(format!(
                        "Context mismatch at line {}: expected '{}', found '{}'",
                        content_idx + 1,
                        text,
                        lines[content_idx]
                    ));
                }
                result.push(lines[content_idx].to_string());
                content_idx += 1;
            }
            "-" => {
                // Deletion - skip this line from content
                if content_idx >= lines.len() {
                    return Err("Diff deletion references beyond end of content".to_string());
                }
                if lines[content_idx] != text {
                    return Err(format!(
                        "Deletion mismatch at line {}: expected '{}', found '{}'",
                        content_idx + 1,
                        text,
                        lines[content_idx]
                    ));
                }
                content_idx += 1;
            }
            "+" => {
                // Addition - insert this line
                result.push(text.to_string());
            }
            _ => {
                return Err(format!("Unknown diff prefix: '{}'", prefix));
            }
        }
    }

    // Add remaining lines from content
    while content_idx < lines.len() {
        result.push(lines[content_idx].to_string());
        content_idx += 1;
    }

    Ok(result.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_diff_identical() {
        let diff = generate_diff("hello", "hello");
        // All lines should be context (space prefix)
        for line in diff.lines() {
            assert!(line.starts_with(' '), "Expected context line: {}", line);
        }
    }

    #[test]
    fn test_generate_diff_addition() {
        let diff = generate_diff("hello", "hello\nworld");
        assert!(diff.contains("+world"));
    }

    #[test]
    fn test_generate_diff_deletion() {
        let diff = generate_diff("hello\nworld", "hello");
        assert!(diff.contains("-world"));
    }

    #[test]
    fn test_generate_diff_modification() {
        let diff = generate_diff("hello", "world");
        assert!(diff.contains("-hello"));
        assert!(diff.contains("+world"));
    }

    #[test]
    fn test_generate_diff_multiline() {
        let old = "line1\nline2\nline3";
        let new = "line1\nmodified\nline3";
        let diff = generate_diff(old, new);
        assert!(diff.contains("-line2"));
        assert!(diff.contains("+modified"));
        assert!(diff.contains(" line1"));
        assert!(diff.contains(" line3"));
    }

    #[test]
    fn test_apply_diff_identity() {
        let content = "hello\nworld";
        let diff = generate_diff(content, content);
        let result = apply_diff(content, &diff).unwrap();
        assert_eq!(result, content);
    }

    #[test]
    fn test_apply_diff_addition() {
        let old = "hello";
        let new = "hello\nworld";
        let diff = generate_diff(old, new);
        let result = apply_diff(old, &diff).unwrap();
        assert_eq!(result, new);
    }

    #[test]
    fn test_apply_diff_deletion() {
        let old = "hello\nworld";
        let new = "hello";
        let diff = generate_diff(old, new);
        let result = apply_diff(old, &diff).unwrap();
        assert_eq!(result, new);
    }

    #[test]
    fn test_apply_diff_modification() {
        let old = "hello\nworld\nfoo";
        let new = "hello\nrust\nfoo";
        let diff = generate_diff(old, new);
        let result = apply_diff(old, &diff).unwrap();
        assert_eq!(result, new);
    }

    #[test]
    fn test_apply_diff_mismatch() {
        let content = "hello\nworld";
        // Create a diff that won't match
        let bad_diff = " hello\n+added\n-other";
        let result = apply_diff(content, bad_diff);
        assert!(result.is_err());
    }

    #[test]
    fn test_roundtrip() {
        let old = "The quick brown fox\njumps over\nthe lazy dog";
        let new = "The quick red fox\njumps over\nthe lazy cat\nand sleeps";
        let diff = generate_diff(old, new);
        let result = apply_diff(old, &diff).unwrap();
        assert_eq!(result, new);
    }

    #[test]
    fn test_apply_diff_empty_content() {
        let diff = generate_diff("", "hello");
        let result = apply_diff("", &diff).unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_apply_diff_to_empty() {
        let diff = generate_diff("hello", "");
        let result = apply_diff("hello", &diff).unwrap();
        assert_eq!(result, "");
    }
}
