# rs-table-fmt

[![CI](https://github.com/philiprehberger/rs-table-fmt/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-table-fmt/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-table-fmt.svg)](https://crates.io/crates/philiprehberger-table-fmt)
[![Last updated](https://img.shields.io/github/last-commit/philiprehberger/rs-table-fmt)](https://github.com/philiprehberger/rs-table-fmt/commits/main)

Terminal table rendering with alignment, borders, Unicode support, and ANSI color awareness

## Installation

```toml
[dependencies]
philiprehberger-table-fmt = "0.2.0"
```

## Usage

```rust
use philiprehberger_table_fmt::{Table, BorderStyle, Alignment};

let table = Table::new()
    .header(["Name", "Age", "City"])
    .row(["Alice", "30", "New York"])
    .row(["Bob", "25", "London"])
    .row(["Charlie", "35", "Tokyo"])
    .border(BorderStyle::Unicode)
    .align(1, Alignment::Right)
    .to_string();

println!("{}", table);
```

## API

| Function / Type | Description |
|----------------|-------------|
| `Table::new()` | Create a new table |
| `.header(cols)` | Set column headers |
| `.row(cols)` | Add a data row |
| `.align(col, alignment)` | Set column alignment |
| `.max_width(col, width)` | Limit column width |
| `.border(style)` | Set border style |
| `.to_string()` | Render as string |
| `.print()` | Render and print to stdout |
| `.to_markdown()` | Render as Markdown table |
| `.to_csv()` | Render as CSV |
| `.to_json()` | Render as JSON array of objects |

## Development

```bash
cargo test
cargo clippy -- -D warnings
```

## Support

If you find this project useful:

⭐ [Star the repo](https://github.com/philiprehberger/rs-table-fmt)

🐛 [Report issues](https://github.com/philiprehberger/rs-table-fmt/issues?q=is%3Aissue+is%3Aopen+label%3Abug)

💡 [Suggest features](https://github.com/philiprehberger/rs-table-fmt/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement)

❤️ [Sponsor development](https://github.com/sponsors/philiprehberger)

🌐 [All Open Source Projects](https://philiprehberger.com/open-source-packages)

💻 [GitHub Profile](https://github.com/philiprehberger)

🔗 [LinkedIn Profile](https://www.linkedin.com/in/philiprehberger)

## License

[MIT](LICENSE)
