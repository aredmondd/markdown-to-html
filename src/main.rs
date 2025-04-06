use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

const HR_TAG: &str = "<hr />";

fn main() -> io::Result<()> {
    let file_name = match std::env::args().nth(1) {
        Some(name) => name,
        None => {
            eprintln!("Please provide a filename as an argument.");
            std::process::exit(1);
        }
    };

    let input_path = format!("{}.md", file_name);
    let output_path = format!("{}-translation.md", file_name);

    process_file(&input_path, &output_path)
}

fn transform_line(line: &str) -> String {
    let mut line = line.replace("---", HR_TAG);

    if line.trim() == HR_TAG {
        line.push('\n');
    } else {
        line.push_str("<br />\n");
    }

    line
}

fn process_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let file = File::open(input_path)?;
    let mut temp_file = File::create(output_path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = transform_line(&line_result?);
        temp_file.write_all(line.as_bytes())?;
    }

    Ok(())
}
