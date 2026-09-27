use decoder_ring::decrypt;

#[test]
fn shifts_lowercase_forward() {
    assert_eq!(decrypt("abc", 1), "bcd");
}

#[test]
fn wraps_around_the_alphabet() {
    assert_eq!(decrypt("xyz", 1), "yza");
    assert_eq!(decrypt("z", 5), "e");
}

#[test]
fn preserves_case() {
    assert_eq!(decrypt("AbC", 1), "BcD");
}

#[test]
fn leaves_non_alphabetic_untouched() {
    assert_eq!(decrypt("a1! b 2", 3), "d1! e 2");
}

#[test]
fn known_vector() {
    assert_eq!(decrypt("Hello, World!", 5), "Mjqqt, Btwqi!");
}

#[test]
fn zero_shift_is_identity() {
    assert_eq!(decrypt("Hello, Rust!", 0), "Hello, Rust!");
}

#[test]
fn empty_string_is_empty() {
    assert_eq!(decrypt("", 7), "");
}

#[test]
fn round_trip_with_complement_shift() {
    let msg = "The quick brown fox jumps over the lazy dog!";
    for shift in 1..26u8 {
        let encrypted = decrypt(msg, shift);
        let decrypted = decrypt(&encrypted, (26 - shift) % 26);
        assert_eq!(decrypted, msg, "round trip failed for shift {shift}");
    }
}
