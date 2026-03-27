# rs-diff-assert

[![CI](https://github.com/philiprehberger/rs-diff-assert/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-diff-assert/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-diff-assert.svg)](https://crates.io/crates/philiprehberger-diff-assert)
[![GitHub release](https://img.shields.io/github/v/release/philiprehberger/rs-diff-assert)](https://github.com/philiprehberger/rs-diff-assert/releases)
[![Last updated](https://img.shields.io/github/last-commit/philiprehberger/rs-diff-assert)](https://github.com/philiprehberger/rs-diff-assert/commits/main)
[![License](https://img.shields.io/github/license/philiprehberger/rs-diff-assert)](LICENSE)
[![Bug Reports](https://img.shields.io/github/issues/philiprehberger/rs-diff-assert/bug)](https://github.com/philiprehberger/rs-diff-assert/issues?q=is%3Aissue+is%3Aopen+label%3Abug)
[![Feature Requests](https://img.shields.io/github/issues/philiprehberger/rs-diff-assert/enhancement)](https://github.com/philiprehberger/rs-diff-assert/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement)
[![Sponsor](https://img.shields.io/badge/sponsor-GitHub%20Sponsors-ec6cb9)](https://github.com/sponsors/philiprehberger)

Better test assertion diffs with colored inline comparisons

## Installation

```toml
[dependencies]
philiprehberger-diff-assert = "0.1.8"
```

## Usage

```rust
use philiprehberger_diff_assert::assert_eq_diff;

#[test]
fn my_test() {
    let expected = "hello\nworld\nfoo";
    let actual = "hello\neveryone\nfoo";

    // Shows a colored diff on failure instead of raw Debug output
    assert_eq_diff!(expected, actual);
}
```

Output on failure:
```
  hello
- world
+ everyone
  foo
```

### Programmatic use

```rust
use philiprehberger_diff_assert::diff_strings;

let diff = diff_strings("line1\nline2", "line1\nline3");
println!("{}", diff);
```

## API

| Function / Macro | Description |
|------------------|-------------|
| `assert_eq_diff!(left, right)` | Assert equality with colored diff on failure |
| `assert_eq_diff!(left, right, msg, ..)` | With custom failure message |
| `diff_strings(left, right)` | Get colored diff of two strings |
| `diff_strings_no_color(left, right)` | Get diff without ANSI codes |
| `diff_debug(left, right)` | Diff Debug output of two values |


## Development

```bash
cargo test
cargo clippy -- -D warnings
```

## Support

If you find this package useful, consider giving it a star on GitHub — it helps motivate continued maintenance and development.

[![LinkedIn](https://img.shields.io/badge/Philip%20Rehberger-LinkedIn-0A66C2?logo=linkedin)](https://www.linkedin.com/in/philiprehberger)
[![More packages](https://img.shields.io/badge/more-open%20source%20packages-blue)](https://philiprehberger.com/open-source-packages)

## License

[MIT](LICENSE)
