# rs-table-fmt

Terminal table rendering with alignment, borders, Unicode support, and ANSI color awareness.

## Installation

```toml
[dependencies]
philiprehberger-table-fmt = "0.1"
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

Output:
```
┌─────────┬─────┬──────────┐
│ Name    │ Age │ City     │
├─────────┼─────┼──────────┤
│ Alice   │  30 │ New York │
│ Bob     │  25 │ London   │
│ Charlie │  35 │ Tokyo    │
└─────────┴─────┴──────────┘
```

### Border styles

- `BorderStyle::Ascii` — classic `+---+` borders
- `BorderStyle::Unicode` — box-drawing characters
- `BorderStyle::Rounded` — rounded corners
- `BorderStyle::Minimal` — header underline only
- `BorderStyle::None` — no borders

### Markdown output

```rust
let md = Table::new()
    .header(["Name", "Score"])
    .row(["Alice", "95"])
    .to_markdown();
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

## License

MIT
