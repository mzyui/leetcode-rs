use std::{
    collections::HashSet,
    fs,
    io,
    path::PathBuf,
};

struct Problem {
    number: u32,
    slug: String,
    title: String,
    category: String,
    level: String,
    percent: String,
    description: String,
    source: String,
}

fn main() -> io::Result<()> {
    let home = std::env::var("HOME").expect("HOME not set");
    let root = PathBuf::from(home).join("leetcode-rs");
    let solutions_dir = root.join("solutions");

    let mut problems = Vec::new();

    // Scan solutions/
    for entry in fs::read_dir(&solutions_dir)? {
        let path = entry?.path();
        if !path.is_file() {
            continue;
        }

        let file = match path.file_name().and_then(|s| s.to_str()) {
            Some(f) => f,
            None => continue,
        };

        // <number>.<slug>.rs
        if let Some((num, rest)) = file.split_once('.') {
            if !num.chars().all(|c| c.is_ascii_digit()) {
                continue;
            }
            if rest.ends_with(".rs") {
                let p = parse_problem(&path)?;
                problems.push(p);
            }
        }
    }

    problems.sort_by_key(|p| p.number);

    // Enable rust-analyzer
    generate_analysis_lib(&root, &problems)?;

    // Docs
    generate_problem_readmes(&problems, &root)?;
    cleanup_orphan_readmes(&problems, &root)?;
    generate_index_readme(&problems, &root)?;

    println!("sync_leetcode: DONE");
    Ok(())
}

/* ===================== Parsing ===================== */

fn parse_problem(path: &PathBuf) -> io::Result<Problem> {
    let content = fs::read_to_string(path)?;

    let mut category = String::new();
    let mut level = String::new();
    let mut percent = String::new();
    let mut desc = Vec::new();

    for line in content.lines() {
        let l = line.trim();

        if l.starts_with("// Category:") {
            category = l[12..].trim().to_string();
        } else if l.starts_with("// Level:") {
            level = l[9..].trim().to_string();
        } else if l.starts_with("// Percent:") {
            percent = l[11..].trim().to_string();
        } else if l.starts_with("//") {
            desc.push(l.trim_start_matches("//").trim().to_string());
        } else if l.starts_with("use ") || l.starts_with("impl ") {
            break;
        }
    }

    let file = path.file_name().unwrap().to_str().unwrap();
    let (num, slug) = file.split_once('.').unwrap();
    let slug = slug.strip_suffix(".rs").unwrap();

    Ok(Problem {
        number: num.parse().unwrap(),
        slug: slug.to_string(),
        title: title_case(slug),
        category,
        level,
        percent,
        description: desc.join("\n"),
        source: format!("solutions/{}.{}.rs", num, slug),
    })
}

/* ===================== rust-analyzer anchor ===================== */

