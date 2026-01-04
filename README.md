# LeetCode Solutions (Rust)

> This repository is documentation-focused.
> Individual problems are documented under the `problems/` directory.

![Rust](https://img.shields.io/badge/language-Rust-orange)

## Index

| # | Problem | Difficulty | Category |
|---|--------|------------|----------|
| 1 | [Two Sum](problems/001-two-sum.md) | Easy | algorithms |
| 2 | [Add Two Numbers](problems/002-add-two-numbers.md) | Medium | algorithms |
| 3 | [Longest Substring Without Repeating Characters](problems/003-longest-substring-without-repeating-characters.md) | Medium | algorithms |
| 4 | [Median Of Two Sorted Arrays](problems/004-median-of-two-sorted-arrays.md) | Hard | algorithms |

---

## Repository Structure

```text
leetcode-rs/
├── solutions/        # Rust solutions (source of truth)
├── problems/         # Auto-generated per-problem documentation
├── tools/            # Synchronization tool
├── src/              # Analysis-only crate root (rust-analyzer)
├── Cargo.toml
└── README.md
```

---

## Tooling

This repository uses a custom synchronization tool to keep solutions and documentation in sync.

### Usage

```bash
cd ~/leetcode-rs
rustc tools/sync_leetcode.rs -O -o tools/sync_leetcode
./tools/sync_leetcode
```

---

## Scope

- This repository focuses on **clear and correct solutions**, not execution.
- Solutions are written for learning and reference purposes.
- There is no runtime, benchmarking, or online judge integration.
- Documentation is auto-generated and kept in sync with source files.

---

## Notes

- `solutions/` is the single source of truth.
- Files under `problems/` are auto-generated.
- Manual edits to generated files will be overwritten.

---

## Credits

- https://github.com/clearloop/leetcode-cli
