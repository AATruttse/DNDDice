extern crate custom_error;
#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod dices;
pub mod errors;
pub mod help;
pub mod init;
pub mod method;
pub mod method_descs;
pub mod method_descs_long;
pub mod methods;
pub mod processes;
pub mod statistics;
pub mod statlists;
pub mod strings;

use std::cmp::max;

use crate::dices::IntValue;
use crate::help::help;
use crate::init::OPT;
use crate::processes::process_dices;
use crate::processes::process_method;
use crate::statistics::show_stats;

fn main() {
    if OPT.is_help() {
        help();
    }

    let mut all_stats: Vec<IntValue> = Vec::new();

    for _i in 0..max(1, OPT.num) {
        if !OPT.method.is_empty() {
            process_method(&mut all_stats);
        }
        else {
            process_dices(&mut all_stats);
        }
    }

    show_stats(&all_stats);
}