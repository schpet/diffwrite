use ansi_term::Colour::{Blue, Green, Red};
use clap::Parser;
use similar::{group_diff_ops, ChangeTag, TextDiff};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// File to update
    file: PathBuf,

    /// Number of context lines to show
    #[arg(short, long, default_value = "3")]
    context: usize,
}

fn generate_diff(old: &str, new: &str, context_lines: usize) -> String {
    let diff = TextDiff::from_lines(old, new);
    let mut result = String::new();

    result.push_str("--- a/\n");
    result.push_str("+++ b/\n");

    for op in group_diff_ops(diff.ops().to_vec(), context_lines) {
        let first_op = &op[0];
        let last_op = &op[op.len() - 1];

        let old_start = match first_op {
            similar::DiffOp::Equal { old_index, .. }
            | similar::DiffOp::Delete { old_index, .. } => *old_index,
            similar::DiffOp::Insert { .. } => 0,
            similar::DiffOp::Replace { old_index, .. } => *old_index,
        };

        let new_start = match first_op {
            similar::DiffOp::Equal { new_index, .. }
            | similar::DiffOp::Insert { new_index, .. } => *new_index,
            similar::DiffOp::Delete { .. } => 0,
            similar::DiffOp::Replace { new_index, .. } => *new_index,
        };

        let old_end = match last_op {
            similar::DiffOp::Equal { old_index, len, .. } => old_index + len,
            similar::DiffOp::Delete {
                old_index, old_len, ..
            } => old_index + old_len,
            similar::DiffOp::Insert { .. } => old_start,
            similar::DiffOp::Replace {
                old_index, old_len, ..
            } => old_index + old_len,
        };

        let new_end = match last_op {
            similar::DiffOp::Equal { new_index, len, .. } => new_index + len,
            similar::DiffOp::Insert {
                new_index, new_len, ..
            } => new_index + new_len,
            similar::DiffOp::Delete { .. } => new_start,
            similar::DiffOp::Replace {
                new_index, new_len, ..
            } => new_index + new_len,
        };

        result.push_str(&format!(
            "@@ -{},{} +{},{} @@\n",
            old_start + 1,
            old_end - old_start,
            new_start + 1,
            new_end - new_start
        ));

        for group in &op {
            for change in diff.iter_changes(group) {
                let (prefix, line) = match change.tag() {
                    ChangeTag::Delete => ("-", change.value()),
                    ChangeTag::Insert => ("+", change.value()),
                    ChangeTag::Equal => (" ", change.value()),
                };
                result.push_str(&format!("{}{}", prefix, line));
            }
        }
    }

    result
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let old_content = if args.file.exists() {
        fs::read_to_string(&args.file)?
    } else {
        String::new()
    };

    let mut new_content = String::new();
    io::stdin().read_to_string(&mut new_content)?;

    let diff_output = generate_diff(&old_content, &new_content, args.context);

    let diff_with_paths = diff_output
        .replace("--- a/\n", &format!("--- a/{}\n", args.file.display()))
        .replace("+++ b/\n", &format!("+++ b/{}\n", args.file.display()));

    for line in diff_with_paths.lines() {
        if line.starts_with('-') {
            print!("{}", Red.paint(line));
            println!();
        } else if line.starts_with('+') {
            print!("{}", Green.paint(line));
            println!();
        } else if line.starts_with("@@") {
            print!("{}", Blue.paint(line));
            println!();
        } else {
            println!("{}", line);
        }
    }

    fs::write(&args.file, new_content)?;

    Ok(())
}
