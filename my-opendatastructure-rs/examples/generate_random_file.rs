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
    opts.optopt("n", "", "set the line length", "LEN");
    opts.optflag("e", "empty-line", "including empty line");
    opts.optflag("r", "", "random length");
    opts.optflag("h", "help", "print this help menu");
    match opts.parse(&args[1..]) {
        Ok(m) => {
            let len: usize = m
                .opt_str("n")
                .unwrap_or(String::from("30"))
                .parse()
                .expect("use integer for option -n");
            let random = m.opt_present("r");
            if m.opt_present("h") {
                let _ = print_usage(&program, opts);
            }
            if m.opt_present("e") {
                let _ = generate_file(
                    "examples/.large_random_with_empty_line_file.txt",
                    len,
                    true,
                    random,
                );
            }
            let _ = generate_file("examples/.large_random_file.txt", len, false, random);
        }
        Err(e) => panic!(e.to_string()),
    };
    println!("{}", now.elapsed().as_secs());
    Ok(())
}

fn generate_file(
    path: &str,
    line_length: usize,
    with_empty_line: bool,
    random_length: bool,
) -> io::Result<()> {
    let f = File::create(path)?;
    {
        let mut writer = BufWriter::new(f);
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let mut s: String = String::with_capacity(line_length * 1001);
            for _ in 0..1000 {
                let length = if random_length {
                    rng.gen_range(1, line_length)
                } else {
                    line_length
                };
                if with_empty_line {
                    if rng.gen_bool(0.001) {
                        s.push_str("\n");
                    } else {
                        s.push_str(
                            &rng.sample_iter(&Alphanumeric)
                                .take(length)
                                .collect::<String>(),
                        );
                        s += "\n";
                    }
                } else {
                    s.push_str(
                        &rng.sample_iter(&Alphanumeric)
                            .take(length)
                            .collect::<String>(),
                    );
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
