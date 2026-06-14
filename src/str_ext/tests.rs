use super::*;

// ---------------------------------------------------------------------------
// char pad — basic padding
// ---------------------------------------------------------------------------

#[test]
fn char_pads_short_string_with_zeros() {
    assert_eq!("123".pad_start(5, '0'), "00123");
}

#[test]
fn char_pads_with_space() {
    assert_eq!("hi".pad_start(5, ' '), "   hi");
}

#[test]
fn char_pads_with_custom_character() {
    assert_eq!("a".pad_start(3, '#'), "##a");
}

#[test]
fn char_pads_exactly_one_character() {
    assert_eq!("abc".pad_start(4, '0'), "0abc");
}

#[test]
fn char_empty_string_pads_to_target_length() {
    assert_eq!("".pad_start(5, 'x'), "xxxxx");
}

// ---------------------------------------------------------------------------
// char pad — no-op cases (already long enough / zero length)
// ---------------------------------------------------------------------------

#[test]
fn char_no_padding_when_length_equals_string_length() {
    assert_eq!("abc".pad_start(3, '0'), "abc");
}

#[test]
fn char_no_padding_when_length_less_than_string_length() {
    assert_eq!("hello".pad_start(3, '0'), "hello");
}

#[test]
fn char_length_zero_returns_original_string() {
    assert_eq!("hello".pad_start(0, '0'), "hello");
}

#[test]
fn char_empty_string_with_zero_length() {
    assert_eq!("".pad_start(0, '0'), "");
}

// ---------------------------------------------------------------------------
// char pad — unicode (length is counted in chars, not bytes)
// ---------------------------------------------------------------------------

#[test]
fn char_unicode_string_gets_padded() {
    assert_eq!("世".pad_start(4, '0'), "000世");
}

#[test]
fn char_emoji_string_gets_padded() {
    assert_eq!("🦀".pad_start(3, '0'), "00🦀");
}

#[test]
fn char_mixed_ascii_and_unicode_string() {
    assert_eq!("a世".pad_start(5, '0'), "000a世");
}

#[test]
fn char_unicode_pad_character() {
    assert_eq!("hi".pad_start(4, '★'), "★★hi");
}

#[test]
fn char_multibyte_pad_into_multibyte_string() {
    assert_eq!("世界".pad_start(4, '★'), "★★世界");
}

// ---------------------------------------------------------------------------
// &str pad — single-character slice behaves like a char pad
// ---------------------------------------------------------------------------

#[test]
fn str_single_char_slice_pads_like_char() {
    assert_eq!("123".pad_start(5, "0"), "00123");
}

#[test]
fn str_single_char_slice_with_space() {
    assert_eq!("hi".pad_start(5, " "), "   hi");
}

// ---------------------------------------------------------------------------
// &str pad — multi-character pattern is repeated and truncated
// ---------------------------------------------------------------------------

#[test]
fn str_pattern_repeats_and_truncates_to_gap() {
    // gap of 3 over pattern "ab" -> a, b, a
    assert_eq!("5".pad_start(4, "ab"), "aba5");
}

#[test]
fn str_pattern_fills_evenly() {
    // gap of 4 over pattern "ab" -> a, b, a, b
    assert_eq!("5".pad_start(5, "ab"), "abab5");
}

#[test]
fn str_pattern_truncated_mid_pattern() {
    // gap of 2 over pattern "xyz" -> x, y
    assert_eq!("5".pad_start(3, "xyz"), "xy5");
}

#[test]
fn str_pattern_shorter_than_gap_wraps_multiple_times() {
    // gap of 7 over pattern "ab" -> abababa
    assert_eq!("z".pad_start(8, "ab"), "abababaz");
}

#[test]
fn str_pattern_longer_than_gap_is_cut_off() {
    // gap of 2 over a long pattern -> only first two chars used
    assert_eq!("X".pad_start(3, "abcdef"), "abX");
}