fn generate_analysis_lib(root: &PathBuf, problems: &[Problem]) -> io::Result<()> {
    let src_dir = root.join("src");
    fs::create_dir_all(&src_dir)?;

    let mut out = String::new();
    out.push_str(r#"//! Analysis-only crate root.
//!
//! This file exists solely to enable IDE tooling (rust-analyzer).
//! It includes all solution files for analysis purposes only.
//!
//! DO NOT use this crate for building or running code.

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod solutions {
"#);

    for p in problems {
        out.push_str(&format!("    include!(\"../{}\");\n", p.source));
    }

    out.push_str("}\n");

    fs::write(src_dir.join("lib.rs"), out)
}

/* ===================== problems markdown ===================== */

fn generate_problem_readmes(problems: &[Problem], root: &PathBuf) -> io::Result<()> {
    let dir = root.join("problems");
    fs::create_dir_all(&dir)?;

    for p in problems {
        let path = dir.join(format!("{:03}-{}.md", p.number, p.slug));
        let mut out = String::new();

        out.push_str(&format!(
            "# {}. {}\n\n**Category:** {}  \n**Difficulty:** {}  \n**Acceptance:** {}\n\n",
            p.number, p.title, p.category, p.level, p.percent
        ));

        out.push_str(&format!(
            "**LeetCode:** [Open problem on leetcode.com](https://leetcode.com/problems/{}/)\n\n---\n\n",
            p.slug
        ));

        out.push_str("## Problem\n\n");

        let mut in_examples = false;
        let mut in_constraints = false;
        let mut example_count = 0;
        let mut constraint_buf = Vec::new();

        for raw in p.description.lines() {
            let line = raw.trim();

            if line.starts_with("Example") {
                if !in_examples {
                    out.push_str("\n---\n\n## Examples\n\n");
                    in_examples = true;
                }
                example_count += 1;
                out.push_str(&format!("### Example {}\n", example_count));
                continue;
            }

            if line.starts_with("Constraints") {
                out.push_str("\n---\n\n## Constraints\n\n");
                in_constraints = true;
                in_examples = false;
                continue;
            }

            if in_examples {
                if line.starts_with("Input:")
                    || line.starts_with("Output:")
                    || line.starts_with("Explanation:")
                {
                    out.push_str(line);
                    out.push_str("  \n");
                }
                continue;
            }

            if in_constraints {
                if !line.is_empty() {
                    constraint_buf.push(line.to_string());
                }
                continue;
            }

            if !line.is_empty() {
                out.push_str(line);
                out.push_str("\n\n");
            }
        }

        if !constraint_buf.is_empty() {
            for (i, c) in constraint_buf.iter().enumerate() {
                out.push_str(c);
                if i + 1 < constraint_buf.len() {
                    out.push_str("  \n");
                }
            }
            out.push('\n');
        }

        let test_path = format!("solutions/{}.{}.tests.dat", p.number, p.slug);
        if root.join(&test_path).exists() {
            out.push_str("\n---\n\n## Test Data\n\n");
            out.push_str(&format!("- [{}](../{})\n", test_path, test_path));
        }

        out.push_str("\n---\n\n## Source / Solution\n\n");
        out.push_str(&format!("- [{}](../{})\n", p.source, p.source));

        fs::write(path, out)?;
    }

    Ok(())
}

/* ===================== cleanup ===================== */

fn cleanup_orphan_readmes(problems: &[Problem], root: &PathBuf) -> io::Result<()> {
    let dir = root.join("problems");
    if !dir.exists() {
        return Ok(());
    }

    let expected: HashSet<_> = problems
        .iter()
        .map(|p| format!("{:03}-{}.md", p.number, p.slug))
        .collect();

    for entry in fs::read_dir(&dir)? {
        let path = entry?.path();
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            if !expected.contains(name) {
                fs::remove_file(path)?;
            }
        }
    }

    Ok(())
}

/* ===================== README.md root ===================== */

fn generate_index_readme(problems: &[Problem], root: &PathBuf) -> io::Result<()> {
    let mut out = String::new();

    out.push_str("# LeetCode Solutions (Rust)\n\n");
    out.push_str("> This repository is documentation-focused.\n");
    out.push_str("> Individual problems are documented under the `problems/` directory.\n\n");
    out.push_str("![Rust](https://img.shields.io/badge/language-Rust-orange)\n\n");

    out.push_str("## Index\n\n");
    out.push_str("| # | Problem | Difficulty | Category |\n");
    out.push_str("|---|--------|------------|----------|\n");

    for p in problems {
        out.push_str(&format!(
            "| {} | [{}](problems/{:03}-{}.md) | {} | {} |\n",
            p.number, p.title, p.number, p.slug, p.level, p.category
        ));
    }

    out.push_str("\n---\n\n");
    out.push_str("## Repository Structure\n\n");
    out.push_str("```text\n");
    out.push_str("leetcode-rs/\n");
    out.push_str("├── solutions/        # Rust solutions (source of truth)\n");
    out.push_str("├── problems/         # Auto-generated per-problem documentation\n");
    out.push_str("├── tools/            # Synchronization tool\n");
    out.push_str("├── src/              # Analysis-only crate root (rust-analyzer)\n");
    out.push_str("├── Cargo.toml\n");
    out.push_str("└── README.md\n");
    out.push_str("```\n");

    // ===== Tooling =====
    out.push_str("\n---\n\n");
    out.push_str("## Tooling\n\n");
    out.push_str("This repository uses a custom synchronization tool to keep solutions and documentation in sync.\n\n");

    out.push_str("### Usage\n\n");
    out.push_str("```bash\n");
    out.push_str("cd ~/leetcode-rs\n");
    out.push_str("rustc tools/sync_leetcode.rs -O -o tools/sync_leetcode\n");
    out.push_str("./tools/sync_leetcode\n");
    out.push_str("```\n");

    // ===== Scope =====
    out.push_str("\n---\n\n");
    out.push_str("## Scope\n\n");
    out.push_str("- This repository focuses on **clear and correct solutions**, not execution.\n");
    out.push_str("- Solutions are written for learning and reference purposes.\n");
    out.push_str("- There is no runtime, benchmarking, or online judge integration.\n");
    out.push_str("- Documentation is auto-generated and kept in sync with source files.\n");

    // ===== Notes =====
    out.push_str("\n---\n\n");
    out.push_str("## Notes\n\n");
    out.push_str("- `solutions/` is the single source of truth.\n");
    out.push_str("- Files under `problems/` are auto-generated.\n");
    out.push_str("- Manual edits to generated files will be overwritten.\n");

    // ===== Credits =====
    out.push_str("\n---\n\n");
    out.push_str("## Credits\n\n");
    out.push_str("- https://github.com/clearloop/leetcode-cli\n");

    fs::write(root.join("README.md"), out)
}

/* ===================== util ===================== */

fn title_case(slug: &str) -> String {
    slug.split('-')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
