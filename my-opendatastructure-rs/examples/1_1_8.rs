use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("examples/.large_random_file.txt")?);
    let mut writer = BufWriter::new(File::create("examples/.1_1_8.txt")?);
    let mut odds: Vec<String> = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        if (index % 2) == 0 {
            if let Ok(line) = line {
                let _ = writer.write(line.as_bytes());
                let _ = writer.write("\n".as_bytes());
            }
        } else {
            odds.push(line.expect("an error happend"));
        }
    }
    for odd in odds {
        let _ = writer.write(odd.as_bytes());
        let _ = writer.write("\n".as_bytes());
    }
    Ok(())
}
