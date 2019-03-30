use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(File::open("examples/.large_random_file.txt")?);
    let mut input = String::new();
    let mut buf: Vec<String> = Vec::with_capacity(50);
    let mut writer = BufWriter::new(File::create("examples/.1_1_2.txt")?);
    let mut i = 0;
    while let Ok(bytes) = reader.read_line(&mut input) {
        if bytes == 0 {
            break;
        } else {
            buf.push(input);
            input = String::new();
        }
        i += 1;
        if i == 50 {
            write_reverse(&mut buf, &mut writer)?;
            i = 0;
        }
    }
    write_reverse(&mut buf, &mut writer)?;
    Ok(())
}

fn write_reverse(
    buf: &mut Vec<String>,
    writer: &mut BufWriter<std::fs::File>,
) -> Result<(), std::io::Error> {
    let mut output = String::new();
    while let Some(e) = buf.pop() {
        output.push_str(&e);
    }
    writer.write_all(output.as_bytes())
}
