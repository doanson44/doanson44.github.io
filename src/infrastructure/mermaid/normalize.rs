//! Mermaid source normaliser — narrow compatibility shim.
//!
//! Normalizes ambiguous unquoted flowchart node labels before passing source to
//! Mermaid.js. The original source is never mutated; normalization only affects
//! the local copy used by the renderer.

use std::borrow::Cow;

/// Return a Mermaid-safe version of `code` for passing to Mermaid.js.
///
/// Only flowchart/graph node labels that are known to be ambiguous are quoted:
/// slash-prefixed labels that are not parallelograms and labels containing
/// Mermaid syntax characters such as `{}`. Quoted labels, comments, edge labels,
/// unsafe labels, and non-flowchart diagrams are left unchanged.
pub fn normalize_mermaid_source(code: &str) -> String {
    if !is_flowchart(code) {
        return code.to_owned();
    }

    let mut output = String::with_capacity(code.len());
    let mut changed = false;
    let bytes = code.as_bytes();
    let mut line_start = 0usize;
    let mut i = 0usize;

    while i < bytes.len() {
        if bytes[i] != b'\r' && bytes[i] != b'\n' {
            i += 1;
            continue;
        }

        let line = &code[line_start..i];
        append_normalized_line(line, &mut output, &mut changed);

        let newline_end = if bytes[i] == b'\r' && i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
            i + 2
        } else {
            i + 1
        };
        output.push_str(&code[i..newline_end]);
        line_start = newline_end;
        i = newline_end;
    }

    if line_start < code.len() {
        append_normalized_line(&code[line_start..], &mut output, &mut changed);
    }

    if changed {
        output
    } else {
        code.to_owned()
    }
}

fn append_normalized_line(line: &str, output: &mut String, changed: &mut bool) {
    let normalized = if line.trim_start().starts_with("%%") {
        Cow::Borrowed(line)
    } else {
        normalize_line(line)
    };

    if normalized.as_ref() != line {
        *changed = true;
    }
    output.push_str(normalized.as_ref());
}

fn is_flowchart(code: &str) -> bool {
    for line in code.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with("%%") {
            continue;
        }
        let lower = t.to_ascii_lowercase();
        return is_keyword_token(&lower, "flowchart") || is_keyword_token(&lower, "graph");
    }
    false
}

fn is_keyword_token(s: &str, keyword: &str) -> bool {
    if !s.starts_with(keyword) {
        return false;
    }

    matches!(
        s.as_bytes().get(keyword.len()),
        None | Some(&b' ') | Some(&b'\t')
    )
}

fn normalize_line(line: &str) -> Cow<'_, str> {
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut flush = 0usize;
    let mut out = String::new();
    let mut i = 0usize;
    let mut in_quotes = false;
    let mut in_edge_label = false;

    while i + 1 < len {
        if in_quotes && bytes[i] == b'\\' {
            i = (i + 2).min(len);
            continue;
        }

        if bytes[i] == b'"' {
            in_quotes = !in_quotes;
            i += 1;
            continue;
        }

        if !in_quotes && bytes[i] == b'|' {
            in_edge_label = !in_edge_label;
            i += 1;
            continue;
        }

        if in_quotes || in_edge_label {
            i += 1;
            continue;
        }

        if bytes[i] != b'[' {
            i += 1;
            continue;
        }

        let content_start = i + 1;
        let mut j = content_start;
        let mut safe = true;

        while j < len {
            match bytes[j] {
                b']' => break,
                b'"' | b'\\' | b'[' => {
                    safe = false;
                    break;
                }
                _ => j += 1,
            }
        }

        if safe && j < len && bytes[j] == b']' {
            let content = &line[content_start..j];
            if needs_quoting(content) {
                out.push_str(&line[flush..i]);
                out.push_str("[\"");
                out.push_str(content);
                out.push_str("\"]");
                flush = j + 1;
            }
            i = j + 1;
        } else {
            i += 1;
        }
    }

    if flush == 0 {
        Cow::Borrowed(line)
    } else {
        out.push_str(&line[flush..]);
        Cow::Owned(out)
    }
}

