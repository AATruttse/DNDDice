// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io::{BufWriter, Write};

use chrono::Local;

use crate::dices::IntValue;
use crate::init::{LOGFILE, OPT};
use crate::render::format_dice_str;
use crate::statlists::StatList;
use crate::strings::{LOGFILEWRITE_ERROR_MSG, NONUTF8FILENAME_ERROR_MSG};

/// Log message to log file
pub fn log(log_str: &str) {
    match LOGFILE.as_ref() {
        Some(x) => {
            let mut f = BufWriter::new(x);
            match f.write_all(log_str.as_bytes()) {
                Err(e) => {
                    eprintln!(
                        "{} {}:",
                        LOGFILEWRITE_ERROR_MSG,
                        match OPT.log_file.to_str() {
                            Some(x) => x,
                            None => NONUTF8FILENAME_ERROR_MSG,
                        }
                    );
                    eprintln!("{}", e);
                }
                _ => (),
            };
        }
        None => (),
    };
}

/// Log message to log file with new line
pub fn logln(log_str: &str) {
    log(log_str);
    log("\n");
}

/// Log method results
pub fn log_method(
    is_shownumber: bool,
    is_ordered: bool,
    i: usize,
    stat: &Vec<IntValue>,
    statlist: &StatList,
) {
    if is_shownumber {
        log(&(i.to_string() + ": "));
    }
    if is_ordered {
        for i in 0..statlist.len() {
            log(&(statlist[i].to_string() + ": " + &stat[i].to_string() + " "));
        }
        logln("");
    } else {
        let stat_str = format!("{:?}\n", stat);
        log(&stat_str);
    }
}

/// Log program start
pub fn log_start() {
    let now = Local::now();
    let time_str = format!("{}", now.format("%Y %b %e %T"));

    logln(&OPT.log_delimiter);
    logln(&OPT.log_delimiter);
    logln(&time_str);
    logln(&OPT.log_delimiter);
}

/// log roll results
pub fn log_roll(is_several_rolls: bool, is_advantage: bool, res: IntValue) {
    if OPT.log > 1 || (OPT.log > 0 && !is_several_rolls && !is_advantage) {
        let log_str = format!(": {}", res);
        logln(&log_str);
    }
}

/// logs dice from codes
pub fn log_codes(dices_num: usize, is_advantage: bool, dicecode: &str, res: IntValue) {
    if OPT.log > 0 && (dices_num > 1 || is_advantage) {
        let log_str = format!("{}: {}", dicecode, res);
        logln(&log_str);
    }
}

/// log single roll dices results
pub fn log_dices(dices: &Vec<usize>) {
    if OPT.log > 2 {
        let log_str = format!(" {:?}", dices);
        log(&log_str);
    }
}

/// log single roll result
pub fn log_dices_res(result: IntValue) {
    if !OPT.method.is_empty() && OPT.log > 1 {
        let res_str = format!(": {}", result);
        logln(&res_str);
    }
}

/// log single roll title
pub fn log_dices_title(
    n: usize,
    d: usize,
    reroll: &[usize],
    add: IntValue,
    drop: usize,
    crop: usize,
) {
    if !OPT.method.is_empty() && OPT.log > 1 {
        let code_str = format_dice_str(false, n, d, reroll, add, drop, crop, false, false);
        log(&code_str);
    }
}
