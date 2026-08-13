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
    if source.trim().is_empty() { return Err("Input is empty.".into()); }
    let value: serde_yaml::Value = serde_yaml::from_str(source).map_err(|e| format!("Invalid YAML: {e}"))?;
    serde_yaml::to_string(&value).map(|output| output.trim_end().to_string()).map_err(|e| format!("Unable to format YAML: {e}"))
}

pub fn format_toml(source: &str) -> Result<String, String> {
    if source.trim().is_empty() { return Err("Input is empty.".into()); }
    let value: toml::Value = source.parse().map_err(|e| format!("Invalid TOML: {e}"))?;
    toml::to_string_pretty(&value).map(|output| output.trim_end().to_string()).map_err(|e| format!("Unable to format TOML: {e}"))
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
    let mut quote = None; let mut escaped = false; let mut line_comment = false; let mut block_comment = false;
    let chars: Vec<char> = source.chars().collect(); let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        if line_comment { current.push(ch); if ch == '\n' { line_comment = false; write_line(&mut out, depth, current.trim()); current.clear(); } i += 1; continue; }
        if block_comment { current.push(ch); if ch == '*' && chars.get(i + 1) == Some(&'/') { current.push('/'); i += 1; block_comment = false; write_line(&mut out, depth, current.trim()); current.clear(); } i += 1; continue; }
        if let Some(q) = quote { current.push(ch); if escaped { escaped = false; } else if ch == '\\' { escaped = true; } else if ch == q { quote = None; } i += 1; continue; }
        if (ch == '\'' || ch == '"' || ch == '`') { quote = Some(ch); current.push(ch); i += 1; continue; }
        if ch == '/' && chars.get(i + 1) == Some(&'/') { current.push('/'); current.push('/'); i += 2; line_comment = true; continue; }
        if ch == '/' && chars.get(i + 1) == Some(&'*') { current.push('/'); current.push('*'); i += 2; block_comment = true; continue; }
        match ch {
            '{' => { if !current.trim().is_empty() { write_line(&mut out, depth, current.trim()); } current.clear(); write_line(&mut out, depth, "{"); depth += 1; }
            '}' => { if !current.trim().is_empty() { write_line(&mut out, depth, current.trim()); } current.clear(); depth = depth.saturating_sub(1); write_line(&mut out, depth, "}"); }
            ';' => { current.push(';'); write_line(&mut out, depth, current.trim()); current.clear(); }
            _ => current.push(ch),
        }
        i += 1;
    }
    if !current.trim().is_empty() { write_line(&mut out, depth, current.trim()); }
    Ok(out.trim_end().into())
}

fn replace_ci(input: &str, needle: &str, replacement: &str) -> String {
    let lower = input.to_ascii_lowercase(); let mut out = String::with_capacity(input.len()); let mut start = 0;
    while let Some(pos) = lower[start..].find(needle) { let absolute = start + pos; out.push_str(&input[start..absolute]); out.push_str(replacement); start = absolute + needle.len(); }
    out.push_str(&input[start..]); out
}

fn write_line(out: &mut String, depth: usize, value: &str) { let _ = writeln!(out, "{}{}", "  ".repeat(depth), value); }