#[test]
fn str_empty_string_padded_with_pattern() {
    assert_eq!("".pad_start(4, "ab"), "abab");
}

// ---------------------------------------------------------------------------
// &str pad — empty pad produces no padding (matches JS)
// ---------------------------------------------------------------------------

#[test]
fn str_empty_pad_does_not_pad() {
    assert_eq!("hi".pad_start(10, ""), "hi");
}

#[test]
fn str_empty_pad_on_empty_string() {
    assert_eq!("".pad_start(5, ""), "");
}

// ---------------------------------------------------------------------------
// &str pad — no-op cases
// ---------------------------------------------------------------------------

#[test]
fn str_no_padding_when_already_long_enough() {
    assert_eq!("hello".pad_start(3, "ab"), "hello");
}

#[test]
fn str_length_zero_returns_original_string() {
    assert_eq!("hello".pad_start(0, "ab"), "hello");
}

// ---------------------------------------------------------------------------
// &str pad — unicode patterns and targets
// ---------------------------------------------------------------------------

#[test]
fn str_unicode_pattern_repeats() {
    assert_eq!("x".pad_start(4, "★☆"), "★☆★x");
}

#[test]
fn str_unicode_pattern_into_unicode_string() {
    assert_eq!("世".pad_start(3, "★☆"), "★☆世");
}

#[test]
fn str_emoji_pattern_truncated_on_char_boundary() {
    // gap of 3 over "🦀🐍" -> 🦀, 🐍, 🦀 (multi-byte chars stay whole)
    assert_eq!("z".pad_start(4, "🦀🐍"), "🦀🐍🦀z");
}

// ---------------------------------------------------------------------------
// owned / borrowed String pad — same behaviour as &str
// ---------------------------------------------------------------------------

#[test]
fn ref_string_pad_repeats_and_truncates() {
    let pad = String::from("ab");
    assert_eq!("5".pad_start(4, &pad), "aba5");
}

#[test]
fn owned_string_pad_repeats_and_truncates() {
    assert_eq!("5".pad_start(4, String::from("ab")), "aba5");
}

#[test]
fn owned_string_pad_from_format() {
    // gap of 3 over pattern "12" -> 1, 2, 1
    assert_eq!("x".pad_start(4, format!("{}{}", 1, 2)), "121x");
}

#[test]
fn empty_owned_string_pad_does_not_pad() {
    assert_eq!("hi".pad_start(10, String::new()), "hi");
}

// ---------------------------------------------------------------------------
// Length invariants — result is always exactly `length` chars when padded
// ---------------------------------------------------------------------------

#[test]
fn char_result_length_matches_target() {
    assert_eq!("foo".pad_start(10, ' ').chars().count(), 10);
    assert_eq!("世".pad_start(2, '0').chars().count(), 2);
}

#[test]
fn str_result_length_matches_target() {
    assert_eq!("foo".pad_start(10, "ab").chars().count(), 10);
    assert_eq!("世".pad_start(6, "★☆").chars().count(), 6);
}

#[test]
fn char_large_padding_gap() {
    let result = "1".pad_start(100, '0');
    assert_eq!(result.chars().count(), 100);
    assert_eq!(result, format!("{:0>100}", "1"));
}

#[test]
fn str_large_padding_gap_repeats_pattern() {
    let result = "1".pad_start(100, "ab");
    assert_eq!(result.chars().count(), 100);
    assert!(result.ends_with('1'));
    assert!(result.starts_with("abab"));
}

// ===========================================================================
// pad_end
// ===========================================================================

// ---------------------------------------------------------------------------
// char pad — basic padding (appended after the string)
// ---------------------------------------------------------------------------

#[test]
fn pad_end_char_pads_short_string_with_zeros() {
    assert_eq!("123".pad_end(5, '0'), "12300");
}

#[test]
fn pad_end_char_pads_with_space() {
    assert_eq!("hi".pad_end(5, ' '), "hi   ");
}

