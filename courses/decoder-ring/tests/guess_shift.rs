use decoder_ring::{decrypt, guess_shift};

const PLAIN: &str = "The quick brown fox jumps over the lazy dog";

#[test]
fn recovers_a_known_caesar_shift() {
    let cipher = decrypt(PLAIN, 7);
    let (depth, best_shift, decrypted, score) = guess_shift(&cipher, 26);
    assert_eq!(depth, 26);
    // decrypt shifts forward by 7, so the inverse shift is 19.
    assert_eq!(best_shift, 19);
    assert_eq!(decrypted, PLAIN);
    assert!(score > 0.0);
}

#[test]
fn returned_decrypted_matches_best_shift() {
    let cipher = decrypt("Rust is great", 3);
    let (_, best_shift, decrypted, _) = guess_shift(&cipher, 26);
    assert_eq!(decrypted, decrypt(&cipher, best_shift));
}

#[test]
fn zero_depth_returns_defaults() {
    let (depth, best_shift, decrypted, score) = guess_shift("abc", 0);
    assert_eq!(depth, 0);
    assert_eq!(best_shift, 0);
    assert!(decrypted.is_empty());
    assert!((score - 0.0_f32).abs() < 1e-3);
}

#[test]
fn best_shift_stays_within_depth() {
    let cipher = decrypt("Some random text to scramble around", 9);
    let (_, best_shift, _, _) = guess_shift(&cipher, 5);
    assert!(best_shift < 5);
}
