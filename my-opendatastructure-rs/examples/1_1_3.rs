use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(File::open(
        "examples/.large_random_with_empty_line_file.txt",
    )?);
    let mut input = String::new();
    let mut queue: VecDeque<String> = VecDeque::with_capacity(42);
    let mut writer = BufWriter::new(File::create("examples/.1_1_3.txt")?);
    let mut i = 0;
    while let Ok(bytes) = reader.read_line(&mut input) {
        if bytes == 0 {
            break;
        } else {
            if queue.len() == 42 {
                let front = queue.pop_front();
                if input == "\n" {
                    if let Some(mut front) = front {
                        writer.write(front.as_bytes());
                    }
                }
            }
            queue.push_back(input);
            input = String::new();
        }
    }
    Ok(())
}
