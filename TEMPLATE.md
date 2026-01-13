# Using This Template

This repository is a **GitHub template** for solving LeetCode problems using **Rust**.

The template starts with an empty structure so you can build everything from scratch.

## What You Get

The following folders are intentionally **empty**:

- `solutions/` – your Rust solutions
- `problems/` – generated problem descriptions
- `docs/` – generated documentation

They contain only placeholder files to preserve the directory structure.

## Required Updates

### 1. LeetCode Configuration

This template includes a pre-configured `leetcode.toml` file required by the tooling.

You must update the `cookies` section with your own values, and you may set your preferred text editor:

```toml
[cookies]
csrf = "<CSRFTOKEN>"
session = "<LEETCODE_SESSION>"
site = "leetcode.com"

[code]
editor = "vim"
```

Replace <CSRFTOKEN> and <LEETCODE_SESSION> with your own LeetCode cookies.
Change editor to the text editor you use.

No other fields need to be changed.

### 2. GitHub Pages (Optional)

If you enable GitHub Pages, update the following constant:

- File: tools/sync_leetcode.rs
- Constant: GITHUB_PAGES_URL

Set it to your own GitHub Pages URL.
If you do not use GitHub Pages, leave it empty.

## Notes

- No solutions are included by default.
- This template is intended for personal practice and learning.
- Add your solutions incrementally as you work on problems.
