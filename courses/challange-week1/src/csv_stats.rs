//! Task 1: read a `CSV` file into a `Vec` and compute summary statistics
//! (min, max, mean) for one numeric column.
//!
//! The parser assumes a simple `CSV` dialect: a header row, comma-separated
//! fields, and no quoted fields with embedded commas.

use std::error::Error;
use std::fs;

/// Summary statistics for one numeric column.
#[derive(Debug)]
pub struct ColumnStats {
    /// Number of data rows parsed.
    pub count: usize,
    /// Smallest value in the column.
    pub min: f64,
    /// Largest value in the column.
    pub max: f64,
    /// Arithmetic mean of the column.
    pub mean: f64,
}

/// Parses `csv` text and computes the statistics of the column named `column`.
///
/// The column lookup is case-insensitive. Blank lines are skipped.
///
/// # Errors
///
/// Returns an error if the text has no header row, the column is missing
/// from the header, a data row has too few fields, a field does not parse
/// as `f64`, or no data rows remain.
pub fn parse_column(csv: &str, column: &str) -> Result<ColumnStats, String> {
    let mut lines = csv.lines();
    let header = lines
        .next()
        .ok_or_else(|| "`csv` text is empty (no header row)".to_string())?
        .split(',')
        .map(|field| field.trim().to_string())
        .collect::<Vec<_>>();

    let idx = header
        .iter()
        .position(|field| field.eq_ignore_ascii_case(column))
        .ok_or_else(|| format!("column `{column}` not found in header: {header:?}"))?;

    let mut values: Vec<f64> = Vec::new();
    for (offset, line) in lines.enumerate() {
        let line_no = offset + 2; // the header consumed above was line 1
        if line.trim().is_empty() {
            continue;
        }
        let field = line.split(',').nth(idx).ok_or_else(|| {
            format!(
                "line {line_no}: expected at least {} comma-separated fields",
                idx + 1
            )
        })?;
        let value: f64 = field.trim().parse().map_err(|_| {
            format!("line {line_no}: cannot parse `{field}` as a number (column `{column}`")
        })?;
        values.push(value);
    }

    if values.is_empty() {
        return Err(format!("no data rows found for column `{column}`"));
    }

    let min = values
        .iter()
        .fold(f64::INFINITY, |best, &value| f64::min(best, value));
    let max = values
        .iter()
        .fold(f64::NEG_INFINITY, |best, &value| f64::max(best, value));
    let sum: f64 = values.iter().sum();
    // The row count of any realistic CSV fits in an f64 mantissa.
    #[allow(clippy::cast_precision_loss)]
    let mean = sum / values.len() as f64;

    Ok(ColumnStats {
        count: values.len(),
        min,
        max,
        mean,
    })
}

/// Reads the `CSV` file at `path` and computes the statistics of `column`.
///
/// # Errors
///
/// Propagates file I/O errors and any error from [`parse_column`].
pub fn column_stats(path: &str, column: &str) -> Result<ColumnStats, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    parse_column(&text, column).map_err(|err| -> Box<dyn Error> { err.into() })
}
