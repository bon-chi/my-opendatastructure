use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(File::open("examples/.large_random_file.txt")?);
    let mut s = String::new();
    let mut buf: Vec<String> = Vec::new();
    while let Ok(bytes) = reader.read_line(&mut s) {
        if bytes == 0 {
            break;
        } else {
            buf.push(s);
            s = String::new();
        }
    }

    let mut writer = BufWriter::new(File::create("examples/.1_1_1.txt")?);
    let mut i = 0;
    let mut s = String::new();
    while let Some(e) = buf.pop() {
        s.push_str(&e);
        if i < 10000 {
            i += 1;
        } else {
            writer.write_all(s.as_bytes())?;
            s = String::new();
            i = 0
        }
    }

    Ok(())
}