#[test]
fn pad_end_char_pads_exactly_one_character() {
    assert_eq!("abc".pad_end(4, '0'), "abc0");
}

#[test]
fn pad_end_char_empty_string_pads_to_target_length() {
    assert_eq!("".pad_end(5, 'x'), "xxxxx");
}

// ---------------------------------------------------------------------------
// char pad — no-op cases
// ---------------------------------------------------------------------------

#[test]
fn pad_end_char_no_padding_when_length_equals_string_length() {
    assert_eq!("abc".pad_end(3, '0'), "abc");
}

#[test]
fn pad_end_char_no_padding_when_length_less_than_string_length() {
    assert_eq!("hello".pad_end(3, '0'), "hello");
}

#[test]
fn pad_end_char_length_zero_returns_original_string() {
    assert_eq!("hello".pad_end(0, '0'), "hello");
}

// ---------------------------------------------------------------------------
// char pad — unicode (length counted in chars, not bytes)
// ---------------------------------------------------------------------------

#[test]
fn pad_end_char_unicode_string_gets_padded() {
    assert_eq!("世".pad_end(4, '0'), "世000");
}

#[test]
fn pad_end_char_emoji_string_gets_padded() {
    assert_eq!("🦀".pad_end(3, '0'), "🦀00");
}

#[test]
fn pad_end_char_unicode_pad_character() {
    assert_eq!("hi".pad_end(4, '★'), "hi★★");
}

// ---------------------------------------------------------------------------
// &str pad — multi-character pattern is repeated and truncated
// ---------------------------------------------------------------------------

#[test]
fn pad_end_str_pattern_repeats_and_truncates_to_gap() {
    // gap of 3 over pattern "ab" -> a, b, a
    assert_eq!("5".pad_end(4, "ab"), "5aba");
}

#[test]
fn pad_end_str_pattern_fills_evenly() {
    assert_eq!("5".pad_end(5, "ab"), "5abab");
}

#[test]
fn pad_end_str_pattern_longer_than_gap_is_cut_off() {
    assert_eq!("X".pad_end(3, "abcdef"), "Xab");
}

#[test]
fn pad_end_str_empty_string_padded_with_pattern() {
    assert_eq!("".pad_end(4, "ab"), "abab");
}

// ---------------------------------------------------------------------------
// &str pad — empty pad produces no padding (matches JS)
// ---------------------------------------------------------------------------

#[test]
fn pad_end_str_empty_pad_does_not_pad() {
    assert_eq!("hi".pad_end(10, ""), "hi");
}

// ---------------------------------------------------------------------------
// owned / borrowed String pad
// ---------------------------------------------------------------------------

#[test]
fn pad_end_ref_string_pad_repeats_and_truncates() {
    let pad = String::from("ab");
    assert_eq!("5".pad_end(4, &pad), "5aba");
}

#[test]
fn pad_end_owned_string_pad_repeats_and_truncates() {
    assert_eq!("5".pad_end(4, String::from("ab")), "5aba");
}

// ---------------------------------------------------------------------------
// Length invariants & relationship to pad_start
// ---------------------------------------------------------------------------

#[test]
fn pad_end_result_length_matches_target() {
    assert_eq!("foo".pad_end(10, "ab").chars().count(), 10);
    assert_eq!("世".pad_end(6, "★☆").chars().count(), 6);
}

#[test]
fn pad_end_large_padding_gap() {
    let result = "1".pad_end(100, '0');
    assert_eq!(result.chars().count(), 100);
    assert_eq!(result, format!("{:0<100}", "1"));
}

#[test]
fn pad_end_is_reverse_of_pad_start() {
    // Padding the reversed string at the end, then reversing, equals pad_start.
    let reversed: String = "5".chars().rev().collect();
    let padded_end: String = reversed.pad_end(4, '0').chars().rev().collect();
    assert_eq!(padded_end, "5".pad_start(4, '0'));
}
