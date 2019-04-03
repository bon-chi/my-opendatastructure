use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("examples/.large_random_file.txt")?);
    let mut set: BTreeSet<StringWithLen> = BTreeSet::new();
    let mut writer = BufWriter::new(File::create("examples/.1_1_6.txt")?);

    for line in reader.lines() {
        if let Ok(line) = line {
            let _ = set.insert(StringWithLen(line));
        }
    }
    for line in set {
        let _ = writer.write(line.as_bytes());
        let _ = writer.write("\n".as_bytes());
    }
    Ok(())
}

struct StringWithLen(String);
impl StringWithLen {
    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
impl std::fmt::Display for StringWithLen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl PartialEq for StringWithLen {
    fn eq(&self, other: &StringWithLen) -> bool {
        self.0.len() == other.0.len() && self.0 == other.0
    }
}
impl Eq for StringWithLen {}

impl PartialOrd for StringWithLen {
    fn partial_cmp(&self, other: &StringWithLen) -> Option<Ordering> {
        match self.0.len().cmp(&other.0.len()) {
            Ordering::Equal => Some(self.0.cmp(&other.0)),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
        }
    }
}

impl Ord for StringWithLen {
    fn cmp(&self, other: &StringWithLen) -> Ordering {
        match self.0.len().cmp(&other.0.len()) {
            Ordering::Equal => self.0.cmp(&other.0),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}
