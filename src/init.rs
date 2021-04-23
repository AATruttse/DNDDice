// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::fs::{File, OpenOptions};

use std::path::PathBuf;
use structopt::StructOpt;

use crate::strings::{LOGFILE_ERROR_MSG, LOGFILENONUTF8FILENAME_ERROR_MSG};

#[derive(Debug, StructOpt)]
#[structopt(name = "dnddice", about = "RPG dice thrower for command line. Author: Dargot, dargot@yandex.ru")]
pub struct Opt {
    /// Activate debug mode
    #[structopt(long)]
    pub debug: bool,

    /// Show help about dice codes' format description
    #[structopt(long)]
    pub help_dice_codes: bool,

    /// Show help about generation methods. See --help-method=METHOD to see help about METHOD
    #[structopt(long)]
    pub help_methods: bool,

    /// Show help about generation method
    #[structopt(long, default_value = "")]
    pub help_method: String,

    /// Show help about generation method by tags (for example: "DnD,ordered"). See --help-tags to see tags' list
    #[structopt(long="find-tags", default_value = "")]
    pub find_tags: String,

    /// Show tags' list.
    #[structopt(long)]
    pub help_tags: bool,

    /// Verbose mode (-v, -vv, etc.)
    #[structopt(short, parse(from_occurrences))]
    pub verbose: u8,

    /// Log mode (-l, -ll, etc.)
    #[structopt(short, parse(from_occurrences))]
    pub log: u8,

    /// Log file
    #[structopt(long="log-file", parse(from_os_str), default_value = "dnddice.log")]
    pub log_file: PathBuf,

    /// Number of repetitions
    #[structopt(short="N", long="repetitions-num", default_value = "1")]
    pub num: usize,

    /// Number of dices
    #[structopt(short="n", long="dice-num", default_value = "1")]
    pub dices_num: usize,

    /// Dice sides
    #[structopt(short="d", long="dice", default_value = "6")]
    pub dice: usize,

    /// Reroll dices' results
    #[structopt(short="r", long="reroll", default_value = "")]
    pub reroll: String,

    /// Result plus
    #[structopt(long="plus", default_value = "0")]
    pub plus: usize,

    /// Result minus
    #[structopt(long="minus", default_value = "0")]
    pub minus: usize,

    /// Number of lowest dices to be dropped
    #[structopt(short="D", long="drop", default_value = "0")]
    pub drop: usize,

    /// Number of greatest dices to be dropped
    #[structopt(short="C", long="crop", default_value = "0")]
    pub crop: usize,

    /// Stat generation method (adnd1, adnd2, etc.) See --help-methods for full list.
    #[structopt(short, long, default_value = "")]
    pub method: String,

    /// Method parameters
    #[structopt(short="p", long="parameters", default_value = "")]
    pub method_parameters: String,

    /// Show method description
    #[structopt(long)]
    pub show_method: bool,

    /// Show all statistics
    #[structopt(long)]
    pub stat: bool,    

    /// Show minimum
    #[structopt(long)]
    pub min: bool,

    /// Show maximum
    #[structopt(long)]
    pub max: bool,

    /// Show mean
    #[structopt(long)]
    pub mean: bool,

    /// Show median
    #[structopt(long)]
    pub median: bool,

    /// Show mode
    #[structopt(long)]
    pub mode: bool,

    /// Show probabilities
    #[structopt(long)]
    pub probabilities: bool,

    /// Show sum
    #[structopt(long)]
    pub sum: bool,

    /// Round probabilities to number of digits
    #[structopt(long, default_value = "2")]
    pub round_digits: u8,

    /// Show only numbers
    #[structopt(long)]
    pub numbers_only: bool,

    /// No help messages
    #[structopt(long)]
    pub no_help: bool,

    /// Dice codes (2d8plus1, 4d6drop1, 2d4-1d6/1d3 etc.) See --help-dice-codes for format description
    #[structopt(default_value = "")]
    pub dicecodes: Vec<String>,
}

// Static variable for options
lazy_static! {
    pub static ref OPT: Opt = {
        let opt = Opt::from_args();
        
        if opt.debug {
            println!("{:?}", opt);
        }
        
        opt
    };

    pub static ref LOGFILE: Option<File> = match OPT.log {
        x if x > 0 => {
            match OpenOptions::new()
                .append(true)
                .create(true)
                .open(&OPT.log_file) {
                    Ok(f) => Some(f),
                    Err(e) => {
                        eprintln!("{} {}:",
                                  LOGFILE_ERROR_MSG,
                                  match OPT.log_file.to_str() {
                                        Some(err_str) => err_str,
                                        None => LOGFILENONUTF8FILENAME_ERROR_MSG
                                });
                        eprintln!("{}", e);
                        None
                    }
            }
        },
        _ => None
    };
}

impl Opt {
    /// checks, if any statistics need to be collected
    pub fn is_collect_stat(&self) -> bool {
        return self.stat ||
               self.min ||
               self.max ||
               self.mean ||
               self.median ||
               self.mode ||
               self.probabilities;
    }

    /// checks, if any help info need to be shown
    pub fn is_help(&self) -> bool {
        return  self.help_dice_codes ||
                self.help_methods ||
                self.help_tags ||
                !self.help_method.is_empty() ||
                !self.find_tags.is_empty()
    }

}

