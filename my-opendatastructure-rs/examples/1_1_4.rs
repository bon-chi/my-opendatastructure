use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("examples/.large_random_file.txt")?);
    let mut set: HashSet<String> = HashSet::new();
    let mut writer = BufWriter::new(File::create("examples/.1_1_4.txt")?);

    for line in reader.lines() {
        if let Ok(line) = line {
            if !set.contains(&line) {
                let _ = writer.write(line.as_bytes());
                let _ = writer.write("\n".as_bytes());
                set.insert(line);
            }
        }
    }
    Ok(())
}
