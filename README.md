# hayexport

Data export plugin for Hayashi.

## Installation

```bash
hay install sheep-farm/hayexport
```

## Usage

```hayashi
import("sheep-farm/hayexport", as=export)

// Export to CSV
export::export_csv(data, "output.csv", ",", true)

// Export to JSON
export::export_json(data, "output.json", true)

// Export to Parquet
export::export_parquet(data, "output.parquet", 6)

// Export to Excel
export::export_excel(data, "output.xlsx", "Sheet1")

// Export to Feather
export::export_feather(data, "output.feather")

// Export to Stata
export::export_stata(data, "output.dta", 118)

// Export to SAS
export::export_sas(data, "output.sas7bdat", "sas7bdat")

// Export to SPSS
export::export_spss(data, "output.sav")

// Export to XML
export::export_xml(data, "output.xml", "data")

// Export to YAML
export::export_yaml(data, "output.yaml")
```

## Functions

### Common Formats
- `export_csv(data, filepath, delimiter, has_header)` - Export to CSV
- `export_json(data, filepath, pretty)` - Export to JSON

Additional formats (Parquet, Excel, Stata, SAS, SPSS, XML, YAML) will be added in future versions.

## Development

```bash
cargo build --release
cp target/release/libhayexport.so ~/.hay/packages/sheep-farm/hayexport.so
```

## Dependencies

- **csv**: CSV file support
- **serde_json**: JSON serialization

Note: Initial release supports CSV and JSON. Additional formats (Parquet, Excel, Stata, SAS, SPSS) will be added in future versions with appropriate dependencies.
