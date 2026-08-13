use regex::RegexBuilder;
use std::fmt::Write;

pub fn regex_test(pattern: &str, text: &str) -> Result<String, String> {
    if pattern.is_empty() { return Err("Regex pattern is empty.".into()); }
    let regex = RegexBuilder::new(pattern).build().map_err(|e| format!("Invalid regex: {e}"))?;
    let matches: Vec<_> = regex.find_iter(text).collect(); let mut out = format!("{} match(es)\n", matches.len());
    for (i, m) in matches.iter().enumerate() { writeln!(&mut out, "{}. [{}..{}] {}", i + 1, m.start(), m.end(), m.as_str()).unwrap(); } Ok(out.trim_end().into())
}

pub fn color(input: &str) -> Result<String, String> {
    let hex = input.trim().trim_start_matches('#'); if hex.len() != 6 { return Err("Enter a six-digit hexadecimal color such as #0d6efd.".into()); }
    let r = u8::from_str_radix(&hex[0..2],16).map_err(|_| "Invalid hexadecimal color.")? as f64 / 255.0; let g = u8::from_str_radix(&hex[2..4],16).map_err(|_| "Invalid hexadecimal color.")? as f64 / 255.0; let b = u8::from_str_radix(&hex[4..6],16).map_err(|_| "Invalid hexadecimal color.")? as f64 / 255.0;
    let max=r.max(g).max(b); let min=r.min(g).min(b); let l=(max+min)/2.0; let d=max-min; let s=if d==0.0 {0.0} else {d/(1.0-(2.0*l-1.0).abs())}; let h=if d==0.0 {0.0} else if max==r {(60.0*((g-b)/d)).rem_euclid(360.0)} else if max==g {60.0*((b-r)/d+2.0)} else {60.0*((r-g)/d+4.0)};
    Ok(format!("HEX: #{hex}\nRGB: {}, {}, {}\nHSL: {:.1}°, {:.1}%, {:.1}%", (r*255.0).round(), (g*255.0).round(), (b*255.0).round(), h, s*100.0, l*100.0))
}

pub fn git(input: &str) -> Result<String, String> {
    let s = input.trim().to_ascii_lowercase(); let keep = s.contains("keep-changes=true");
    let command = if s.starts_with("undo-last-commit") { if keep { "git reset --soft HEAD~1" } else { "git reset --hard HEAD~1" } } else if s.starts_with("create-branch") { "git switch -c <branch-name>" } else if s.starts_with("delete-branch") { "git branch -d <branch-name>" } else if s.starts_with("update-branch") { "git pull --rebase" } else if s.starts_with("stash") { "git stash push -m \"work\"" } else if s.starts_with("unstash") { "git stash pop" } else if s.starts_with("revert") { "git revert <commit>" } else { return Err("Supported operations: undo-last-commit, create-branch, delete-branch, update-branch, stash, unstash, revert.".into()); };
    Ok(format!("{command}\n\nThis tool only generates the command; it never executes Git."))
}

pub fn gitignore(input: &str) -> Result<String, String> {
    let mut out = String::new(); let stacks = input.lines().map(|s| s.trim().to_ascii_lowercase()).collect::<Vec<_>>();
    if stacks.iter().any(|s| s=="rust") { out.push_str("/target/\nCargo.lock\n"); }
    if stacks.iter().any(|s| s=="node") { out.push_str("node_modules/\nnpm-debug.log*\n"); }
    if stacks.iter().any(|s| s=="vscode") { out.push_str(".vscode/*\n!.vscode/settings.json\n"); }
    if stacks.iter().any(|s| s=="docker") { out.push_str(".docker/\n"); }
    if stacks.iter().any(|s| s=="windows") { out.push_str("Thumbs.db\nDesktop.ini\n"); }
    if stacks.iter().any(|s| s=="macos") { out.push_str(".DS_Store\n"); }
    if stacks.iter().any(|s| s=="linux") { out.push_str("*~\n.nfs*\n"); }
    if stacks.iter().any(|s| s=="dotnet") || stacks.iter().any(|s| s==".net") { out.push_str("bin/\nobj/\n*.user\n"); }
    if out.is_empty() { return Err("Select at least one supported stack: Rust, Node, VSCode, Docker, Windows, macOS, Linux, .NET.".into()); } Ok(out.trim_end().into())
}

pub fn chmod(input: &str) -> Result<String, String> {
    let s=input.trim(); let value:u16=s.parse().map_err(|_| "Enter a three-digit numeric mode such as 755.")?; if value>777 || s.len()!=3 { return Err("Mode must be a three-digit value between 000 and 777.".into()); }
    let mut symbolic=String::new(); for digit in s.chars() { let n=digit.to_digit(8).ok_or("Invalid octal digit.")?; symbolic.push(if n&4!=0 {'r'} else {'-'}); symbolic.push(if n&2!=0 {'w'} else {'-'}); symbolic.push(if n&1!=0 {'x'} else {'-'}); }
    Ok(format!("Numeric: {s}\nSymbolic: {symbolic}"))
}

pub fn text_diff(left: &str, right: &str) -> Result<String, String> {
    let a=left.lines().collect::<Vec<_>>(); let b=right.lines().collect::<Vec<_>>(); let mut out=String::new(); let max=a.len().max(b.len());
    for i in 0..max { match (a.get(i),b.get(i)) { (Some(x),Some(y)) if x==y => writeln!(&mut out,"  {x}").unwrap(), (Some(x),Some(y)) => { writeln!(&mut out,"- {x}").unwrap(); writeln!(&mut out,"+ {y}").unwrap(); }, (Some(x),None)=>writeln!(&mut out,"- {x}").unwrap(), (None,Some(y))=>writeln!(&mut out,"+ {y}").unwrap(), _=>{} } } Ok(if out.is_empty(){"No differences.".into()}else{out.trim_end().into()})
}
