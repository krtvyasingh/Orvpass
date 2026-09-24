use std::collections::HashMap;

pub fn unescape_csv_field(field: &str) -> String {
    let trimmed = field.trim();
    if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
        trimmed[1..trimmed.len() - 1].replace("\"\"", "\"")
    } else {
        trimmed.to_string()
    }
}

pub fn detect_csv_delimiter(content: &str) -> char {
    let comma_count = content.chars().take(500).filter(|c| *c == ',').count();
    let semi_count = content.chars().take(500).filter(|c| *c == ';').count();
    let tab_count = content.chars().take(500).filter(|c| *c == '\t').count();
    if tab_count > comma_count && tab_count > semi_count {
        '\t'
    } else if semi_count > comma_count {
        ';'
    } else {
        ','
    }
}

pub fn parse_csv_records(content: &str) -> Vec<HashMap<String, String>> {
    let mut lines = content.lines();
    let header_line = match lines.next() {
        Some(h) => h,
        None => return Vec::new(),
    };
    let delim = detect_csv_delimiter(content);
    let headers: Vec<String> = header_line
        .split(delim)
        .map(|h| unescape_csv_field(h).to_lowercase())
        .collect();

    let mut records = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let fields: Vec<String> = line.split(delim).map(unescape_csv_field).collect();
        let mut map = HashMap::new();
        for (idx, header) in headers.iter().enumerate() {
            if let Some(val) = fields.get(idx) {
                map.insert(header.clone(), val.clone());
            }
        }
        records.push(map);
    }
    records
}
