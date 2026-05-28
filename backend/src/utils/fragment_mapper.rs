use serde::{Deserialize, Serialize};

/// A fragment represents a text edit at a specific position in content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, utoipa::ToSchema)]
pub struct Fragment {
    pub start_line: i32,
    pub start_col: i32,
    pub end_line: i32,
    pub end_col: i32,
    pub replacement: String,
    pub description: Option<String>,
}

/// Validate that fragment positions are within content bounds and don't overlap.
pub fn validate_fragments(content: &str, fragments: &[Fragment]) -> Result<(), String> {
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len() as i32;

    for (i, frag) in fragments.iter().enumerate() {
        // Check line bounds
        if frag.start_line < 1 || frag.start_line > total_lines {
            return Err(format!(
                "Fragment {}: start_line {} out of bounds (1-{})",
                i, frag.start_line, total_lines
            ));
        }
        if frag.end_line < 1 || frag.end_line > total_lines {
            return Err(format!(
                "Fragment {}: end_line {} out of bounds (1-{})",
                i, frag.end_line, total_lines
            ));
        }

        // Check that start <= end
        if frag.start_line > frag.end_line
            || (frag.start_line == frag.end_line && frag.start_col > frag.end_col)
        {
            return Err(format!(
                "Fragment {}: start position ({},{}) is after end position ({},{})",
                i, frag.start_line, frag.start_col, frag.end_line, frag.end_col
            ));
        }

        // Check column bounds
        let start_line_len = lines[(frag.start_line - 1) as usize].len() as i32;
        let end_line_len = lines[(frag.end_line - 1) as usize].len() as i32;

        if frag.start_col < 0 || frag.start_col > start_line_len {
            return Err(format!(
                "Fragment {}: start_col {} out of bounds for line {} (len {})",
                i, frag.start_col, frag.start_line, start_line_len
            ));
        }
        if frag.end_col < 0 || frag.end_col > end_line_len {
            return Err(format!(
                "Fragment {}: end_col {} out of bounds for line {} (len {})",
                i, frag.end_col, frag.end_line, end_line_len
            ));
        }
    }

    // Check for overlapping fragments
    for i in 0..fragments.len() {
        for j in (i + 1)..fragments.len() {
            let a = &fragments[i];
            let b = &fragments[j];
            if ranges_overlap(a, b) {
                return Err(format!("Fragments {} and {} overlap", i, j));
            }
        }
    }

    Ok(())
}

/// Check if two fragment ranges overlap.
fn ranges_overlap(a: &Fragment, b: &Fragment) -> bool {
    // a starts after b ends, or b starts after a ends => no overlap
    !(a.start_line > b.end_line
        || (a.start_line == b.end_line && a.start_col >= b.end_col)
        || b.start_line > a.end_line
        || (b.start_line == a.end_line && b.start_col >= a.end_col))
}

/// Apply fragments to content, returning the modified content.
/// Fragments must be valid (call validate_fragments first).
/// Fragments are applied in reverse order to preserve positions.
pub fn apply_fragments(content: &str, fragments: &[Fragment]) -> Result<String, String> {
    validate_fragments(content, fragments)?;

    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    // Sort fragments by position (last first) so we can apply without shifting
    let mut indexed: Vec<(usize, &Fragment)> = fragments.iter().enumerate().collect();
    indexed.sort_by(|a, b| {
        (b.1.end_line, b.1.end_col, b.0).cmp(&(a.1.end_line, a.1.end_col, a.0))
    });

    for (_orig_idx, frag) in indexed {
        apply_single_fragment(&mut lines, frag)?;
    }

    Ok(lines.join("\n"))
}

