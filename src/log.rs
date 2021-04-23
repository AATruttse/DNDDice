// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io::{BufWriter, Write};

use chrono::{Local};

use crate::dices::IntValue;
use crate::init::{LOGFILE, OPT};
use crate::statlists::StatList;
use crate::strings::{DELIMITER, LOGFILENONUTF8FILENAME_ERROR_MSG, LOGFILEWRITE_ERROR_MSG};

/// Log message to log file
pub fn log(log_str: &str) {
    match LOGFILE.as_ref() {
        Some(x) => {
            let mut f = BufWriter::new(x);
            match f.write_all(log_str.as_bytes()) {
                Err(e) => {
                    eprintln!("{} {}:",
                        LOGFILEWRITE_ERROR_MSG,
                        match OPT.log_file.to_str() {
                            Some(x) => x,
                            None => LOGFILENONUTF8FILENAME_ERROR_MSG
                        });
                    eprintln!("{}", e);
                },
                _ => ()
            };
        },
        None => ()
    };
}

/// Log message to log file with new line
pub fn logln(log_str: &str) {
    log(log_str);
    log("\n");
}

/// Log method results
pub fn log_method(is_shownumber: bool,
            is_ordered: bool,
            i: usize,
            stat : &Vec<IntValue>,
            statlist: &StatList) {
    if is_shownumber {
        log(&(i.to_string() + ": "));
    }
    if is_ordered {
        for i in 0..statlist.len() {
            log(&(statlist[i].to_string() + ": " + &stat[i].to_string() + " "));
        }
        logln("");
    }
    else {
        let stat_str = format!("{:?}\n", stat);
        log(&stat_str);
    }
}

/// Log program start
pub fn log_start() {
    let now = Local::now();
    let time_str = format!("{}", now.format("%Y %b %e %T"));

    logln(DELIMITER);
    logln(DELIMITER);
    logln(&time_str);
    logln(DELIMITER);
}