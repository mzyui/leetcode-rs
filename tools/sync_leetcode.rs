use std::{
    collections::HashSet,
    fs,
    io::{self},
    path::PathBuf,
};

#[derive(Debug, Hash, Eq, PartialEq)]
struct Bin {
    name: String,
    path: String,
}

struct Problem {
    number: u32,
    slug: String,
    title: String,
    category: String,
    level: String,
    percent: String,
    description: String,
    tests: Vec<(String, String)>,
    source: String,
}

fn main() -> io::Result<()> {
    let home = std::env::var("HOME").expect("HOME not set");
    let root = PathBuf::from(home).join("leetcode");

    let solutions_dir = root.join("solutions");
    let cargo_toml = root.join("Cargo.toml");

    let mut bins = HashSet::new();
    let mut problems = Vec::new();

    for entry in fs::read_dir(&solutions_dir)? {
        let path = entry?.path();
        if !path.is_file() {
            continue;
        }

        let file = match path.file_name().and_then(|s| s.to_str()) {
            Some(f) => f,
            None => continue,
        };

        // match <number>.<slug>.rs
        if let Some((num, rest)) = file.split_once('.') {
            if !num.chars().all(|c| c.is_ascii_digit()) {
                continue;
            }

            if let Some(slug) = rest.strip_suffix(".rs") {
                bins.insert(Bin {
                    name: slug.to_string(),
                    path: format!("solutions/{}", file),
                });

                let mut p = parse_problem(&path)?;
                p.tests = parse_tests(&root.join(format!(
                    "solutions/{}.{}.tests.dat",
                    p.number, p.slug
                )))?;
                problems.push(p);
            }
        }
    }

    // canonical order
    problems.sort_by_key(|p| p.number);

    generate_problem_readmes(&problems, &root)?;
    cleanup_orphan_readmes(&problems, &root)?;
    generate_index_readme(&problems, &root)?;

    write_bins(&cargo_toml, bins)?;

    println!("sync_leetcode: DONE (solutions/ as source of truth)");
    Ok(())
}

/* ===================== Cargo.toml ===================== */

fn write_bins(cargo_toml: &PathBuf, bins: HashSet<Bin>) -> io::Result<()> {
    let original = fs::read_to_string(cargo_toml)?;
    let mut cleaned = remove_existing_bins(&original);

    cleaned = cleaned.trim_end().to_string();
    cleaned.push('\n');

    let mut bins: Vec<_> = bins.into_iter().collect();
    bins.sort_by(|a, b| a.path.cmp(&b.path));

    let mut out = cleaned;
    for b in bins {
        out.push_str(&format!(
            "\n[[bin]]\nname = \"{}\"\npath = \"{}\"\n",
            b.name, b.path
        ));
    }

    fs::write(cargo_toml, out)
}

fn remove_existing_bins(input: &str) -> String {
    let mut lines = Vec::new();
    let mut skip = false;

    for line in input.lines() {
        if line.trim() == "[[bin]]" {
            skip = true;
            continue;
        }
        if skip {
            if line.trim().is_empty() {
                skip = false;
            }
            continue;
        }
        lines.push(line);
    }

    lines.join("\n")
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
        tests: vec![],
        source: format!("solutions/{}.{}.rs", num, slug),
    })
}

fn parse_tests(path: &PathBuf) -> io::Result<Vec<(String, String)>> {
    if !path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(path)?;
    let lines: Vec<_> = content.lines().collect();

    Ok(lines
        .chunks(2)
        .filter(|c| c.len() == 2)
        .map(|c| (c[0].to_string(), c[1].to_string()))
        .collect())
}

/* ===================== README generation ===================== */

