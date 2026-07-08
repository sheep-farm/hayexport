use hayashi_plugin_sdk::{hayashi_fn, hayashi_plugin};
use std::fs::File;
use std::io::Write;
use csv::WriterBuilder;
use serde_json::{to_string_pretty, to_string};

hayashi_plugin!();

/// 1. export_csv(data, filepath, delimiter, has_header)
/// Export data to CSV file
/// data: data to export (JSON string representation of data)
/// filepath: output file path
/// delimiter: CSV delimiter (comma, semicolon, tab)
/// has_header: whether to include header row
#[hayashi_fn]
pub fn export_csv(data: String, filepath: String, delimiter: String, has_header: bool) -> bool {
    // Parse JSON data
    let parsed: serde_json::Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(_) => return false,
    };

    // Convert to CSV
    let mut wtr = match File::create(&filepath) {
        Ok(f) => WriterBuilder::new()
            .delimiter(delimiter.chars().next().unwrap_or(',') as u8)
            .has_headers(has_header)
            .from_writer(f),
        Err(_) => return false,
    };

    // Handle array of objects
    if let Some(arr) = parsed.as_array() {
        if let Some(first) = arr.first() {
            if let Some(obj) = first.as_object() {
                let headers: Vec<String> = obj.keys().cloned().collect();
                if has_header {
                    if wtr.write_record(&headers).is_err() {
                        return false;
                    }
                }
                for item in arr {
                    if let Some(obj) = item.as_object() {
                        let row: Vec<String> = headers.iter()
                            .map(|k| obj.get(k).and_then(|v| v.as_str()).unwrap_or("").to_string())
                            .collect();
                        if wtr.write_record(&row).is_err() {
                            return false;
                        }
                    }
                }
            }
        }
    }

    wtr.flush().is_ok()
}

/// 2. export_json(data, filepath, pretty)
/// Export data to JSON file
/// data: data to export (JSON string)
/// filepath: output file path
/// pretty: whether to format JSON with indentation
#[hayashi_fn]
pub fn export_json(data: String, filepath: String, pretty: bool) -> bool {
    // Parse and validate JSON
    let parsed: serde_json::Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(_) => return false,
    };

    // Serialize with or without pretty printing
    let json_str = if pretty {
        to_string_pretty(&parsed)
    } else {
        to_string(&parsed)
    };

    match json_str {
        Ok(s) => {
            match File::create(&filepath) {
                Ok(mut file) => file.write_all(s.as_bytes()).is_ok(),
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}
