// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp::max;
use std::collections::BTreeMap;

use crate::dices::IntValue;
use crate::init::OPT;
use crate::output::{output, outputln};
use crate::strings::{GRAPH_CHAR, ZEROSTAT_ERROR_MSG};

/// Float type for statistics
pub type StatValue = f32;

/// type for values probabilities
type IntBins = BTreeMap<IntValue, usize>;
type StatBins = BTreeMap<IntValue, StatValue>;

//static ERROR_MSG: &str = "Can't calculate statistics from zero-length slice";

/// struct for dice statistics
pub struct Statistics
{
    mean:   StatValue,
    median: IntValue,
    mode:   IntValue,
    sum:    IntValue,

    min:    IntValue,
    max:    IntValue,
        
    bins:   IntBins,
    probabilities:   StatBins
}

impl Statistics {
    pub fn get_min(&self) -> IntValue {
        self.min
    }

    pub fn get_max(&self) -> IntValue {
        self.max
    }

    pub fn get_mean(&self) -> StatValue {
        self.mean
    }

    pub fn get_median(&self) -> IntValue {
        self.median
    }

    pub fn get_mode(&self) -> IntValue {
        self.mode
    }

    pub fn get_sum(&self) -> IntValue {
        self.sum
    }    

    pub fn get_bins(&self) -> &IntBins {
        &self.bins
    }

    pub fn get_probabilities(&self) -> &StatBins {
        &self.probabilities
    }

    fn calc_min(&mut self, data: &[IntValue]) {
        self.min = *data.iter().min().expect(ZEROSTAT_ERROR_MSG);
    }

    fn calc_max(&mut self, data: &[IntValue]) {
        self.max = *data.iter().max().expect(ZEROSTAT_ERROR_MSG);
    }

    fn calc_mean(&mut self, data: &[IntValue]) {
        if data.len() == 0 {
            panic!("{}", ZEROSTAT_ERROR_MSG);
        }

        self.mean = data.iter().sum::<IntValue>() as StatValue / data.len() as StatValue;
    }

    fn calc_sum(&mut self, data: &[IntValue]) {
        if data.len() == 0 {
            panic!("{}", ZEROSTAT_ERROR_MSG);
        }

        self.sum = data.iter().sum::<IntValue>();
    }

    fn calc_median(&mut self, data: &[IntValue]) {
        if data.len() == 0 {
            panic!("{}", ZEROSTAT_ERROR_MSG);
        }

        let mut vec: Vec<IntValue> = data.to_vec();

        vec.sort();
        let mid = data.len() / 2;
        self.median = vec[mid];
    }

    fn calc_bins(&mut self, data: &[IntValue]) {
        if data.len() == 0 {
            panic!("{}", ZEROSTAT_ERROR_MSG);
        }
  
        for &value in data {
            *self.bins.entry(value).or_insert(0) += 1;
        }

        self.probabilities = self.bins.iter()
            .map(|(key, val)| (*key, *val as StatValue / data.len() as StatValue))
            .collect();

        if OPT.debug && OPT.is_collect_stat() {
            println!("{:?}", self.bins);
            if OPT.probabilities {
                println!("{:?}", self.probabilities);
            }
        }

        self.mode = *self.bins.iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .expect(ZEROSTAT_ERROR_MSG);
    }

    pub fn new(data: &[IntValue]) -> Self {
        let mut new_stat = Statistics {
            mean: 0.0,
            median: 0,
            mode: 0,
            sum: 0,
            min: 0,
            max: 0,
            bins: IntBins::new(),
            probabilities: StatBins::new(),
        };

        new_stat.calc_min(data);
        new_stat.calc_max(data);
        new_stat.calc_mean(data);
        new_stat.calc_median(data);
        new_stat.calc_sum(data);
        new_stat.calc_bins(data);

        new_stat
    }
}

/// shows statistics result
pub fn show_stats(stats: &Vec<IntValue>) {
    let statistics : Statistics = Statistics::new(&stats);

    if OPT.sum {
        if !OPT.numbers_only {
            output("Sum: ");    
        }
        outputln(&statistics.get_sum().to_string());
    }

    if OPT.stat || OPT.min {
        if !OPT.numbers_only {
            output("Min value: ");    
        }
        outputln(&statistics.get_min().to_string());
    }

    if OPT.stat || OPT.max {
        if !OPT.numbers_only {
            output("Max value: ");    
        }
        outputln(&statistics.get_max().to_string());
    }

    if OPT.stat || OPT.mean {
        if !OPT.numbers_only {
            output("Mean value: ");    
        }
        let mean_str = format!("{:.digits$}", statistics.get_mean(), digits=OPT.round_digits as usize);
        outputln(&mean_str);
    }

    if OPT.stat || OPT.median {
        if !OPT.numbers_only {
            output("Median value: ");    
        }
        outputln(&statistics.get_median().to_string());
    }

    if OPT.stat || OPT.mode {
        if !OPT.numbers_only {
            output("Mode value: ");    
        }
        outputln(&statistics.get_mode().to_string());
    }

    if OPT.stat || OPT.probabilities {
        if !OPT.numbers_only {
            outputln("Probabilities: ");    
        }

        for (key, val) in statistics.get_probabilities() {
            let max_key_len = max(
                statistics.get_max().to_string().len(),
                statistics.get_min().to_string().len()
            );

            let key_str = format!("{: <1$} -", key, max_key_len);
            output(&key_str);

            if !OPT.prob_no_numbers {
                let val_str = format!(" {:.digits$}", val, digits=OPT.round_digits as usize);
                output(&val_str);
            }

            if OPT.prob_chart {
                let graph_str = format!(" {}", GRAPH_CHAR.to_string().repeat((val*(OPT.prob_chart_precision as f32)).round() as usize));
                output(&graph_str);
            }

            outputln("");
        }
    }
}


