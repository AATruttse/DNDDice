#[cfg(test)]
mod tests {
    use std::io::{prelude::*, BufReader};
    use std::process::Command;
    use regex::{Regex, escape};
    use std::str;

    pub static TESTFILE: &str = "./test/tests.txt";
    pub static TESTDELIMITER: &str = "%%%%";
    pub static TESTCOMMAND_WIN: &str = "./target/debug/dnddice.exe";
    pub static TESTCOMMAND_LIN: &str = "./target/debug/dnddice";
    pub static TESTCOMMENT: &str = "//";

    /// runs all tests on windows
    #[test]
    fn test_win() {
        test_dice_codes(TESTCOMMAND_WIN);
    }

    /// runs all tests on linux
    #[test]
    fn test_linux() {
        test_dice_codes(TESTCOMMAND_LIN);
    }

    /// reads file with tests divided by delimiter strings and executes test one-by-one
    fn test_dice_codes(test_command: &str) {
        let file = match std::fs::File::open(TESTFILE) {
            Ok(ok) => ok,
            Err(e) => {
                panic!("Can't open test file {}: {}", TESTFILE, e);
            }
        };

        let reader = BufReader::new(file);
        let mut test_num : usize = 1;
        let mut line_num : usize = 0;
        let mut lines = Vec::new(); // test line-by-line. first line - command to execute, second and other - command stdout
        for line in reader.lines() {
            match line {
                Ok(l) => {
                    line_num += 1;
                    // if strings starts with delimiter - executes test, clear lines vector
                    if l.as_str().starts_with(TESTDELIMITER) {
                            single_test(test_command, line_num, test_num, &lines);
                            lines.clear();
                            test_num += 1;
                    }
                    // if strings starts with comment - do nothing
                    else if l.as_str().starts_with(TESTCOMMENT) {
                    }
                    else {
                            lines.push(l);
                    }
                },
                Err(e) => {
                    panic!("Can't read test file {}: {}", TESTFILE, e);
                }
            }
        }

        println!("Successfully executed {} tests.", test_num);
    }

    /// executes single test
    /// n - number of test
    /// lines - first line - command to execute, second and other - command stdout
    fn single_test(test_command: &str, line_num: usize, test_num: usize, lines: &Vec<String>) {
        // test must be non-empty
        if lines.len() == 0
        {
            panic!("Test file error - empty test #{} on line {}", test_num, line_num);
        }

        let mut run_cmd = Command::new(test_command);
        run_cmd.args(lines[0].split(" "));

        let cmd_res = match run_cmd.output() {
            Ok(out) => out,
            Err(e) => {
                panic!("Can't execute command {} {} (test #{} on line {}):\n{}", test_command, lines[0], test_num, line_num, e);
            }
        };

        assert!(cmd_res.status.success());
        println!("{} {}", test_command, lines[0]);

        let cmd_stdout = match str::from_utf8(&cmd_res.stdout) {
            Ok(s) => s,
            Err(e) => panic!("Invalid UTF-8 sequence in command {} {} output (test #{} on line {}):\n{}", test_command, lines[0], test_num, line_num, e),
        };

        let cmd_out_strings : Vec<&str> = cmd_stdout.split("\n").filter(|&s| s.len() != 0).collect(); // command stdout splitted line-by-line
        let test_out_strings : Vec<&str> = lines[1..].iter().map(|s| s.as_str()).collect(); // test template for output - second and other lines 
        assert!(cmd_out_strings.len() == test_out_strings.len()); // number of lines in cmd stdout and in test template must be the same

        for i in 0..cmd_out_strings.len() {
            print!("\"{}\" - \"{}\"...", cmd_out_strings[i], test_out_strings[i]);

            let rg = Regex::new(r"(@(-?\d+)-(-?\d+))").unwrap(); // regex for min-max sequence (@min-max) in test string

            // need to do complex operation
            // replace all min-max sequence (@min-max) in test string with @ symbol
            // escape all special characters in test string
            // replace @ symbol with sequence for number (\d+)
            let test_str = escape(
                    &rg.replace_all(test_out_strings[i], "@") 
                ).
                replace("@","(-?\\d+)");
            let test_rg = Regex::new(&test_str).unwrap();

            if !test_rg.is_match(cmd_out_strings[i]) {
                println!("   Error!");
                panic!("Failed test #{} (line {}) - template \"{}\" doesn't match output \"{}\"", test_num, line_num, test_out_strings[i], cmd_out_strings[i]);
            }

            let num_rg_test = Regex::new(r"(?:(@(-?\d+)-(-?\d+))|(-?\d+))").unwrap(); // regex for numbers and min-max sequence (@min-max) in test string
            let num_rg_stdout = Regex::new(r"(?:-?\d+)").unwrap(); // regex for numbers in stdout string

            // regexp groups with min's and max's for numbers from tests
            let test_nums : Vec<Vec<&str>> = num_rg_test.
                captures_iter(test_out_strings[i]).
                into_iter().
                map(|capture_it| capture_it.iter().map(|m| m.map_or("", |m| m.as_str())).collect::<Vec<&str>>()).
                collect::<Vec<Vec<&str>>>();

            // regexp groups for numbers from cmd's stdout
            let out_nums : Vec<Vec<&str>> = num_rg_stdout.
                captures_iter(cmd_out_strings[i]).
                into_iter().
                map(|capture_it| capture_it.iter().map(|m| m.map_or("", |m| m.as_str())).collect::<Vec<&str>>()).
                collect::<Vec<Vec<&str>>>();
                
            if test_nums.len() != out_nums.len() {
                println!("   Error!");
                panic!("Failed test #{} (line {}) - template \"{}\" doesn't match output \"{}\"", test_num, line_num, test_out_strings[i], cmd_out_strings[i]);                
            }

            for j in 0..out_nums.len() {
                if test_nums[j][1].len() != 0
                {
                    let out_num : i64 = out_nums[j][0].parse().unwrap();
                    let test_min : i64 = test_nums[j][2].parse().unwrap();
                    let test_max : i64 = test_nums[j][3].parse().unwrap();

                    if out_num < test_min || out_num > test_max {
                        println!("   Error!");
                        panic!("Failed test #{} (line {}) - template \"{}\" doesn't match output \"{}\":
                        Number {} doesn't fit in [{},{}] limits.", test_num, line_num, test_out_strings[i], cmd_out_strings[i], out_num, test_min, test_max); 
                    }
                }
            }

            println!("   Ok!");            
        }
    }

}