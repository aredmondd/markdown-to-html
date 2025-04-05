use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // get args for file location
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    let path = format!("{}.md", file_name);
    let temp_path = format!("{}-translation.md", file_name);

    println!("{}", path);
    println!("{}", temp_path);

    let file = File::open(path)?;
    // let temp_file = File::create(temp_path)?;
    let reader = BufReader::new(file);

    for (line_number, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        if let Some(index) = line.find("---") {
            println!("Found '---' on line {} at index {}", line_number + 1, index);
        }
    }

    Ok(())
}
