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
use crate::init::OPT;
use crate::methods::METHODSMAP;
use crate::statistics::Statistics;

fn main() {
    let mut stat: Vec<IntValue> = (0..6).collect();

    let mut all_stats: Vec<IntValue> = Vec::new();

    for _i in 0..max(1, OPT.stat) {
        match METHODSMAP.get(&OPT.method[..]) {
            Some(method) => method(&mut stat).unwrap(),
            None => println!("Unknown method {}.", OPT.method)
        }

        if OPT.debug || OPT.stat == 1 {
            println!("{:?}", stat);
        }

        if OPT.stat > 1 {
            all_stats.extend(stat.clone());
        }
    }

    if OPT.stat > 1 {
        let statistics : Statistics = Statistics::new(&all_stats);
        println!("Min value: {}", statistics.get_min());
        println!("Max value: {}", statistics.get_max());
        println!("Mean value: {}", statistics.get_mean());
        println!("Median value: {}", statistics.get_median());
    }
}