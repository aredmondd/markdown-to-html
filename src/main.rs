use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

lazy_static! {
    static ref RE_HEADINGS: Regex = Regex::new(r"^(#+)\s+(.+)$").unwrap();
    static ref RE_STRIKETHROUGH: Regex = Regex::new(r"~{2}(.+?)~{2}").unwrap();
    static ref RE_ITALICS: Regex = Regex::new(r"_(.+?)_").unwrap();
    static ref RE_INLINE_CODE: Regex = Regex::new(r"`(.+?)`").unwrap();
    static ref RE_LINKS: Regex = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
}

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
    let line = replace_strikethrough(&line);
    let line = replace_italics(&line);
    let line = replace_links(&line);
    let mut line = replace_inline_code(&line);

    if line.trim() == "<hr />" {
        line.push_str("\n");
    } else {
        line.push_str("<br>\n");
    }

    line
}

fn replace_hr(line: &str) -> String {
    line.replace("---", "<hr />")
}

fn replace_headings(line: &str) -> String {
    RE_HEADINGS
        .replace(line, |caps: &regex::Captures| {
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

fn replace_strikethrough(line: &str) -> String {
    RE_STRIKETHROUGH.replace_all(line, "<s>$1</s>").to_string()
}

fn replace_italics(line: &str) -> String {
    RE_ITALICS.replace_all(line, "<em>$1</em>").to_string()
}

fn replace_inline_code(line: &str) -> String {
    RE_INLINE_CODE
        .replace_all(
            line,
            "<span class=\"rounded-md bg-black/10 p-0.5 px-1 font-mono'>$1</span>",
        )
        .to_string()
}

fn replace_links(line: &str) -> String {
    RE_LINKS
        .replace_all(line, |caps: &regex::Captures| {
            let link_text = caps.get(1).map_or("", |m| m.as_str());
            let url = caps.get(2).map_or("", |m| m.as_str());

            format!(
                "<a href=\"{}\" class=\"hover:text-pink\">{} ↗</a>",
                url, link_text
            )
        })
        .to_string()
}
