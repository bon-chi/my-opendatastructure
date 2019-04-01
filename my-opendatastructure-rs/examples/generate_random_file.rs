extern crate rand;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::Instant;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use getopts::Options;

fn main() -> io::Result<()> {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("e", "empty-line", "including empty line");
    opts.optflag("h", "help", "print this help menu");
    match opts.parse(&args[1..]) {
        Ok(m) => {
            if m.opt_present("h") {
                let _ = print_usage(&program, opts);
            }
            if m.opt_present("e") {
                let _ = generate_file("examples/.large_random_with_empty_line_file.txt", true);
            }
            let _ = generate_file("examples/.large_random_file.txt", false);
        }
        Err(e) => panic!(e.to_string()),
    };
    println!("{}", now.elapsed().as_secs());
    Ok(())
}

fn generate_file(path: &str, with_empty_line: bool) -> io::Result<()> {
    let f = File::create(path)?;
    {
        let mut writer = BufWriter::new(f);
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let mut s: String = String::with_capacity(30 * 1001);
            for _ in 0..1000 {
                if with_empty_line {
                    if rng.gen_bool(0.001) {
                        s.push_str("\n");
                    } else {
                        s.push_str(&rng.sample_iter(&Alphanumeric).take(30).collect::<String>());
                        s += "\n";
                    }
                } else {
                    s.push_str(&rng.sample_iter(&Alphanumeric).take(30).collect::<String>());
                    s += "\n";
                }
            }
            writer.write_all(s.as_bytes())?;
        }
        Ok(())
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}
