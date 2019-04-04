use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("examples/.large_random_file.txt")?);
    let vec: Result<Vec<_>, std::io::Error> = reader.lines().collect();
    let mut rng = thread_rng();
    if let Ok(mut vec) = vec {
        vec.shuffle(&mut rng);
        let mut writer = BufWriter::new(File::create("examples/.1_1_9.txt")?);
        for string in vec {
            let _ = writer.write(string.as_bytes());
            let _ = writer.write("\n".as_bytes());
        }
    }

    Ok(())
}