fn generate_problem_readmes(problems: &[Problem], root: &PathBuf) -> io::Result<()> {
    let dir = root.join("problems");
    fs::create_dir_all(&dir)?;

    for p in problems {
        let path = dir.join(format!("{:03}-{}.md", p.number, p.slug));
        let mut out = String::new();

        out.push_str(&format!(
            "# {}. {}\n\n**Category:** {}  \n**Difficulty:** {}  \n**Acceptance:** {}\n\n---\n\n",
            p.number, p.title, p.category, p.level, p.percent
        ));

        out.push_str("## Problem\n\n");
        out.push_str(&p.description);
        out.push_str("\n\n---\n\n");

        if !p.tests.is_empty() {
            out.push_str("## Test Cases\n\n| nums | target |\n|------|--------|\n");
            for (n, t) in &p.tests {
                out.push_str(&format!("| {} | {} |\n", n, t));
            }
            out.push_str("\n---\n\n");
        }

        out.push_str("## Source / Solution\n\n");
        out.push_str(&format!(
            "- [{}](../{})\n",
            p.source, p.source
        ));

        fs::write(path, out)?;
    }

    Ok(())
}

fn cleanup_orphan_readmes(problems: &[Problem], root: &PathBuf) -> io::Result<()> {
    let dir = root.join("problems");
    if !dir.exists() {
        return Ok(());
    }

    let expected: HashSet<String> = problems
        .iter()
        .map(|p| format!("{:03}-{}.md", p.number, p.slug))
        .collect();

    for entry in fs::read_dir(&dir)? {
        let path = entry?.path();
        if !path.is_file() {
            continue;
        }

        let name = match path.file_name().and_then(|s| s.to_str()) {
            Some(n) => n,
            None => continue,
        };

        if !expected.contains(name) {
            fs::remove_file(&path)?;
            println!("Removed orphan README: {}", name);
        }
    }

    Ok(())
}

fn generate_index_readme(problems: &[Problem], root: &PathBuf) -> io::Result<()> {
    let mut out = String::new();

    out.push_str("# LeetCode Solutions (Rust)\n\n");
    out.push_str("| # | Problem | Difficulty | Category |\n");
    out.push_str("|---|--------|------------|----------|\n");

    for p in problems {
        out.push_str(&format!(
            "| {} | [{}](problems/{:03}-{}.md) | {} | {} |\n",
            p.number, p.title, p.number, p.slug, p.level, p.category
        ));
    }

    out.push_str("\n---\n\n");
    out.push_str("## Tooling\n\n");
    out.push_str("This repository uses a custom synchronization tool to keep binaries and documentation consistent.\n\n");

    out.push_str("### Tools Used\n\n");
    out.push_str("- **Rust** (stable toolchain)\n");
    out.push_str("- **Cargo**\n");
    out.push_str("- **rustc**\n");
    out.push_str("- Custom synchronization tool: `tools/sync_leetcode.rs`\n");
    out.push_str("- LeetCode CLI: https://github.com/clearloop/leetcode-cli\n\n");

    out.push_str("### Tool Responsibilities\n\n");
    out.push_str("- Register all `solutions/<no>.<slug>.rs` as Cargo binaries\n");
    out.push_str("- Generate per-problem README under `problems/`\n");
    out.push_str("- Generate this index README\n");
    out.push_str("- Remove orphan documentation files (hard sync)\n\n");

    out.push_str("---\n\n");
    out.push_str("## How to Compile & Run the Tool\n\n");
    out.push_str("```bash\n");
    out.push_str("cd ~/leetcode\n");
    out.push_str("rustc tools/sync_leetcode.rs -O -o tools/sync_leetcode\n");
    out.push_str("./tools/sync_leetcode\n");
    out.push_str("```\n\n");

    out.push_str("> Safe to run repeatedly. All outputs are deterministic.\n\n");

    out.push_str("---\n\n");
    out.push_str("## Credits\n\n");
    out.push_str("Problem statements and test data were generated using:\n\n");
    out.push_str("- https://github.com/clearloop/leetcode-cli\n\n");

    out.push_str("---\n\n");
    out.push_str("## Source of Truth\n\n");
    out.push_str("- `solutions/*.rs`\n");
    out.push_str("- `solutions/*.tests.dat`\n\n");
    out.push_str("Manual edits to generated README files will be overwritten.\n");

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
