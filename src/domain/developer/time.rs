pub fn timestamp(input: &str) -> Result<String, String> {
    let value: i64 = input
        .trim()
        .parse()
        .map_err(|_| "Enter a Unix timestamp in seconds.")?;
    let days = value.div_euclid(86_400);
    let seconds = value.rem_euclid(86_400);
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 }.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096).div_euclid(365);
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2).div_euclid(153);
    let d = doy - (153 * mp + 2).div_euclid(5) + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };
    if !(1..=9999).contains(&year) {
        return Err("Timestamp is outside the supported date range.".into());
    }
    Ok(format!(
        "UTC: {year:04}-{m:02}-{d:02} {:02}:{:02}:{:02}Z",
        seconds / 3600,
        (seconds % 3600) / 60,
        seconds % 60
    ))
}

pub fn cron(input: &str) -> Result<String, String> {
    let fields: Vec<_> = input.split_whitespace().collect();
    if fields.len() != 5 {
        return Err("Cron expression must contain exactly five fields.".into());
    }
    let names = ["minute", "hour", "day of month", "month", "day of week"];
    for (i, field) in fields.iter().enumerate() {
        if field.is_empty()
            || field
                .chars()
                .any(|c| !c.is_ascii_digit() && !"*/,-?".contains(c))
        {
            return Err(format!("Invalid {} field: {field}", names[i]));
        }
    }
    let minute = fields[0];
    let description = if minute == "*" {
        "Every minute".to_string()
    } else if let Some(step) = minute.strip_prefix("*/") {
        format!("Every {step} minutes")
    } else {
        format!("At minute {minute}")
    };
    Ok(format!("{description}\nSchedule: {}", input.trim()))
}
