// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io::{BufWriter, Write};

use crate::init::{OUTPUTFILE, OPT};
use crate::strings::{NONUTF8FILENAME_ERROR_MSG, OUTPUTFILEWRITE_ERROR_MSG};

/// Output message to output file
fn output_file(output_str: &str) {
    match OUTPUTFILE.as_ref() {
        Some(x) => {
            let mut f = BufWriter::new(x);
            match f.write_all(output_str.as_bytes()) {
                Err(e) => {
                    eprintln!("{} {}:",
                    OUTPUTFILEWRITE_ERROR_MSG,
                        match OPT.output_file.to_str() {
                            Some(x) => x,
                            None => NONUTF8FILENAME_ERROR_MSG
                        });
                    eprintln!("{}", e);
                },
                _ => ()
            };
        },
        None => ()
    };
}

/// Output message to stdin
pub fn print(output_str: &str) {
    if !OPT.silent_mode {
        print!("{}", output_str);
    }
}

/// Output message to stdin with new line
pub fn println(output_str: &str) {
    print(output_str);
    print("\n");
}

/// Output message to stdin or output file
pub fn output(output_str: &str) {
    output_file(output_str);
    print(output_str);
}

/// Output message to stdin or output file with new line
pub fn outputln(output_str: &str) {
    output(output_str);
    output("\n");
}


