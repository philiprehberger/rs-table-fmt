# rs-table-fmt

[![CI](https://github.com/philiprehberger/rs-table-fmt/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-table-fmt/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-table-fmt.svg)](https://crates.io/crates/philiprehberger-table-fmt)
[![GitHub release](https://img.shields.io/github/v/release/philiprehberger/rs-table-fmt)](https://github.com/philiprehberger/rs-table-fmt/releases)
[![Last updated](https://img.shields.io/github/last-commit/philiprehberger/rs-table-fmt)](https://github.com/philiprehberger/rs-table-fmt/commits/main)
[![License](https://img.shields.io/github/license/philiprehberger/rs-table-fmt)](LICENSE)
[![Bug Reports](https://img.shields.io/github/issues/philiprehberger/rs-table-fmt/bug)](https://github.com/philiprehberger/rs-table-fmt/issues?q=is%3Aissue+is%3Aopen+label%3Abug)
[![Feature Requests](https://img.shields.io/github/issues/philiprehberger/rs-table-fmt/enhancement)](https://github.com/philiprehberger/rs-table-fmt/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement)
[![Sponsor](https://img.shields.io/badge/sponsor-GitHub%20Sponsors-ec6cb9)](https://github.com/sponsors/philiprehberger)

Terminal table rendering with alignment, borders, Unicode support, and ANSI color awareness

## Installation

```toml
[dependencies]
philiprehberger-table-fmt = "0.1.1"
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

## Development

```bash
cargo test
cargo clippy -- -D warnings
```

## Support

If you find this package useful, consider giving it a star on GitHub — it helps motivate continued maintenance and development.

[![LinkedIn](https://img.shields.io/badge/Philip%20Rehberger-LinkedIn-0A66C2?logo=linkedin)](https://www.linkedin.com/in/philiprehberger)
[![More packages](https://img.shields.io/badge/more-open%20source%20packages-blue)](https://philiprehberger.com/open-source-packages)

## License

[MIT](LICENSE)
