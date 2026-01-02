# LeetCode Solutions (Rust)

| # | Problem | Difficulty | Category |
|---|--------|------------|----------|
| 1 | [Two Sum](problems/001-two-sum.md) | Easy | algorithms |
| 2 | [Add Two Numbers](problems/002-add-two-numbers.md) | Medium | algorithms |

---

## Tooling

This repository uses a custom synchronization tool to keep binaries and documentation consistent.

### Tools Used

- **Rust** (stable toolchain)
- **Cargo**
- **rustc**
- Custom synchronization tool: `tools/sync_leetcode.rs`
- LeetCode CLI: https://github.com/clearloop/leetcode-cli

### Tool Responsibilities

- Register all `solutions/<no>.<slug>.rs` as Cargo binaries
- Generate per-problem README under `problems/`
- Generate this index README
- Remove orphan documentation files (hard sync)

---

## How to Compile & Run the Tool

```bash
cd ~/leetcode
rustc tools/sync_leetcode.rs -O -o tools/sync_leetcode
./tools/sync_leetcode
```

> Safe to run repeatedly. All outputs are deterministic.

---

## Credits

Problem statements and test data were generated using:

- https://github.com/clearloop/leetcode-cli

---

## Source of Truth

- `solutions/*.rs`
- `solutions/*.tests.dat`

Manual edits to generated README files will be overwritten.