fn needs_quoting(content: &str) -> bool {
    (content.len() >= 2 && content.starts_with('/') && !content.ends_with('/'))
        || content.contains('{')
        || content.contains('}')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotes_slash_prefix_label() {
        assert_eq!(
            normalize_line("  B[/harness-write-goal]"),
            "  B[\"/harness-write-goal\"]"
        );
    }

    #[test]
    fn quotes_route_parameter_label() {
        let line = "  I[GET /api/VisitOutlet/staffs/{staffId}/sp-qrcode<br/>?outletId=from step D]";
        assert_eq!(
            normalize_line(line),
            "  I[\"GET /api/VisitOutlet/staffs/{staffId}/sp-qrcode<br/>?outletId=from step D\"]"
        );
    }

    #[test]
    fn quotes_route_parameter_without_html() {
        let src = "flowchart TD\n  I[GET /staffs/{staffId}/qrcode]\n";
        assert_eq!(
            normalize_mermaid_source(src),
            "flowchart TD\n  I[\"GET /staffs/{staffId}/qrcode\"]\n"
        );
    }

    #[test]
    fn transforms_multiple_nodes_on_one_line() {
        assert_eq!(
            normalize_line("  B[/write-goal] --> C[/read-goal]"),
            "  B[\"/write-goal\"] --> C[\"/read-goal\"]"
        );
    }

    #[test]
    fn preserves_parallelogram() {
        let src = "flowchart TD\n  B[/Input Data/]\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }

    #[test]
    fn preserves_already_quoted_label() {
        let src = "flowchart TD\n  B[\"/Deploy\"]\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }

    #[test]
    fn preserves_decision_node() {
        let line = "  D{/harness-review-card?}";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn preserves_edge_label() {
        let line = r#"  A -->|[/harness-goal]| B[Normal]"#;
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn preserves_quoted_label() {
        let line = r#"  A["Example [/harness-goal] command"]"#;
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn preserves_unsafe_label() {
        let line = "  B[/foo\"bar]";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn non_flowchart_diagrams_unchanged() {
        let cases = [
            "sequenceDiagram\n  A->>B: /path\n  X[/fake]\n",
            "classDiagram\n  A <|-- B\n",
            "stateDiagram-v2\n  [*] --> S\n",
            "gantt\n  title G\n",
        ];

        for src in &cases {
            assert_eq!(normalize_mermaid_source(src), *src);
        }
    }

    #[test]
    fn preserves_crlf_line_endings() {
        let src = "flowchart TD\r\n  A[User intent]\r\n  B[/harness-goal]\r\n";
        let expected = "flowchart TD\r\n  A[User intent]\r\n  B[\"/harness-goal\"]\r\n";
        assert_eq!(normalize_mermaid_source(src), expected);
    }

    #[test]
    fn original_source_is_not_mutated() {
        let original = "flowchart TD\n  I[GET /staffs/{staffId}/qrcode]\n";
        let normalized = normalize_mermaid_source(original);
        assert!(original.contains("I[GET /staffs/{staffId}/qrcode]"));
        assert!(normalized.contains("I[\"GET /staffs/{staffId}/qrcode\"]"));
        assert_ne!(original, normalized.as_str());
    }

    #[test]
    fn detects_flowchart_and_graph_variants() {
        assert!(is_flowchart("flowchart TD\n  A-->B"));
        assert!(is_flowchart("flowchart LR\n  A-->B"));
        assert!(is_flowchart("FLOWCHART TD\n  A-->B"));
        assert!(is_flowchart("graph TD\n  A-->B"));
        assert!(is_flowchart("GRAPH LR\n  A-->B"));
    }

    #[test]
    fn flowchart_keyword_is_token_aware() {
        assert!(!is_flowchart("flowchartSomething\n  A-->B"));
        assert!(!is_flowchart("graphSomething\n  A-->B"));
        assert!(!is_flowchart("flowchartTD\n  A-->B"));
        assert!(!is_flowchart("graphLR\n  A-->B"));
    }
}
