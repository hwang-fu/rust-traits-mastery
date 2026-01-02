# Rust Traits Mastery

A comprehensive, hands-on learning project for mastering Rust's 50+ most essential traits through progressive micro-projects.

## Project Structure

This is a Cargo workspace containing 24 library crates, each focusing on specific trait categories:

| Crate | Phase | Traits Covered |
|-------|-------|----------------|
| `p01_derive_basics` | 1 | Clone, Copy, Debug, Default |
| `p02_comparison_traits` | 1 | PartialEq, Eq, PartialOrd, Ord, Hash |
| `p03_conversion_from_into` | 2 | From, Into |
| `p04_conversion_try` | 2 | TryFrom, TryInto |
| `p05_conversion_asref` | 2 | AsRef, AsMut, FromStr, ToString |
| `p06_deref_magic` | 3 | Deref, DerefMut |
| `p07_drop_destructor` | 3 | Drop |
| `p08_borrow_toowned` | 3 | Borrow, BorrowMut, ToOwned |
| `p09_iterator_core` | 4 | Iterator |
| `p10_iterator_collect` | 4 | IntoIterator, FromIterator |
| `p11_iterator_advanced` | 4 | ExactSizeIterator, DoubleEndedIterator, Extend |
| `p12_closure_fn` | 5 | Fn |
| `p13_closure_fnmut_fnonce` | 5 | FnMut, FnOnce |
| `p14_operator_arithmetic` | 6 | Add, Sub, Mul, Div, Rem, Neg |
| `p15_operator_index` | 6 | Index, IndexMut |
| `p16_operator_bitwise` | 6 | BitAnd, BitOr, BitXor, Not, Shl, Shr |
| `p17_display_debug` | 7 | Display, Debug |
| `p18_error_handling` | 7 | Error |
| `p19_io_read_write` | 8 | Read, Write |
| `p20_io_bufread_seek` | 8 | BufRead, Seek |
| `p21_send_sync` | 9 | Send, Sync |
| `p22_sized_any` | 10 | Sized, ?Sized, Any |
| `p23_extend_sum` | 10 | Extend, Sum, Product |
| `p24_capstone` | 11 | Capstone: Expression Evaluator |

## Getting Started

```bash
# Check all crates compile
cargo check

# Run tests for all crates
cargo test

# Run tests for a specific crate
cargo test -p p01_derive_basics

# Build documentation
cargo doc --open
```

## Requirements

- Rust 1.70+ (tested with 1.92)
- Edition 2024

## License

MIT
