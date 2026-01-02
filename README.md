# LeetCode Solutions (Rust)

| # | Problem | Difficulty | Category |
|---|--------|------------|----------|
| 1 | [Two Sum](problems/001-two-sum.md) | Easy | algorithms |
| 2 | [Add Two Numbers](problems/002-add-two-numbers.md) | Medium | algorithms |

---

## Tooling

This repository uses a custom synchronization tool to keep solutions and documentation consistent.

### Usage

```bash
cd ~/leetcode-rs
rustc tools/sync_leetcode.rs -O -o tools/sync_leetcode
./tools/sync_leetcode
```

### Credits

- https://github.com/clearloop/leetcode-cli

---

## Source of Truth

- `solutions/*.rs`
- `solutions/*.tests.dat`

Generated files should not be edited manually.
