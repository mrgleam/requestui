use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use regex::Regex;
use serde_json::Value;

pub fn format_json_response<'a>(raw: &str) -> Text<'a> {
    // 1. Try to Pretty Print. If it's not valid JSON (e.g., HTML), fallback to raw text.
    let pretty = match serde_json::from_str::<Value>(raw) {
        Ok(val) => serde_json::to_string_pretty(&val).unwrap_or_else(|_| raw.to_string()),
        Err(_) => raw.to_string(),
    };

    // 2. Compile our syntax highlighter regex (we use lazy_static in production,
    // but compiling it once per response is fast enough for a TUI client)
    let re =
        Regex::new(r#"(".*?"\s*:)|(".*?")|(\b\d+\.?\d*\b)|(\b(?:true|false|null)\b)"#).unwrap();

    let mut lines = Vec::new();

    // 3. Iterate over the pretty string and apply Ratatui colors
    for line in pretty.lines() {
        let mut spans = Vec::new();
        let mut last_end = 0;

        for cap in re.captures_iter(line) {
            let m = cap.get(0).unwrap();

            // Add any uncolored text (like brackets {} []) before the match
            if m.start() > last_end {
                spans.push(Span::raw(line[last_end..m.start()].to_string()));
            }

            let text = m.as_str().to_string();

            // Apply color based on which regex group matched
            let style = if cap.get(1).is_some() {
                Style::default().fg(Color::LightBlue) // Keys
            } else if cap.get(2).is_some() {
                Style::default().fg(Color::LightGreen) // Strings
            } else if cap.get(3).is_some() {
                Style::default().fg(Color::LightYellow) // Numbers 
            } else if cap.get(4).is_some() {
                Style::default().fg(Color::LightMagenta) // Booleans/Null
            } else {
                Style::default()
            };

            spans.push(Span::styled(text, style));
            last_end = m.end();
        }

        // Add remaining text at the end of the line
        if last_end < line.len() {
            spans.push(Span::raw(line[last_end..].to_string()));
        }

        lines.push(Line::from(spans));
    }

    Text::from(lines)
}
