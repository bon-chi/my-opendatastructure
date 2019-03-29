extern crate rand;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::Instant;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() -> io::Result<()> {
    let now = Instant::now();
    let f = File::create("examples/.large_random_file.txt")?;
    {
        let mut writer = BufWriter::new(f);
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let mut s: String = String::with_capacity(30 * 1001);
            for _ in 0..1000 {
                s.push_str(&rng.sample_iter(&Alphanumeric).take(30).collect::<String>());
                s += "\n";
            }
            writer.write_all(s.as_bytes())?;
        }
    }
    println!("{}", now.elapsed().as_secs());
    Ok(())
}
