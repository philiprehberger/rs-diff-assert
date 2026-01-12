# rs-diff-assert

[![CI](https://github.com/philiprehberger/rs-diff-assert/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-diff-assert/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-diff-assert.svg)](https://crates.io/crates/philiprehberger-diff-assert)
[![Last updated](https://img.shields.io/github/last-commit/philiprehberger/rs-diff-assert)](https://github.com/philiprehberger/rs-diff-assert/commits/main)

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

If you find this project useful:

⭐ [Star the repo](https://github.com/philiprehberger/rs-diff-assert)

🐛 [Report issues](https://github.com/philiprehberger/rs-diff-assert/issues?q=is%3Aissue+is%3Aopen+label%3Abug)

💡 [Suggest features](https://github.com/philiprehberger/rs-diff-assert/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement)

❤️ [Sponsor development](https://github.com/sponsors/philiprehberger)

🌐 [All Open Source Projects](https://philiprehberger.com/open-source-packages)

💻 [GitHub Profile](https://github.com/philiprehberger)

🔗 [LinkedIn Profile](https://www.linkedin.com/in/philiprehberger)

## License

[MIT](LICENSE)
