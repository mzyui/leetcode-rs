use std::{
    collections::HashSet,
    fs,
    io::{self, Write},
    path::PathBuf,
};

#[derive(Debug, Hash, Eq, PartialEq)]
struct Bin {
    name: String,
    path: String,
}

fn main() -> io::Result<()> {
    // Resolve ~/leetcode
    let home = std::env::var("HOME")
        .expect("HOME environment variable not set");
    let project_root = PathBuf::from(home).join("leetcode");

    let src_dir = project_root.join("src");
    let cargo_toml_path = project_root.join("Cargo.toml");

    let mut bins: HashSet<Bin> = HashSet::new();

    // Scan src/<number>.<name>.rs
    for entry in fs::read_dir(&src_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let file_name = match path.file_name().and_then(|s| s.to_str()) {
            Some(name) => name,
            None => continue,
        };

        // Match: <number>.<name>.rs
        if let Some((num, rest)) = file_name.split_once('.') {
            if !num.chars().all(|c| c.is_ascii_digit()) {
                continue;
            }

            if let Some(bin_name) = rest.strip_suffix(".rs") {
                bins.insert(Bin {
                    name: bin_name.to_string(),
                    path: format!("src/{}", file_name),
                });
            }
        }
    }

    // Deterministic order
    let mut bins: Vec<_> = bins.into_iter().collect();
    bins.sort_by(|a, b| a.path.cmp(&b.path));

    // Read + clean Cargo.toml
    let original = fs::read_to_string(&cargo_toml_path)?;
    let mut cleaned = remove_existing_bins(&original);

    // Normalize trailing whitespace
    cleaned = cleaned.trim_end().to_string();
    cleaned.push('\n');

    // Append bins
    let mut output = cleaned;
    for bin in bins {
        output.push_str(&format!(
            "\n[[bin]]\nname = \"{}\"\npath = \"{}\"\n",
            bin.name, bin.path
        ));
    }

    // Write back Cargo.toml
    let mut file = fs::File::create(&cargo_toml_path)?;
    file.write_all(output.as_bytes())?;
    
    Ok(())
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
