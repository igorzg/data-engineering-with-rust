//! Task 1 tests: CSV column statistics.

use challange_week1::csv_stats::parse_column;

const SAMPLE: &str = "\
date,region,revenue,units
2026-09-01,north,119.88,12
2026-09-02,south,240.50,20

2026-09-03,east,10.00,2
";

#[test]
#[allow(clippy::float_cmp)] // parsed text and literals are the identical f64 bits
fn computes_min_max_and_mean() {
    let stats = parse_column(SAMPLE, "revenue").unwrap();
    assert_eq!(stats.count, 3);
    assert_eq!(stats.min, 10.0);
    assert_eq!(stats.max, 240.5);
    assert!((stats.mean - (119.88 + 240.5 + 10.0) / 3.0).abs() < f64::EPSILON);
}

#[test]
fn column_lookup_is_case_insensitive() {
    assert!(parse_column(SAMPLE, "Revenue").is_ok());
}

#[test]
#[allow(clippy::float_cmp)] // 12 + 20 + 2 parses exactly to 34.0; both sides divide by 3.0 identically
fn skips_blank_lines() {
    let stats = parse_column(SAMPLE, "units").unwrap();
    assert_eq!(stats.count, 3);
    assert_eq!(stats.mean, 34.0 / 3.0);
}

#[test]
fn unknown_column_is_an_error() {
    assert!(parse_column(SAMPLE, "nope").unwrap_err().contains("nope"));
}

#[test]
fn malformed_number_reports_the_file_line() {
    let err = parse_column("a\nx\n", "a").unwrap_err();
    assert!(err.contains("line 2"), "got: {err}");
}

#[test]
fn empty_input_is_an_error() {
    assert!(parse_column("", "a").is_err());
}
