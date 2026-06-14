/// Implementation detail that seals [`PadWith`] against outside implementations.
///
/// Because `Sealed` lives in a private module, no crate other than this one can
/// name it, and therefore none can implement [`PadWith`]. This lets the set of
/// valid pad types — and the signature of [`PadWith::pad_chars`] — evolve
/// without it being a breaking change.
mod sealed {
    pub trait Sealed {}

    impl Sealed for char {}
    impl Sealed for &str {}
    impl Sealed for &String {}
    impl Sealed for String {}
}

/// A value that can be used to pad a string.
///
/// Implemented for [`char`] (a single repeated character) and for string types
/// (`&str`, `&String`, `String`), whose characters are repeated and truncated
/// to fill the required width. This mirrors the role of the `padString`
/// argument to JavaScript's [`String.prototype.padStart`]; see
/// [`JsStrExt::pad_start`] for the full padding semantics.
///
/// This trait is sealed: it cannot be implemented for types outside this crate.
///
/// [`String.prototype.padStart`]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/padStart
pub trait PadWith: sealed::Sealed {
    /// Returns an iterator over the padding characters.
    ///
    /// A non-empty pad yields an infinite, repeating sequence; an empty string
    /// yields nothing. Callers take only as many characters as the gap
    /// requires, so the unbounded length is intentional and never fully
    /// consumed.
    fn pad_chars(&self) -> impl Iterator<Item = char> + '_;
}

impl PadWith for char {
    fn pad_chars(&self) -> impl Iterator<Item = char> + '_ {
        std::iter::repeat(*self)
    }
}

impl PadWith for &str {
    fn pad_chars(&self) -> impl Iterator<Item = char> + '_ {
        self.chars().cycle()
    }
}

impl PadWith for &String {
    fn pad_chars(&self) -> impl Iterator<Item = char> + '_ {
        self.chars().cycle()
    }
}

impl PadWith for String {
    fn pad_chars(&self) -> impl Iterator<Item = char> + '_ {
        self.chars().cycle()
    }
}

/// Extension trait adding JavaScript-style helpers to string slices.
pub trait JsStrExt {
    /// Pads the start of the string with `pad` until it is at least `length`
    /// [`char`]s long.
    ///
    /// `pad` may be a single [`char`] or a `&str`. A multi-character `&str` is
    /// repeated and truncated to exactly fill the space inserted before the
    /// original string; for example, padding `"5"` to width 4 with `"ab"`
    /// produces `"aba5"`.
    ///
    /// The string is returned unchanged when it is already at least `length`
    /// [`char`]s long, or when `pad` is an empty `&str`.
    ///
    /// # Length is counted in `char`s
    ///
    /// `length` is a number of Unicode scalar values ([`char`]s). This differs
    /// from JavaScript's [`String.prototype.padStart`], which counts UTF-16
    /// code units: a character outside the Basic Multilingual Plane such as
    /// `'🦀'` counts as 1 here but as 2 in JavaScript.
    ///
    /// # Examples
    ///
    /// ```
    /// use js_ergo::JsStrExt;
    ///
    /// // Pad with a single character.
    /// assert_eq!("123".pad_start(5, '0'), "00123");
    ///
    /// // Pad with a repeating, truncated pattern.
    /// assert_eq!("5".pad_start(4, "ab"), "aba5");
    ///
    /// // Already long enough: returned unchanged.
    /// assert_eq!("hello".pad_start(3, '.'), "hello");
    /// ```
    ///
    /// [`String.prototype.padStart`]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/padStart
    fn pad_start<P: PadWith>(&self, length: usize, pad: P) -> String;
}

impl JsStrExt for str {
    fn pad_start<P: PadWith>(&self, length: usize, pad: P) -> String {
        let len = self.chars().count();

        if len >= length {
            return self.to_string();
        }

        let gap = length - len;

        let mut result = String::with_capacity(self.len() + gap);
        result.extend(pad.pad_chars().take(gap));
        result.push_str(self);
        result
    }
}

#[cfg(test)]
mod tests;
