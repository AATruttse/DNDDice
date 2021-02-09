extern crate custom_error;
#[macro_use]
extern crate lazy_static;

pub mod dices;
pub mod errors;
pub mod init;
pub mod methods;
pub mod statistics;

use std::cmp::max;

use crate::dices::IntValue;
use crate::dices::n_d_drop_crop_plus;
use crate::init::OPT;
use crate::methods::METHODSMAP;
use crate::statistics::Statistics;

fn show_stats(all_stats: &Vec<IntValue>) {
    let statistics : Statistics = Statistics::new(&all_stats);

    if OPT.stat || OPT.min {
        if !OPT.numbers_only {
            print!("Min value: ");    
        }
        println!("{}", statistics.get_min());
    }

    if OPT.stat || OPT.max {
        if !OPT.numbers_only {
            print!("Max value: ");    
        }
        println!("{}", statistics.get_max());
    }

    if OPT.stat || OPT.mean {
        if !OPT.numbers_only {
            print!("Mean value: ");    
        }
        println!("{}", statistics.get_mean());
    }

    if OPT.stat || OPT.median {
        if !OPT.numbers_only {
            print!("Median value: ");    
        }
        println!("{}", statistics.get_median());
    }

    if OPT.stat || OPT.mode {
        if !OPT.numbers_only {
            print!("Mode value: ");    
        }
        println!("{}", statistics.get_mode());
    }

    if OPT.stat || OPT.probabilities {
        if !OPT.numbers_only {
            print!("Probabilities: ");    
        }
        println!("{:?}", statistics.get_probabilities());
    }

}

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
            let res = match n_d_drop_crop_plus(OPT.dices_num,
                OPT.dice,
                OPT.plus as IntValue - OPT.minus as IntValue,
                OPT.drop,
                OPT.crop
            ) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };

            if OPT.debug || OPT.verbose > 0 || !OPT.is_collect_stat() {
                println!("{}", res);
            }

            all_stats.push(res);
        }
    }

    show_stats(&all_stats);
}