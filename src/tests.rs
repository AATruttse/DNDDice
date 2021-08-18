#[cfg(test)]
mod tests {
    use std::io::{prelude::*, BufReader};

    pub static TESTFILE: &str = "./test/tests.txt";

    #[test]
    fn test_file() {
        let file = match std::fs::File::open(TESTFILE) {
            Ok(ok) => ok,
            Err(e) => {
                panic!("Can't open test file {}: {}", TESTFILE, e);
            }
        };

        let reader = BufReader::new(file);
        let mut lines = Vec::new();
        for line in reader.lines() {
            match &line {
                Ok(l) => {
                    match l[0] {
                        '%' => {
                            single_test(&lines);
                            lines.clear();
                        },
                        _ => {
                            lines.push(l);
                        }
                    }
                },
                Err(e) => {
                    panic!("Can't read test file {}: {}", TESTFILE, e);
                }
            }
        }
    }

    fn single_test(lines: &Vec<String>) {
        println!("{:?}", lines);
    }

}