//! Terminal table rendering with alignment, borders, Unicode support, and ANSI color awareness.
//!
//! `philiprehberger-table-fmt` provides a fluent builder API for constructing formatted tables
//! suitable for terminal output. It supports multiple border styles, per-column alignment,
//! Unicode-aware width calculation, and correct handling of ANSI color codes.
//!
//! # Example
//!
//! ```
//! use philiprehberger_table_fmt::{Table, BorderStyle, Alignment};
//!
//! let output = Table::new()
//!     .header(["Name", "Age", "City"])
//!     .row(["Alice", "30", "New York"])
//!     .row(["Bob", "25", "London"])
//!     .border(BorderStyle::Unicode)
//!     .align(1, Alignment::Right)
//!     .to_string();
//!
//! println!("{}", output);
//! ```

use std::collections::HashMap;
use std::fmt;
use unicode_width::UnicodeWidthStr;

/// Column alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// Left-align column content (default).
    Left,
    /// Right-align column content.
    Right,
    /// Center column content.
    Center,
}

/// Border style for the rendered table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    /// No borders at all.
    None,
    /// Classic ASCII borders: `+---+---+` and `|   |   |`.
    Ascii,
    /// Unicode box-drawing characters: `┌───┬───┐` and `│   │   │`.
    Unicode,
    /// Rounded Unicode corners: `╭───┬───╮` and `│   │   │`.
    Rounded,
    /// Minimal style: no outer borders, column separators and header underline only.
    Minimal,
}

#[derive(Debug, Clone)]
struct BorderChars {
    top_left: &'static str,
    top_right: &'static str,
    bottom_left: &'static str,
    bottom_right: &'static str,
    horizontal: &'static str,
    vertical: &'static str,
    cross: &'static str,
    top_tee: &'static str,
    bottom_tee: &'static str,
    left_tee: &'static str,
    right_tee: &'static str,
}

impl BorderStyle {
    fn chars(self) -> BorderChars {
        match self {
            BorderStyle::None => BorderChars {
                top_left: "",
                top_right: "",
                bottom_left: "",
                bottom_right: "",
                horizontal: "",
                vertical: "",
                cross: "",
                top_tee: "",
                bottom_tee: "",
                left_tee: "",
                right_tee: "",
            },
            BorderStyle::Ascii => BorderChars {
                top_left: "+",
                top_right: "+",
                bottom_left: "+",
                bottom_right: "+",
                horizontal: "-",
                vertical: "|",
                cross: "+",
                top_tee: "+",
                bottom_tee: "+",
                left_tee: "+",
                right_tee: "+",
            },
            BorderStyle::Unicode => BorderChars {
                top_left: "\u{250c}",
                top_right: "\u{2510}",
                bottom_left: "\u{2514}",
                bottom_right: "\u{2518}",
                horizontal: "\u{2500}",
                vertical: "\u{2502}",
                cross: "\u{253c}",
                top_tee: "\u{252c}",
                bottom_tee: "\u{2534}",
                left_tee: "\u{251c}",
                right_tee: "\u{2524}",
            },
            BorderStyle::Rounded => BorderChars {
                top_left: "\u{256d}",
                top_right: "\u{256e}",
                bottom_left: "\u{2570}",
                bottom_right: "\u{256f}",
                horizontal: "\u{2500}",
                vertical: "\u{2502}",
                cross: "\u{253c}",
                top_tee: "\u{252c}",
                bottom_tee: "\u{2534}",
                left_tee: "\u{251c}",
                right_tee: "\u{2524}",
            },
            BorderStyle::Minimal => BorderChars {
                top_left: "",
                top_right: "",
                bottom_left: "",
                bottom_right: "",
                horizontal: "-",
                vertical: "|",
                cross: "+",
                top_tee: "",
                bottom_tee: "",
                left_tee: "",
                right_tee: "",
            },
        }
    }
}

