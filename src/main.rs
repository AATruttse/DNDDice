extern crate custom_error;
#[macro_use]
extern crate lazy_static;

pub mod dices;
pub mod errors;
pub mod init;
pub mod methods;
pub mod processes;
pub mod statistics;

use std::cmp::max;

use crate::dices::IntValue;
use crate::init::OPT;
use crate::methods::METHODSMAP;
use crate::processes::process_dices;
use crate::statistics::show_stats;

fn main() {
    let mut stat: Vec<IntValue> = (0..6).collect();

    let mut all_stats: Vec<IntValue> = Vec::new();

    for _i in 0..max(1, OPT.num) {
        if !OPT.method.is_empty() {
            match METHODSMAP.get(&OPT.method[..]) {
                Some(method) => method(&mut stat).unwrap(),
                None => {
                    eprintln!("Unknown method {}.", OPT.method);
                    std::process::exit(1);
                }
            }

            if OPT.debug || OPT.verbose > 0 || !OPT.is_collect_stat() {
                println!("{:?}", stat);
            }

            all_stats.extend(stat.clone());
        }
        else {
            process_dices(&mut all_stats);
        }
    }

    show_stats(&all_stats);
}