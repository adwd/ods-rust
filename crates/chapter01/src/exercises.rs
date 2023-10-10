mod tests {
    use std::fs::File;
    use std::io::{self, BufRead};

    use crate::LINE_COUNT;

    fn read_text_file() -> io::Result<io::Lines<io::BufReader<File>>> {
        let file = File::open("text.txt")?;
        Ok(io::BufReader::new(file).lines())
    }

    #[test]
    fn test_read_lines() {
        let lines = read_text_file().unwrap();
        for (index, line) in lines.enumerate() {
            if let Ok(ip) = line {
                assert_eq!(ip, format!("{}", index + 1));
            }
        }
    }

    #[test]
    fn question_1() {
        let mut buf = Vec::with_capacity(LINE_COUNT);
        let lines = read_text_file().unwrap();
        for line in lines {
            if let Ok(ip) = line {
                buf.push(ip);
            }
        }

        for line in buf.iter().rev() {
            println!("{}", line);
        }
    }
}
