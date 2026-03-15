//! # philiprehberger-diff-assert
//!
//! Better test assertion diffs with colored inline comparisons.
//!
//! Instead of the default `assert_eq!` output that dumps raw `Debug` representations,
//! `assert_eq_diff!` shows a colored line-by-line diff so you can immediately see
//! what changed.
//!
//! ## Quick start
//!
//! ```rust
//! use philiprehberger_diff_assert::assert_eq_diff;
//!
//! let expected = "hello\nworld";
//! let actual = "hello\nworld";
//! assert_eq_diff!(expected, actual);
//! ```
//!
//! ## Programmatic use
//!
//! ```rust
//! use philiprehberger_diff_assert::diff_strings;
//!
//! let diff = diff_strings("a\nb\nc", "a\nx\nc");
//! // Returns a formatted diff string (with ANSI color codes)
//! ```
//!
//! ## NO_COLOR support
//!
//! If the `NO_COLOR` environment variable is set, all color output is suppressed
//! automatically. You can also use [`diff_strings_no_color`] explicitly.

use std::fmt;

/// Represents a single line in a diff result.
#[derive(Debug, Clone, PartialEq, Eq)]
enum DiffLine<'a> {
    /// Line present in both inputs.
    Same(&'a str),
    /// Line present only in the left (removed).
    Removed(&'a str),
    /// Line present only in the right (added).
    Added(&'a str),
}

/// Compute the longest common subsequence table for two slices of lines.
///
/// Returns a 2D vector where `table[i][j]` is the LCS length of
/// `left[0..i]` and `right[0..j]`.
fn lcs_table<'a>(left: &[&'a str], right: &[&'a str]) -> Vec<Vec<usize>> {
    let m = left.len();
    let n = right.len();
    let mut table = vec![vec![0usize; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if left[i - 1] == right[j - 1] {
                table[i][j] = table[i - 1][j - 1] + 1;
            } else {
                table[i][j] = std::cmp::max(table[i - 1][j], table[i][j - 1]);
            }
        }
    }

    table
}

/// Compute a line-by-line diff between two string slices using LCS.
fn compute_diff<'a>(left: &'a str, right: &'a str) -> Vec<DiffLine<'a>> {
    let left_lines: Vec<&str> = left.lines().collect();
    let right_lines: Vec<&str> = right.lines().collect();

    let table = lcs_table(&left_lines, &right_lines);
    let mut result = Vec::new();

    let mut i = left_lines.len();
    let mut j = right_lines.len();

    // Backtrack through the LCS table to build the diff.
    while i > 0 || j > 0 {
        if i > 0 && j > 0 && left_lines[i - 1] == right_lines[j - 1] {
            result.push(DiffLine::Same(left_lines[i - 1]));
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || table[i][j - 1] >= table[i - 1][j]) {
            result.push(DiffLine::Added(right_lines[j - 1]));
            j -= 1;
        } else {
            result.push(DiffLine::Removed(left_lines[i - 1]));
            i -= 1;
        }
    }

    result.reverse();
    result
}

/// Format diff lines into a string with optional ANSI color codes.
fn format_diff(diff: &[DiffLine<'_>], use_color: bool) -> String {
    let mut output = String::new();

    for (idx, line) in diff.iter().enumerate() {
        if idx > 0 {
            output.push('\n');
        }
        match line {
            DiffLine::Same(text) => {
                output.push_str("  ");
                output.push_str(text);
            }
            DiffLine::Removed(text) => {
                if use_color {
                    output.push_str("\x1b[31m- ");
                    output.push_str(text);
                    output.push_str("\x1b[0m");
                } else {
                    output.push_str("- ");
                    output.push_str(text);
                }
            }
            DiffLine::Added(text) => {
                if use_color {
                    output.push_str("\x1b[32m+ ");
                    output.push_str(text);
                    output.push_str("\x1b[0m");
                } else {
                    output.push_str("+ ");
                    output.push_str(text);
                }
            }
        }
    }

    output
}

/// Returns `true` if colors should be used (i.e., `NO_COLOR` is not set).
fn should_use_color() -> bool {
    std::env::var("NO_COLOR").is_err()
}

/// Returns a formatted line-by-line diff of two strings with ANSI color codes.
///
/// Removed lines are shown in red with a `- ` prefix, added lines in green
/// with a `+ ` prefix, and unchanged lines with a `  ` prefix.
///
/// If the `NO_COLOR` environment variable is set, colors are omitted
/// (equivalent to calling [`diff_strings_no_color`]).
///
/// # Examples
///
/// ```
/// use philiprehberger_diff_assert::diff_strings;
///
/// let result = diff_strings("hello\nworld", "hello\neveryone");
/// // result contains a colored diff showing "world" removed and "everyone" added
/// ```
pub fn diff_strings(left: &str, right: &str) -> String {
    let diff = compute_diff(left, right);
    format_diff(&diff, should_use_color())
}

/// Returns a formatted line-by-line diff of two strings without ANSI color codes.
///
/// This always produces plain text output regardless of the `NO_COLOR` setting.
///
/// # Examples
///
/// ```
/// use philiprehberger_diff_assert::diff_strings_no_color;
///
/// let result = diff_strings_no_color("a\nb\nc", "a\nx\nc");
/// assert_eq!(result, "  a\n- b\n+ x\n  c");
/// ```
pub fn diff_strings_no_color(left: &str, right: &str) -> String {
    let diff = compute_diff(left, right);
    format_diff(&diff, false)
}

/// Returns a formatted diff of two values using their [`Debug`] representations.
///
/// Both values are formatted with `{:#?}` (pretty-printed Debug), then compared
/// line by line. Color output respects the `NO_COLOR` environment variable.
///
/// # Examples
///
/// ```
/// use philiprehberger_diff_assert::diff_debug;
///
/// let left = vec![1, 2, 3];
/// let right = vec![1, 4, 3];
/// let result = diff_debug(&left, &right);
/// // Shows a diff of the pretty-printed Debug output
/// ```
pub fn diff_debug<T: fmt::Debug>(left: &T, right: &T) -> String {
    let left_str = format!("{:#?}", left);
    let right_str = format!("{:#?}", right);
    diff_strings(&left_str, &right_str)
}

/// Asserts that two expressions are equal, showing a colored line-by-line diff on failure.
///
/// Works like [`assert_eq!`] but instead of dumping raw `Debug` output, it shows
/// a line-by-line diff with removed lines in red and added lines in green.
///
/// Both expressions must implement [`Debug`] and [`PartialEq`].
///
/// # Examples
///
/// ```
/// use philiprehberger_diff_assert::assert_eq_diff;
///
/// let a = "hello";
/// let b = "hello";
/// assert_eq_diff!(a, b);
/// ```
///
/// With a custom message:
///
/// ```should_panic
/// use philiprehberger_diff_assert::assert_eq_diff;
///
/// assert_eq_diff!(1, 2, "values should match: {}", "test");
/// ```
#[macro_export]
macro_rules! assert_eq_diff {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let left_str = format!("{:#?}", left_val);
                    let right_str = format!("{:#?}", right_val);
                    let diff = $crate::diff_strings(&left_str, &right_str);
                    panic!(
                        "assertion `left == right` failed\n\n--- left\n+++ right\n\n{}\n",
                        diff
                    );
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let left_str = format!("{:#?}", left_val);
                    let right_str = format!("{:#?}", right_val);
                    let diff = $crate::diff_strings(&left_str, &right_str);
                    panic!(
                        "assertion `left == right` failed: {}\n\n--- left\n+++ right\n\n{}\n",
                        format_args!($($arg)+),
                        diff
                    );
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_eq_diff_passes_when_equal() {
        assert_eq_diff!("hello", "hello");
    }

    #[test]
    fn assert_eq_diff_passes_with_equal_structs() {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        let a = Point { x: 1, y: 2 };
        let b = Point { x: 1, y: 2 };
        assert_eq_diff!(a, b);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn assert_eq_diff_panics_on_inequality() {
        assert_eq_diff!("hello", "world");
    }

    #[test]
    #[should_panic(expected = "custom error message")]
    fn assert_eq_diff_custom_message() {
        assert_eq_diff!(1, 2, "custom error message: {}", 42);
    }

    #[test]
    fn diff_strings_identical_returns_all_same() {
        let result = diff_strings_no_color("hello\nworld", "hello\nworld");
        assert_eq!(result, "  hello\n  world");
    }

    #[test]
    fn diff_strings_with_added_lines() {
        let result = diff_strings_no_color("a\nc", "a\nb\nc");
        assert_eq!(result, "  a\n+ b\n  c");
    }

    #[test]
    fn diff_strings_with_removed_lines() {
        let result = diff_strings_no_color("a\nb\nc", "a\nc");
        assert_eq!(result, "  a\n- b\n  c");
    }

    #[test]
    fn diff_strings_with_mixed_changes() {
        let result = diff_strings_no_color("a\nb\nc\nd", "a\nx\nc\ny");
        assert_eq!(result, "  a\n- b\n+ x\n  c\n- d\n+ y");
    }

    #[test]
    fn diff_strings_no_color_has_no_ansi_codes() {
        let result = diff_strings_no_color("hello", "world");
        assert!(!result.contains("\x1b["));
        assert!(result.contains("- hello"));
        assert!(result.contains("+ world"));
    }

    #[test]
    fn diff_debug_with_structs() {
        #[derive(Debug)]
        struct Config {
            name: String,
            value: i32,
        }

        let left = Config {
            name: "alpha".to_string(),
            value: 10,
        };
        let right = Config {
            name: "beta".to_string(),
            value: 10,
        };

        let result = diff_debug(&left, &right);
        // Should contain something about the name difference
        assert!(!result.is_empty());
    }

    #[test]
    fn diff_strings_empty_inputs() {
        let result = diff_strings_no_color("", "");
        assert_eq!(result, "");
    }

    #[test]
    fn diff_strings_left_empty() {
        let result = diff_strings_no_color("", "hello");
        // Empty string has one empty line, "hello" has one line
        assert!(result.contains("+ hello"));
    }
}
