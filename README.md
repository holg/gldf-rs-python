[![Rust](https://github.com/holg/gldf-rs-python/actions/workflows/rust.yml/badge.svg)](https://github.com/holg/gldf-rs-python/actions/workflows/rust.yml)
# gldf-rs-python

Python bindings and CLI for the GLDF (General Lighting Data Format) parser and writer for Rust.

## Features

- **Improved Error Handling**: All functions now use proper Result types instead of unwrap()
- **Trait-based Logging**: Implemented Logger trait for structured, configurable logging
- **CLI Interface**: Complete command-line tool with multiple conversion commands
- **Async Support**: Demonstrates async capabilities from the underlying gldf-rs library
- **Modern Rust Practices**: Uses traits, proper error handling, and comprehensive testing

## Installation

Install via pip:

```bash
pip install gldf-rs-python
```

## Python Usage

```python
import gldf_rs_python

# Convert GLDF file to XML (with improved error handling and logging)
xml = gldf_rs_python.gldf_to_xml('tests/data/test.gldf')

# Convert GLDF file to JSON (with improved error handling and logging)
json = gldf_rs_python.gldf_to_json('tests/data/test.gldf')

# Convert XML to JSON
xml2 = gldf_rs_python.xml_from_json(json)

# Convert JSON to XML
json2 = gldf_rs_python.json_from_xml_str(xml)

# Fetch content from URL and convert to JSON (demonstrates async capabilities)
json_from_url = gldf_rs_python.fetch_and_convert_to_json('https://example.com/data.xml')

# All functions now provide proper error messages instead of panics
xml == xml2  # => True
```

## CLI Usage

The package now includes a powerful command-line interface:

```bash
# Convert GLDF file to XML
gldf-cli to-xml input.gldf -o output.xml

# Convert GLDF file to JSON with verbose logging
gldf-cli -v to-json input.gldf -o output.json

# Convert XML to JSON
gldf-cli xml-to-json input.xml -o output.json

# Convert JSON to XML
gldf-cli json-to-xml input.json -o output.xml

# Fetch content from URL and convert (demonstrates async capabilities)
gldf-cli -v fetch-and-convert https://example.com/data.xml -o output.json

# Get help for any command
gldf-cli --help
gldf-cli to-xml --help
```

## Development

Create virtual environment and use maturin:

```bash
maturin develop
```

Run tests:

```bash
cargo test
```

Build CLI binary:

```bash
cargo build --bin gldf-cli
```

## Recent Improvements

### v0.2.3+ (Latest)
- **Replaced all unwrap() calls** with proper PyResult error handling
- **Implemented Logger trait** from gldf-rs for structured logging
- **Added complete CLI interface** with multiple conversion commands
- **Added async functionality** demonstrating underlying async capabilities
- **Comprehensive testing** including integration tests
- **Modern Rust patterns** with proper error handling and trait usage

### v0.2.3
- added new header definition, because FormatVersion changed
- added Logger to pass into some methods
- added test to completely check and compare to URL based gldf
- bumped up versions of dependencies

### v0.2.2
- added support meta-information.xml

### v0.2.1
- added better documentation fo the main page
- for wasm support some refactoring was needed, to use reqwest::blocking

### v0.2.0
- support for file types of url
- support for BOM encoded UTF8 product.xml

## License

GPL-3.0-or-later

## Contributing

WIP python module and binding for the gldf-rs rust library. The GitHub workflow ensures all wheels are built and published to PyPI.
