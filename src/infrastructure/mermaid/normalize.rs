//! Mermaid source normaliser — narrow compatibility shim.
//!
//! Fixes one specific Mermaid.js 11 parse failure: unquoted flowchart node
//! labels of the form `[/text]` (leading `/`, no trailing `/`) that Mermaid.js
//! misreads as a malformed parallelogram shape. The original source is never
//! mutated; this module only produces a locally-normalised copy for the JS call.

use std::borrow::Cow;

/// Return a Mermaid-safe version of `code` for passing to Mermaid.js.
///
/// Only ambiguous unquoted flowchart/graph node labels of the form `[/...]`
/// are normalized. Valid parallelograms, quoted labels, comments, edge labels,
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

    if changed { output } else { code.to_owned() }
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
    matches!(s.as_bytes().get(keyword.len()), None | Some(&b' ') | Some(&b'\t'))
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

        if bytes[i] != b'[' || bytes[i + 1] != b'/' {
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
    content.len() >= 2 && !content.ends_with('/')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn needs_quoting_yes_for_slash_prefix() {
        assert!(needs_quoting("/harness-write-goal"));
        assert!(needs_quoting("/harness-goal slug"));
        assert!(needs_quoting("/foo"));
        assert!(needs_quoting("/a/b"));
    }

    #[test]
    fn needs_quoting_no_for_parallelogram() {
        assert!(!needs_quoting("/Deploy/"));
        assert!(!needs_quoting("/Input Data/"));
        assert!(!needs_quoting("/x/"));
    }

    #[test]
    fn needs_quoting_no_for_lone_slash() {
        assert!(!needs_quoting("/"));
    }

    #[test]
    fn detects_flowchart_and_graph_variants() {
        assert!(is_flowchart("flowchart TD\n  A-->B"));
        assert!(is_flowchart("flowchart LR\n  A-->B"));
        assert!(is_flowchart("flowchart RL\n  A-->B"));
        assert!(is_flowchart("flowchart TB\n  A-->B"));
        assert!(is_flowchart("flowchart\n  A-->B"));
        assert!(is_flowchart("FLOWCHART TD\n  A-->B"));
        assert!(is_flowchart("graph TD\n  A-->B"));
        assert!(is_flowchart("graph LR\n  A-->B"));
        assert!(is_flowchart("graph\n  A-->B"));
        assert!(is_flowchart("GRAPH LR\n  A-->B"));
    }

    #[test]
    fn non_flowchart_diagrams_not_detected() {
        assert!(!is_flowchart("sequenceDiagram\n  A->>B: Hi"));
        assert!(!is_flowchart("classDiagram\n  A <|-- B"));
        assert!(!is_flowchart("stateDiagram-v2\n  [*] --> S"));
        assert!(!is_flowchart("gantt\n  title G"));
        assert!(!is_flowchart("erDiagram\n  A }|--|{ B : has"));
    }

    #[test]
    fn flowchart_keyword_is_token_aware() {
        assert!(!is_flowchart("flowchartSomething\n  A-->B"));
        assert!(!is_flowchart("graphSomething\n  A-->B"));
        assert!(!is_flowchart("flowchartTD\n  A-->B"));
        assert!(!is_flowchart("graphLR\n  A-->B"));
    }

    #[test]
    fn skips_blank_lines_and_comments_before_type() {
        assert!(is_flowchart("\n%% comment\nflowchart TD\n  A-->B"));
    }

    #[test]
    fn quotes_slash_prefix_label() {
        assert_eq!(normalize_line("  B[/harness-write-goal]"), "  B[\"/harness-write-goal\"]");
    }

    #[test]
    fn quotes_slash_prefix_label_with_space() {
        assert_eq!(normalize_line("  F[/harness-goal slug]"), "  F[\"/harness-goal slug\"]");
    }

    #[test]
    fn transforms_multiple_nodes_on_one_line() {
        assert_eq!(
            normalize_line("  B[/write-goal] --> C[/read-goal]"),
            "  B[\"/write-goal\"] --> C[\"/read-goal\"]"
        );
    }

    #[test]
    fn transforms_only_ambiguous_when_mixed_with_parallelogram() {
        assert_eq!(
            normalize_line("  A[/foo] --> B[/bar/]"),
            "  A[\"/foo\"] --> B[/bar/]"
        );
    }

    #[test]
    fn leaves_parallelogram_unchanged() {
        let line = "  B[/Deploy/]";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_already_quoted_label_unchanged() {
        let line = "  B[\"/Deploy\"]";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_label_with_embedded_quote_unchanged() {
        let line = "  B[/foo\"bar]";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_label_with_backslash_unchanged() {
        let line = "  B[/foo\\bar]";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_label_with_open_bracket_unchanged() {
        let line = "  B[/foo[bar]";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_ordinary_label_unchanged() {
        let line = "  A[User intent]";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_internal_slash_no_leading_slash_unchanged() {
        let line = "  J[Fix / log iteration]";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_decision_node_unchanged() {
        let line = "  D{/harness-review-card?}";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_edge_label_with_slash_unchanged() {
        let line = r#"  A -->|[/harness-goal]| B[Normal]"#;
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_url_in_edge_label_unchanged() {
        let line = "  A -->|https://example.com/path| B[Normal]";
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_quoted_label_containing_slash_bracket_unchanged() {
        let line = r#"  A["Example [/harness-goal] command"]"#;
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn leaves_quoted_with_slash_bracket_unchanged_alongside_real_node() {
        let line = r#"  A["desc [/harness] x"] --> B[/real-node]"#;
        assert_eq!(normalize_line(line), r#"  A["desc [/harness] x"] --> B["/real-node"]"#);
    }

    #[test]
    fn leaves_quoted_label_with_escaped_quote_unchanged() {
        let line = r#"  A["Example \"quoted\" [/harness-goal]"]"#;
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn does_not_normalize_after_escaped_quote_inside_label() {
        let line = r#"  A["Command \"test\" [/harness-goal]"]"#;
        assert_eq!(normalize_line(line), line);
    }

    #[test]
    fn preserves_slash_bracket_inside_edge_label() {
        let src = "flowchart TD\n  A -->|[/harness-goal]| B[Normal]\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }

    #[test]
    fn fixes_exact_failing_flowchart() {
        let src = concat!(
            "flowchart TD\n",
            "  A[User intent] --> B[/harness-write-goal]\n",
            "  B --> C[goals/slug.yaml]\n",
            "  C --> D{/harness-review-card?}\n",
            "  D -->|NO-GO| E[Fix goal YAML]\n",
            "  E --> D\n",
            "  D -->|GO or skip if low| F[/harness-goal slug]\n",
            "  F --> G[Plan + stay on branch]\n",
            "  G --> H[Implement]\n",
            "  H --> I[verify.ps1 slug]\n",
            "  I -->|exit != 0| J[Fix / log iteration]\n",
            "  J --> H\n",
            "  I -->|exit 0| K{risk medium/high?}\n",
            "  K -->|yes| L[harness-review]\n",
            "  L -->|REQUEST_CHANGES| H\n",
            "  L -->|APPROVE| M[Summarize branch + diff]\n",
            "  K -->|low| M\n",
            "  M --> N[STOP]\n",
            "  I -->|blocked| O[blocker.md + STOP]\n"
        );
        let normalized = normalize_mermaid_source(src);
        assert!(normalized.contains("B[\"/harness-write-goal\"]"));
        assert!(normalized.contains("F[\"/harness-goal slug\"]"));
        assert!(src.contains("B[/harness-write-goal]"));
        assert!(src.contains("F[/harness-goal slug]"));
        assert!(normalized.contains("A[User intent]"));
        assert!(normalized.contains("E[Fix goal YAML]"));
        assert!(normalized.contains("J[Fix / log iteration]"));
        assert!(normalized.contains("D{/harness-review-card?}"));
        assert!(normalized.contains("|NO-GO|"));
        assert!(normalized.contains("|exit != 0|"));
    }

    #[test]
    fn fixes_harness_write_goal() {
        let src = "flowchart TD\n  B[/harness-write-goal]\n";
        assert!(normalize_mermaid_source(src).contains("B[\"/harness-write-goal\"]"));
    }

    #[test]
    fn fixes_harness_goal_slug() {
        let src = "flowchart LR\n  F[/harness-goal slug]\n";
        assert!(normalize_mermaid_source(src).contains("F[\"/harness-goal slug\"]"));
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
    fn preserves_label_with_embedded_quote() {
        let src = "flowchart TD\n  B[/foo\"bar]\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }

    #[test]
    fn preserves_label_with_open_bracket() {
        let src = "flowchart TD\n  B[/foo[bar]\n";
        assert_eq!(normalize_mermaid_source(src), src);
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
    fn original_source_is_not_mutated() {
        let original = "flowchart TD\n  B[/harness-write-goal]\n";
        let normalized = normalize_mermaid_source(original);
        assert!(original.contains("B[/harness-write-goal]"));
        assert!(normalized.contains("B[\"/harness-write-goal\"]"));
        assert_ne!(original, normalized.as_str());
    }

    #[test]
    fn trailing_newline_preserved() {
        let src = "flowchart TD\n  A[/foo]\n";
        assert!(normalize_mermaid_source(src).ends_with('\n'));
    }

    #[test]
    fn no_trailing_newline_not_added() {
        let src = "flowchart TD\n  A[/foo]";
        assert!(!normalize_mermaid_source(src).ends_with('\n'));
    }

    #[test]
    fn preserves_crlf_line_endings() {
        let src = "flowchart TD\r\n  A[User intent]\r\n  B[/harness-goal]\r\n";
        let expected = "flowchart TD\r\n  A[User intent]\r\n  B[\"/harness-goal\"]\r\n";
        assert_eq!(normalize_mermaid_source(src), expected);
    }

    #[test]
    fn preserves_crlf_when_no_normalization_is_needed() {
        let src = "flowchart TD\r\n  A[User intent]\r\n  B[/Deploy/]\r\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }

    #[test]
    fn graph_keyword_detected() {
        let src = "graph TD\n  B[/harness-write-goal]\n";
        assert!(normalize_mermaid_source(src).contains("B[\"/harness-write-goal\"]"));
    }

    #[test]
    fn url_in_edge_label_unchanged() {
        let src = "flowchart TD\n  A -->|https://example.com/path| B[Normal]\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }

    #[test]
    fn leaves_comment_line_with_slash_bracket_unchanged() {
        let src = concat!(
            "flowchart TD\n",
            "  %% Example: [/harness-goal]\n",
            "  A --> B\n",
        );
        assert_eq!(normalize_mermaid_source(src), src);
    }

    #[test]
    fn leaves_indented_comment_unchanged() {
        let src = "flowchart TD\n    %% [/some-command] -- skip this\n  A --> B\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }

    #[test]
    fn flowchart_something_not_normalized() {
        let src = "flowchartSomething\n  B[/harness-write-goal]\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }

    #[test]
    fn graph_something_not_normalized() {
        let src = "graphSomething\n  B[/harness-write-goal]\n";
        assert_eq!(normalize_mermaid_source(src), src);
    }
}
