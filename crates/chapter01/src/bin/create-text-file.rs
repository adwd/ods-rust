use std::io::Write;

use chapter01::LINE_COUNT;

fn main() {
    let mut file = std::fs::File::create("text.txt").unwrap();
    for i in 1..=LINE_COUNT {
        let line = format!("{}\n", i);
        file.write_all(line.as_bytes()).unwrap();
    }
}
