use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // get the file name passed in from the CLI
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

fn transform_line(line: &str) -> String {
    let line = replace_hr(line);
    let line = replace_headings(&line);
    let line = replace_italics(&line);
    let line = replace_strikethrough(&line);
    // let line = replace_links(&line);
    // let line = replace_inline_code(&line);
    // let line = replace_code_block(&line);

    // append a <br /> tag if necessary

    line
}

fn replace_hr(line: &str) -> String {
    line.replace("---", "<hr />")
}

fn replace_strikethrough(line: &str) -> String {
    let re = Regex::new(r"~{2}(.+?)~{2}").unwrap();

    re.replace_all(line, "<s>$1</s>").to_string()
}

fn replace_italics(line: &str) -> String {
    let re = Regex::new(r"_(.+?)_").unwrap();

    re.replace_all(line, "<em>$1</em>").to_string()
}

fn replace_headings(line: &str) -> String {
    let re = Regex::new(r"^(#+)\s+(.+)$").unwrap();

    re.replace_all(line, |caps: &regex::Captures| {
        let num_hashes = caps.get(1).map_or(0, |m| m.as_str().len());

        let tag = match num_hashes {
            1 => "h1",
            2 => "h2",
            3 => "h3",
            4 => "h4",
            _ => "h3",
        };

        let heading_text = caps.get(2).map_or("", |m| m.as_str());

        format!("<{}>{}</{}>", tag, heading_text, tag)
    })
    .to_string()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