/// Apply a single fragment to the lines vector.
fn apply_single_fragment(lines: &mut Vec<String>, frag: &Fragment) -> Result<(), String> {
    let start_line = (frag.start_line - 1) as usize;
    let start_col = frag.start_col as usize;
    let end_line = (frag.end_line - 1) as usize;
    let end_col = frag.end_col as usize;

    if start_line == end_line {
        // Single line replacement
        let line = &lines[start_line];
        let prefix: String = line.chars().take(start_col).collect();
        let suffix: String = line.chars().skip(end_col).collect();
        lines[start_line] = format!("{}{}{}", prefix, frag.replacement, suffix);
    } else {
        // Multi-line replacement
        let first_line = &lines[start_line];
        let last_line = &lines[end_line];

        let prefix: String = first_line.chars().take(start_col).collect();
        let suffix: String = last_line.chars().skip(end_col).collect();

        let new_line = format!("{}{}{}", prefix, frag.replacement, suffix);

        // Remove the range and insert the new line
        lines.splice(start_line..=end_line, std::iter::once(new_line));
    }

    Ok(())
}

/// Map rendered text selection to source text position.
/// Useful for mapping user selections in rendered markdown back to source positions.
pub fn map_selection_to_source(
    rendered_text: &str,
    source_text: &str,
    sel_start: usize,
    sel_end: usize,
) -> Option<(usize, usize)> {
    // Simple line-based mapping: count lines in rendered text up to selection,
    // then find corresponding position in source text
    let rendered_lines: Vec<&str> = rendered_text.lines().collect();
    let source_lines: Vec<&str> = source_text.lines().collect();

    if rendered_lines.is_empty() || source_lines.is_empty() {
        return None;
    }

    // Find which line the selection starts on in rendered text
    let mut char_count = 0;
    let mut start_line = 0;
    for (i, line) in rendered_lines.iter().enumerate() {
        if char_count + line.len() + 1 > sel_start {
            start_line = i;
            break;
        }
        char_count += line.len() + 1; // +1 for newline
    }

    // Find which line the selection ends on
    char_count = 0;
    let mut end_line = 0;
    for (i, line) in rendered_lines.iter().enumerate() {
        if char_count + line.len() + 1 > sel_end {
            end_line = i;
            break;
        }
        char_count += line.len() + 1;
    }

    // Map to source text positions (line-based approximation)
    if start_line >= source_lines.len() || end_line >= source_lines.len() {
        return None;
    }

    let source_start: usize = source_lines[..start_line].iter().map(|l| l.len() + 1).sum();
    let source_end: usize = source_lines[..=end_line].iter().map(|l| l.len() + 1).sum();

    Some((source_start, source_end.saturating_sub(1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_fragments_valid() {
        let content = "Hello World\nFoo Bar\nBaz Qux";
        let fragments = vec![Fragment {
            start_line: 1,
            start_col: 0,
            end_line: 1,
            end_col: 5,
            replacement: "Hi".to_string(),
            description: None,
        }];
        assert!(validate_fragments(content, &fragments).is_ok());
    }

    #[test]
    fn test_validate_fragments_out_of_bounds_line() {
        let content = "Hello\nWorld";
        let fragments = vec![Fragment {
            start_line: 3,
            start_col: 0,
            end_line: 3,
            end_col: 5,
            replacement: "x".to_string(),
            description: None,
        }];
        assert!(validate_fragments(content, &fragments).is_err());
    }

    #[test]
    fn test_validate_fragments_out_of_bounds_col() {
        let content = "Hi";
        let fragments = vec![Fragment {
            start_line: 1,
            start_col: 0,
            end_line: 1,
            end_col: 100,
            replacement: "x".to_string(),
            description: None,
        }];
        assert!(validate_fragments(content, &fragments).is_err());
    }

    #[test]
    fn test_validate_fragments_overlapping() {
        let content = "Hello World";
        let fragments = vec![
            Fragment {
                start_line: 1,
                start_col: 0,
                end_line: 1,
                end_col: 5,
                replacement: "A".to_string(),
                description: None,
            },
            Fragment {
                start_line: 1,
                start_col: 3,
                end_line: 1,
                end_col: 8,
                replacement: "B".to_string(),
                description: None,
            },
        ];
        assert!(validate_fragments(content, &fragments).is_err());
    }

    #[test]
    fn test_validate_fragments_reversed_positions() {
        let content = "Hello";
        let fragments = vec![Fragment {
            start_line: 1,
            start_col: 4,
            end_line: 1,
            end_col: 1,
            replacement: "x".to_string(),
            description: None,
        }];
        assert!(validate_fragments(content, &fragments).is_err());
    }

    #[test]
    fn test_apply_single_fragment() {
        let content = "Hello World";
        let fragments = vec![Fragment {
            start_line: 1,
            start_col: 6,
            end_line: 1,
            end_col: 11,
            replacement: "Rust".to_string(),
            description: None,
        }];
        let result = apply_fragments(content, &fragments).unwrap();
        assert_eq!(result, "Hello Rust");
    }

    #[test]
    fn test_apply_fragment_at_start() {
        let content = "Hello World";
        let fragments = vec![Fragment {
            start_line: 1,
            start_col: 0,
            end_line: 1,
            end_col: 5,
            replacement: "Hi".to_string(),
            description: None,
        }];
        let result = apply_fragments(content, &fragments).unwrap();
        assert_eq!(result, "Hi World");
    }

    #[test]
    fn test_apply_fragment_at_end() {
        let content = "Hello World";
        let fragments = vec![Fragment {
            start_line: 1,
            start_col: 6,
            end_line: 1,
            end_col: 11,
            replacement: "Rust".to_string(),
            description: None,
        }];
        let result = apply_fragments(content, &fragments).unwrap();
        assert_eq!(result, "Hello Rust");
    }

    #[test]
    fn test_apply_multiline_fragment() {
        let content = "Line 1\nLine 2\nLine 3";
        let fragments = vec![Fragment {
            start_line: 1,
            start_col: 5,
            end_line: 3,
            end_col: 4,
            replacement: "replaced".to_string(),
            description: None,
        }];
        let result = apply_fragments(content, &fragments).unwrap();
        assert_eq!(result, "Line replaced 3");
    }

    #[test]
    fn test_apply_multiple_fragments() {
        let content = "Hello World\nFoo Bar";
        let fragments = vec![
            Fragment {
                start_line: 1,
                start_col: 0,
                end_line: 1,
                end_col: 5,
                replacement: "Hi".to_string(),
                description: None,
            },
            Fragment {
                start_line: 2,
                start_col: 0,
                end_line: 2,
                end_col: 3,
                replacement: "Baz".to_string(),
                description: None,
            },
        ];
        let result = apply_fragments(content, &fragments).unwrap();
        assert_eq!(result, "Hi World\nBaz Bar");
    }

    #[test]
    fn test_apply_fragment_with_description() {
        let content = "Hello World";
        let fragments = vec![Fragment {
            start_line: 1,
            start_col: 6,
            end_line: 1,
            end_col: 11,
            replacement: "Rust".to_string(),
            description: Some("Replace World with Rust".to_string()),
        }];
        let result = apply_fragments(content, &fragments).unwrap();
        assert_eq!(result, "Hello Rust");
    }

    #[test]
    fn test_map_selection_to_source_basic() {
        let rendered = "Hello World\nFoo Bar";
        let source = "Hello World\nFoo Bar";
        let result = map_selection_to_source(rendered, source, 0, 5);
        assert!(result.is_some());
    }

    #[test]
    fn test_map_selection_to_source_empty() {
        let result = map_selection_to_source("", "", 0, 5);
        assert!(result.is_none());
    }

    #[test]
    fn test_fragment_serialization() {
        let frag = Fragment {
            start_line: 1,
            start_col: 0,
            end_line: 1,
            end_col: 5,
            replacement: "test".to_string(),
            description: Some("a test".to_string()),
        };
        let json = serde_json::to_string(&frag).unwrap();
        let deserialized: Fragment = serde_json::from_str(&json).unwrap();
        assert_eq!(frag, deserialized);
    }
}
