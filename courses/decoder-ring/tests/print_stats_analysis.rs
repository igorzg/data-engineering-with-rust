use decoder_ring::print_stats_analysis;

/// This function only prints; the test harness captures its stdout,
/// so the point is exercising every code path without panicking.
#[test]
fn does_not_panic_on_text() {
    print_stats_analysis("The quick brown fox jumps over the lazy dog");
}

#[test]
fn does_not_panic_on_empty_text() {
    print_stats_analysis("");
}