/// A terminal table with configurable headers, rows, alignment, borders, and column widths.
///
/// Use the fluent builder API to construct a table, then render it with
/// [`to_string()`](Table::to_string), [`print()`](Table::print),
/// [`to_markdown()`](Table::to_markdown), or [`to_csv()`](Table::to_csv).
pub struct Table {
    headers: Option<Vec<String>>,
    rows: Vec<Vec<String>>,
    alignments: HashMap<usize, Alignment>,
    max_widths: HashMap<usize, usize>,
    border: BorderStyle,
}

impl Table {
    /// Create a new empty table with ASCII border style.
    pub fn new() -> Self {
        Self {
            headers: None,
            rows: Vec::new(),
            alignments: HashMap::new(),
            max_widths: HashMap::new(),
            border: BorderStyle::Ascii,
        }
    }

    /// Set the column headers.
    pub fn header<I, S>(&mut self, cols: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.headers = Some(cols.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Add a single data row.
    pub fn row<I, S>(&mut self, cols: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.rows.push(cols.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Add multiple data rows at once.
    pub fn rows<I, R, S>(&mut self, rows: I) -> &mut Self
    where
        I: IntoIterator<Item = R>,
        R: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for row in rows {
            self.rows
                .push(row.into_iter().map(|s| s.into()).collect());
        }
        self
    }

    /// Set the alignment for a specific column (0-indexed).
    pub fn align(&mut self, col: usize, alignment: Alignment) -> &mut Self {
        self.alignments.insert(col, alignment);
        self
    }

    /// Set the maximum display width for a column. Content exceeding this width is truncated.
    pub fn max_width(&mut self, col: usize, width: usize) -> &mut Self {
        self.max_widths.insert(col, width);
        self
    }

    /// Set the border style for the table.
    pub fn border(&mut self, style: BorderStyle) -> &mut Self {
        self.border = style;
        self
    }

    /// Render the table and print it to stdout.
    pub fn print(&self) {
        print!("{}", self);
    }

    /// Render the table as a Markdown-formatted string.
    pub fn to_markdown(&self) -> String {
        let col_count = self.column_count();
        if col_count == 0 {
            return String::new();
        }
        let widths = self.compute_column_widths();
        let mut out = String::new();

        // Header row
        let header_cells: Vec<String> = if let Some(ref headers) = self.headers {
            (0..col_count)
                .map(|i| {
                    let cell = headers.get(i).map(|s| s.as_str()).unwrap_or("");
                    let cell = self.apply_max_width(i, cell);
                    pad_to_width(&cell, widths[i], Alignment::Left)
                })
                .collect()
        } else {
            (0..col_count)
                .map(|i| pad_to_width("", widths[i], Alignment::Left))
                .collect()
        };
        out.push_str("| ");
        out.push_str(&header_cells.join(" | "));
        out.push_str(" |\n");

        // Separator row with alignment markers
        let sep_cells: Vec<String> = (0..col_count)
            .map(|i| {
                let align = self.alignments.get(&i).copied().unwrap_or(Alignment::Left);
                let w = widths[i];
                match align {
                    Alignment::Left => format!(":{}", "-".repeat(w.saturating_sub(1).max(1))),
                    Alignment::Right => format!("{}:", "-".repeat(w.saturating_sub(1).max(1))),
                    Alignment::Center => {
                        format!(":{}:", "-".repeat(w.saturating_sub(2).max(1)))
                    }
                }
            })
            .collect();
        out.push_str("| ");
        out.push_str(&sep_cells.join(" | "));
        out.push_str(" |\n");

        // Data rows
        for row in &self.rows {
            let cells: Vec<String> = (0..col_count)
                .map(|i| {
                    let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");
                    let cell = self.apply_max_width(i, cell);
                    pad_to_width(&cell, widths[i], Alignment::Left)
                })
                .collect();
            out.push_str("| ");
            out.push_str(&cells.join(" | "));
            out.push_str(" |\n");
        }

        out
    }

    /// Render the table as a CSV string.
    pub fn to_csv(&self) -> String {
        let col_count = self.column_count();
        if col_count == 0 {
            return String::new();
        }
        let mut out = String::new();

        if let Some(ref headers) = self.headers {
            let cells: Vec<String> = (0..col_count)
                .map(|i| {
                    let cell = headers.get(i).map(|s| s.as_str()).unwrap_or("");
                    csv_escape(cell)
                })
                .collect();
            out.push_str(&cells.join(","));
            out.push('\n');
        }

        for row in &self.rows {
            let cells: Vec<String> = (0..col_count)
                .map(|i| {
                    let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");
                    csv_escape(cell)
                })
                .collect();
            out.push_str(&cells.join(","));
            out.push('\n');
        }

        out
    }

    fn column_count(&self) -> usize {
        let header_count = self.headers.as_ref().map(|h| h.len()).unwrap_or(0);
        let row_max = self.rows.iter().map(|r| r.len()).max().unwrap_or(0);
        header_count.max(row_max)
    }

    fn compute_column_widths(&self) -> Vec<usize> {
        let col_count = self.column_count();
        let mut widths = vec![0usize; col_count];

        if let Some(ref headers) = self.headers {
            for (i, h) in headers.iter().enumerate() {
                widths[i] = widths[i].max(visible_width(h));
            }
        }

        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                widths[i] = widths[i].max(visible_width(cell));
            }
        }

        // Apply max_width caps
        for (&col, &max_w) in &self.max_widths {
            if col < col_count {
                widths[col] = widths[col].min(max_w);
            }
        }

        // Ensure minimum width of 1
        for w in &mut widths {
            if *w == 0 {
                *w = 1;
            }
        }

        widths
    }

    fn apply_max_width(&self, col: usize, s: &str) -> String {
        if let Some(&max_w) = self.max_widths.get(&col) {
            truncate_to_width(s, max_w)
        } else {
            s.to_string()
        }
    }

    fn render(&self) -> String {
        let col_count = self.column_count();
        if col_count == 0 {
            return String::new();
        }

        let widths = self.compute_column_widths();
        let bc = self.border.chars();
        let mut out = String::new();

        match self.border {
            BorderStyle::None => {
                // Render rows without any borders
                if let Some(ref headers) = self.headers {
                    let cells: Vec<String> = (0..col_count)
                        .map(|i| {
                            let cell = headers.get(i).map(|s| s.as_str()).unwrap_or("");
                            let cell = self.apply_max_width(i, cell);
                            let align =
                                self.alignments.get(&i).copied().unwrap_or(Alignment::Left);
                            pad_to_width(&cell, widths[i], align)
                        })
                        .collect();
                    out.push_str(&cells.join("  "));
                    out.push('\n');
                }
                for row in &self.rows {
                    let cells: Vec<String> = (0..col_count)
                        .map(|i| {
                            let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");
                            let cell = self.apply_max_width(i, cell);
                            let align =
                                self.alignments.get(&i).copied().unwrap_or(Alignment::Left);
                            pad_to_width(&cell, widths[i], align)
                        })
                        .collect();
                    out.push_str(&cells.join("  "));
                    out.push('\n');
                }
            }
            BorderStyle::Minimal => {
                // No outer borders, column separators and header underline
                if let Some(ref headers) = self.headers {
                    let cells: Vec<String> = (0..col_count)
                        .map(|i| {
                            let cell = headers.get(i).map(|s| s.as_str()).unwrap_or("");
                            let cell = self.apply_max_width(i, cell);
                            let align =
                                self.alignments.get(&i).copied().unwrap_or(Alignment::Left);
                            pad_to_width(&cell, widths[i], align)
                        })
                        .collect();
                    out.push_str(&format!(
                        " {} \n",
                        cells.join(&format!(" {} ", bc.vertical))
                    ));

                    // Header underline
                    let seps: Vec<String> =
                        widths.iter().map(|w| bc.horizontal.repeat(*w)).collect();
                    out.push_str(&format!(
                        "-{}-\n",
                        seps.join(&format!("-{}-", bc.cross))
                    ));
                }
                for row in &self.rows {
                    let cells: Vec<String> = (0..col_count)
                        .map(|i| {
                            let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");
                            let cell = self.apply_max_width(i, cell);
                            let align =
                                self.alignments.get(&i).copied().unwrap_or(Alignment::Left);
                            pad_to_width(&cell, widths[i], align)
                        })
                        .collect();
                    out.push_str(&format!(
                        " {} \n",
                        cells.join(&format!(" {} ", bc.vertical))
                    ));
                }
            }
            _ => {
                // Full border styles: Ascii, Unicode, Rounded

                // Top border
                let top_segments: Vec<String> = widths
                    .iter()
                    .map(|w| bc.horizontal.repeat(w + 2))
                    .collect();
                out.push_str(&format!(
                    "{}{}{}\n",
                    bc.top_left,
                    top_segments.join(bc.top_tee),
                    bc.top_right
                ));

                // Header row
                if let Some(ref headers) = self.headers {
                    let cells: Vec<String> = (0..col_count)
                        .map(|i| {
                            let cell = headers.get(i).map(|s| s.as_str()).unwrap_or("");
                            let cell = self.apply_max_width(i, cell);
                            let align =
                                self.alignments.get(&i).copied().unwrap_or(Alignment::Left);
                            pad_to_width(&cell, widths[i], align)
                        })
                        .collect();
                    out.push_str(&format!(
                        "{} {} {}\n",
                        bc.vertical,
                        cells.join(&format!(" {} ", bc.vertical)),
                        bc.vertical
                    ));

                    // Header separator
                    let sep_segments: Vec<String> = widths
                        .iter()
                        .map(|w| bc.horizontal.repeat(w + 2))
                        .collect();
                    out.push_str(&format!(
                        "{}{}{}\n",
                        bc.left_tee,
                        sep_segments.join(bc.cross),
                        bc.right_tee
                    ));
                }

                // Data rows
                for row in &self.rows {
                    let cells: Vec<String> = (0..col_count)
                        .map(|i| {
                            let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");
                            let cell = self.apply_max_width(i, cell);
                            let align =
                                self.alignments.get(&i).copied().unwrap_or(Alignment::Left);
                            pad_to_width(&cell, widths[i], align)
                        })
                        .collect();
                    out.push_str(&format!(
                        "{} {} {}\n",
                        bc.vertical,
                        cells.join(&format!(" {} ", bc.vertical)),
                        bc.vertical
                    ));
                }

                // Bottom border
                let bottom_segments: Vec<String> = widths
                    .iter()
                    .map(|w| bc.horizontal.repeat(w + 2))
                    .collect();
                out.push_str(&format!(
                    "{}{}{}\n",
                    bc.bottom_left,
                    bottom_segments.join(bc.bottom_tee),
                    bc.bottom_right
                ));
            }
        }

        out
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Calculate the visible display width of a string, ignoring ANSI escape codes
/// and accounting for Unicode character widths (e.g., CJK characters taking 2 columns).
pub fn visible_width(s: &str) -> usize {
    let stripped = strip_ansi(s);
    UnicodeWidthStr::width(stripped.as_str())
}

/// Strip ANSI escape sequences from a string.
fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Look for CSI sequence: ESC [ ... final_byte
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['
                // Consume until we find a letter (the final byte of the sequence)
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
            // Also handle ESC followed by other things; just skip the ESC
        } else {
            result.push(c);
        }
    }
    result
}

/// Truncate a string to a maximum visible width, appending "\u{2026}" if truncated.
/// Handles ANSI codes correctly (does not count them toward width, does not break mid-escape).
pub fn truncate_to_width(s: &str, max: usize) -> String {
    if max == 0 {
        return String::new();
    }
    let vis = visible_width(s);
    if vis <= max {
        return s.to_string();
    }

    // We need to truncate to (max - 1) visible chars, then append ellipsis
    let target = max.saturating_sub(1);
    let mut result = String::new();
    let mut current_width = 0usize;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Pass through ANSI escape sequence
            result.push(c);
            if chars.peek() == Some(&'[') {
                result.push(chars.next().unwrap());
                while let Some(&next) = chars.peek() {
                    result.push(chars.next().unwrap());
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            let cw = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
            if current_width + cw > target {
                break;
            }
            result.push(c);
            current_width += cw;
        }
    }

    result.push('\u{2026}');
    result
}

/// Pad a string to a target display width using the given alignment.
/// Uses visible_width to account for ANSI codes and Unicode.
pub fn pad_to_width(s: &str, width: usize, alignment: Alignment) -> String {
    let vis = visible_width(s);
    if vis >= width {
        return s.to_string();
    }
    let padding = width - vis;
    match alignment {
        Alignment::Left => format!("{}{}", s, " ".repeat(padding)),
        Alignment::Right => format!("{}{}", " ".repeat(padding), s),
        Alignment::Center => {
            let left = padding / 2;
            let right = padding - left;
            format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
        }
    }
}

/// Escape a value for CSV output. Wraps in quotes if needed.
fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_table_with_headers() {
        let output = Table::new()
            .header(["Name", "Age"])
            .row(["Alice", "30"])
            .row(["Bob", "25"])
            .border(BorderStyle::Ascii)
            .to_string();

        assert!(output.contains("Name"));
        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));
        assert!(output.contains("+"));
        assert!(output.contains("|"));
    }

    #[test]
    fn test_ascii_border_style() {
        let output = Table::new()
            .header(["A", "B"])
            .row(["1", "2"])
            .border(BorderStyle::Ascii)
            .to_string();

        assert!(output.contains("+---"));
        assert!(output.contains("| A"));
    }

    #[test]
    fn test_unicode_border_style() {
        let output = Table::new()
            .header(["A", "B"])
            .row(["1", "2"])
            .border(BorderStyle::Unicode)
            .to_string();

        assert!(output.contains("\u{250c}")); // top-left ┌
        assert!(output.contains("\u{2518}")); // bottom-right ┘
        assert!(output.contains("\u{2502}")); // vertical │
        assert!(output.contains("\u{2500}")); // horizontal ─
    }

    #[test]
    fn test_rounded_border_style() {
        let output = Table::new()
            .header(["X"])
            .row(["Y"])
            .border(BorderStyle::Rounded)
            .to_string();

        assert!(output.contains("\u{256d}")); // top-left ╭
        assert!(output.contains("\u{256f}")); // bottom-right ╯
    }

    #[test]
    fn test_minimal_border_style() {
        let output = Table::new()
            .header(["A", "B"])
            .row(["1", "2"])
            .border(BorderStyle::Minimal)
            .to_string();

        // Should have separators but no box corners
        assert!(output.contains("|"));
        assert!(output.contains("-"));
        assert!(!output.contains("+---+---+"));
    }

    #[test]
    fn test_no_border_style() {
        let output = Table::new()
            .header(["A", "B"])
            .row(["1", "2"])
            .border(BorderStyle::None)
            .to_string();

        assert!(!output.contains("|"));
        assert!(!output.contains("+"));
        assert!(output.contains("A"));
        assert!(output.contains("1"));
    }

    #[test]
    fn test_left_alignment() {
        let output = Table::new()
            .header(["Name"])
            .row(["Hi"])
            .border(BorderStyle::Ascii)
            .align(0, Alignment::Left)
            .to_string();

        // "Name" is 4 chars, "Hi" is 2 chars, so "Hi" should be left-padded
        assert!(output.contains("| Hi   |") || output.contains("| Name |"));
    }

    #[test]
    fn test_right_alignment() {
        let output = Table::new()
            .header(["Num"])
            .row(["1"])
            .row(["200"])
            .border(BorderStyle::Ascii)
            .align(0, Alignment::Right)
            .to_string();

        assert!(output.contains("|   1 |"));
        assert!(output.contains("| 200 |"));
    }

    #[test]
    fn test_center_alignment() {
        let output = Table::new()
            .header(["Title"])
            .row(["Hi"])
            .border(BorderStyle::Ascii)
            .align(0, Alignment::Center)
            .to_string();

        // "Title" = 5, "Hi" = 2, padding = 3, left=1, right=2
        assert!(output.contains(" Hi  ") || output.contains("  Hi "));
    }

    #[test]
    fn test_max_width_truncation() {
        let output = Table::new()
            .header(["Name"])
            .row(["Alexander"])
            .border(BorderStyle::Ascii)
            .max_width(0, 5)
            .to_string();

        // "Alexander" should be truncated to 4 chars + ellipsis
        assert!(output.contains("Alex\u{2026}"));
        assert!(!output.contains("Alexander"));
    }

    #[test]
    fn test_visible_width_strips_ansi() {
        let s = "\x1b[31mred\x1b[0m";
        assert_eq!(visible_width(s), 3);
    }

    #[test]
    fn test_visible_width_plain_text() {
        assert_eq!(visible_width("hello"), 5);
        assert_eq!(visible_width(""), 0);
    }

    #[test]
    fn test_visible_width_cjk() {
        // CJK characters take 2 columns
        assert_eq!(visible_width("\u{4f60}\u{597d}"), 4); // 你好
    }

    #[test]
    fn test_unicode_characters_in_table() {
        let output = Table::new()
            .header(["Name"])
            .row(["\u{4f60}\u{597d}"]) // 你好
            .border(BorderStyle::Ascii)
            .to_string();

        assert!(output.contains("\u{4f60}\u{597d}"));
    }

    #[test]
    fn test_empty_table() {
        let output = Table::new().to_string();
        assert_eq!(output, "");
    }

    #[test]
    fn test_single_column() {
        let output = Table::new()
            .header(["X"])
            .row(["1"])
            .border(BorderStyle::Ascii)
            .to_string();

        assert!(output.contains("| X |"));
        assert!(output.contains("| 1 |"));
    }

    #[test]
    fn test_single_row() {
        let output = Table::new()
            .row(["only", "row"])
            .border(BorderStyle::Ascii)
            .to_string();

        assert!(output.contains("only"));
        assert!(output.contains("row"));
    }

    #[test]
    fn test_to_markdown() {
        let md = Table::new()
            .header(["Name", "Score"])
            .row(["Alice", "95"])
            .row(["Bob", "88"])
            .to_markdown();

        assert!(md.contains("| Name"));
        assert!(md.contains("| Alice"));
        assert!(md.contains(":"));
        assert!(md.contains("---"));
    }

    #[test]
    fn test_to_markdown_with_alignment() {
        let md = Table::new()
            .header(["Name", "Score"])
            .row(["Alice", "95"])
            .align(0, Alignment::Left)
            .align(1, Alignment::Right)
            .to_markdown();

        let lines: Vec<&str> = md.lines().collect();
        let sep_line = lines[1];
        // Left-aligned column should start with ':'
        // Right-aligned column should end with ':'
        assert!(sep_line.contains(":---"));
        assert!(sep_line.contains("---:"));
    }

    #[test]
    fn test_to_csv() {
        let csv = Table::new()
            .header(["Name", "City"])
            .row(["Alice", "New York"])
            .row(["Bob", "London"])
            .to_csv();

        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines[0], "Name,City");
        assert_eq!(lines[1], "Alice,New York");
    }

    #[test]
    fn test_to_csv_with_special_chars() {
        let csv = Table::new()
            .header(["Name", "Note"])
            .row(["Alice", "has, comma"])
            .row(["Bob", "has \"quotes\""])
            .to_csv();

        assert!(csv.contains("\"has, comma\""));
        assert!(csv.contains("\"has \"\"quotes\"\"\""));
    }

    #[test]
    fn test_no_headers_data_only() {
        let output = Table::new()
            .row(["A", "B"])
            .row(["C", "D"])
            .border(BorderStyle::Ascii)
            .to_string();

        assert!(output.contains("| A | B |"));
        assert!(output.contains("| C | D |"));
        // Should not have header separator (only top and bottom borders)
        let line_count = output.lines().count();
        // top border + 2 data rows + bottom border = 4
        assert_eq!(line_count, 4);
    }

    #[test]
    fn test_mismatched_row_lengths() {
        let output = Table::new()
            .header(["A", "B", "C"])
            .row(["1"]) // shorter than header
            .row(["x", "y", "z"])
            .border(BorderStyle::Ascii)
            .to_string();

        // Short row should be padded with empty cells
        assert!(output.contains("| 1 |"));
    }

    #[test]
    fn test_ansi_in_table_alignment() {
        let output = Table::new()
            .header(["Color"])
            .row(["\x1b[31mred\x1b[0m"])
            .row(["green"])
            .border(BorderStyle::Ascii)
            .to_string();

        // "green" is 5 chars, "\x1b[31mred\x1b[0m" is 3 visible chars
        // Column width should be 5 (from "green" and "Color")
        assert!(output.contains("\x1b[31mred\x1b[0m"));
        assert!(output.contains("green"));
    }

    #[test]
    fn test_truncate_to_width() {
        assert_eq!(truncate_to_width("hello world", 5), "hell\u{2026}");
        assert_eq!(truncate_to_width("hi", 5), "hi");
        assert_eq!(truncate_to_width("hello", 5), "hello");
        assert_eq!(truncate_to_width("", 5), "");
    }

    #[test]
    fn test_truncate_preserves_ansi() {
        let s = "\x1b[31mhello world\x1b[0m";
        let result = truncate_to_width(s, 5);
        // Should truncate to 4 visible chars + ellipsis, preserving ANSI
        assert!(result.contains("\x1b[31m"));
        assert!(result.contains("\u{2026}"));
        assert_eq!(visible_width(&strip_ansi(&result.replace('\u{2026}', "x"))), 5);
    }

    #[test]
    fn test_pad_to_width() {
        assert_eq!(pad_to_width("hi", 5, Alignment::Left), "hi   ");
        assert_eq!(pad_to_width("hi", 5, Alignment::Right), "   hi");
        assert_eq!(pad_to_width("hi", 5, Alignment::Center), " hi  ");
        assert_eq!(pad_to_width("hello", 5, Alignment::Left), "hello");
    }

    #[test]
    fn test_multiple_rows_method() {
        let output = Table::new()
            .header(["A", "B"])
            .rows(vec![vec!["1", "2"], vec!["3", "4"]])
            .border(BorderStyle::Ascii)
            .to_string();

        assert!(output.contains("| 1 | 2 |"));
        assert!(output.contains("| 3 | 4 |"));
    }

    #[test]
    fn test_display_trait() {
        let table = Table::new()
            .header(["X"])
            .row(["Y"])
            .border(BorderStyle::Ascii)
            .to_string();

        let display = format!(
            "{}",
            Table::new()
                .header(["X"])
                .row(["Y"])
                .border(BorderStyle::Ascii)
        );

        assert_eq!(table, display);
    }

    #[test]
    fn test_default_trait() {
        let table = Table::default();
        assert_eq!(table.to_string(), "");
    }
}
