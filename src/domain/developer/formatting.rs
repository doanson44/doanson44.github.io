use std::fmt::Write;

pub fn format_markup(source: &str) -> Result<String, String> {
    let source = source.trim();
    if source.is_empty() { return Err("Input is empty.".into()); }
    let mut out = String::new();
    let mut depth = 0usize;
    let mut token = String::new();
    for ch in source.chars() {
        if ch == '<' {
            if !token.trim().is_empty() { write_line(&mut out, depth, token.trim()); }
            token.clear(); token.push(ch);
        } else if ch == '>' {
            token.push(ch);
            let t = token.trim();
            if t.starts_with("</") { depth = depth.saturating_sub(1); }
            write_line(&mut out, depth, t);
            if !t.starts_with("</") && !t.starts_with("<?") && !t.starts_with("<!") && !t.ends_with("/>") && !t.contains("</") { depth += 1; }
            token.clear();
        } else { token.push(ch); }
    }
    if !token.trim().is_empty() { write_line(&mut out, depth, token.trim()); }
    Ok(out.trim_end().into())
}

pub fn format_yaml(source: &str) -> Result<String, String> {
    let mut out = String::new();
    for raw in source.lines() {
        let line = raw.trim();
        if line.is_empty() { continue; }
        let indent = raw.chars().take_while(|c| c.is_whitespace()).count();
        write_line(&mut out, indent / 2, line.replace('\t', "  ").trim());
    }
    if out.is_empty() { Err("Input is empty.".into()) } else { Ok(out.trim_end().into()) }
}

pub fn format_toml(source: &str) -> Result<String, String> {
    let source = source.trim();
    if source.is_empty() { return Err("Input is empty.".into()); }
    let mut out = String::new();
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        if line.starts_with('[') && line.ends_with(']') && !out.is_empty() { out.push('\n'); }
        out.push_str(line); out.push('\n');
    }
    Ok(out.trim_end().into())
}

pub fn format_sql(source: &str) -> Result<String, String> {
    let mut s = source.split_whitespace().collect::<Vec<_>>().join(" ");
    for keyword in ["SELECT", "FROM", "WHERE", "GROUP BY", "ORDER BY", "HAVING", "LIMIT", "VALUES", "SET", "JOIN", "LEFT JOIN", "RIGHT JOIN", "INNER JOIN"] {
        s = replace_ci(&s, &keyword.to_ascii_lowercase(), &format!("\n{keyword}"));
    }
    s = s.replace(", ", ",\n  ");
    if s.trim().is_empty() { Err("Input is empty.".into()) } else { Ok(s.trim().into()) }
}

pub fn format_braced(source: &str) -> Result<String, String> {
    if source.trim().is_empty() { return Err("Input is empty.".into()); }
    let mut out = String::new(); let mut depth = 0usize; let mut current = String::new();
    for ch in source.chars() {
        match ch {
            '{' => { if !current.trim().is_empty() { write_line(&mut out, depth, current.trim()); } current.clear(); write_line(&mut out, depth, "{"); depth += 1; }
            '}' => { if !current.trim().is_empty() { write_line(&mut out, depth, current.trim()); } current.clear(); depth = depth.saturating_sub(1); write_line(&mut out, depth, "}"); }
            ';' => { current.push(';'); write_line(&mut out, depth, current.trim()); current.clear(); }
            _ => current.push(ch),
        }
    }
    if !current.trim().is_empty() { write_line(&mut out, depth, current.trim()); }
    Ok(out.trim_end().into())
}

fn replace_ci(input: &str, needle: &str, replacement: &str) -> String {
    let lower = input.to_ascii_lowercase();
    let mut out = String::with_capacity(input.len()); let mut start = 0;
    while let Some(pos) = lower[start..].find(needle) {
        let absolute = start + pos;
        out.push_str(&input[start..absolute]); out.push_str(replacement); start = absolute + needle.len();
    }
    out.push_str(&input[start..]); out
}

fn write_line(out: &mut String, depth: usize, value: &str) {
    let _ = writeln!(out, "{}{}", "  ".repeat(depth), value);
}
