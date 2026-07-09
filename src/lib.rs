#![allow(clippy::not_unsafe_ptr_arg_deref)]
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
                if has_header && wtr.write_record(&headers).is_err() {
                    return false;
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

/// export_json(data, filepath, pretty)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // #[hayashi_fn] renomeia a fn para __hayashi_impl_*; chamamos diretamente.

    #[test]
    fn test_export_json_compact() {
        let path = "/tmp/hayexport_test_compact.json".to_string();
        let data = r#"[{"a":1,"b":"hello"}]"#.to_string();
        let ok = __hayashi_impl_export_json(data, path.clone(), false);
        assert!(ok, "export_json deve retornar true");
        let content = fs::read_to_string(&path).unwrap();
        // JSON compacto não tem newlines extras
        assert!(content.contains("\"a\""));
        assert!(content.contains("\"hello\""));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_export_json_pretty() {
        let path = "/tmp/hayexport_test_pretty.json".to_string();
        let data = r#"{"x":42,"y":3.14}"#.to_string();
        let ok = __hayashi_impl_export_json(data, path.clone(), true);
        assert!(ok, "export_json pretty deve retornar true");
        let content = fs::read_to_string(&path).unwrap();
        // Pretty print tem pelo menos uma quebra de linha
        assert!(content.contains('\n'), "esperado JSON indentado");
        assert!(content.contains("42"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_export_json_invalid_data() {
        let path = "/tmp/hayexport_test_invalid.json".to_string();
        let ok = __hayashi_impl_export_json("not json".to_string(), path.clone(), false);
        assert!(!ok, "JSON inválido deve retornar false");
    }

    #[test]
    fn test_export_csv_basic() {
        let path = "/tmp/hayexport_test.csv".to_string();
        let data = r#"[{"name":"Alice","age":"30"},{"name":"Bob","age":"25"}]"#.to_string();
        let ok = __hayashi_impl_export_csv(data, path.clone(), ",".to_string(), true);
        assert!(ok, "export_csv deve retornar true");
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("Alice"));
        assert!(content.contains("Bob"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn test_export_csv_invalid_path() {
        let ok = __hayashi_impl_export_csv(
            r#"[{"a":"1"}]"#.to_string(),
            "/nonexistent_dir/file.csv".to_string(),
            ",".to_string(),
            true,
        );
        assert!(!ok, "caminho inválido deve retornar false");
    }

    #[test]
    fn test_export_csv_invalid_data() {
        let ok = __hayashi_impl_export_csv(
            "not json".to_string(),
            "/tmp/hayexport_bad.csv".to_string(),
            ",".to_string(),
            true,
        );
        assert!(!ok, "JSON inválido deve retornar false");
    }
}
