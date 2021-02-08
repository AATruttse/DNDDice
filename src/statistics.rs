use std::collections::BTreeMap;

use crate::dices::IntValue;

/// Float type for statistics
pub type StatValue = f32;

/// type for values probabilities
type Bins = BTreeMap<IntValue, StatValue>;

/// struct for dice statistics
pub struct Statistics
{
    mean:   StatValue,
    median: IntValue,

    min:    IntValue,
    max:    IntValue
    
    //pub bins:   Bins;
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

    fn calc_min(&mut self, data: &[IntValue]) {
        self.min = *data.iter().min().unwrap();
    }

    fn calc_max(&mut self, data: &[IntValue]) {
        self.max = *data.iter().max().unwrap();
    }

    fn calc_mean(&mut self, data: &[IntValue]) {
        self.mean = data.iter().sum::<IntValue>() as StatValue / data.len() as StatValue
    }

    fn calc_median(&mut self, data: &[IntValue]) {
        let mut vec: Vec<IntValue> = data.to_vec();

        vec.sort();
        let mid = data.len() / 2;
        self.median = vec[mid];
    }

    pub fn new(data: &[IntValue]) -> Self {
        let mut new_stat = Statistics {
            mean: 0.0,
            median: 0,
            min: 0,
            max: 0
        };

        new_stat.calc_min(data);
        new_stat.calc_max(data);
        new_stat.calc_mean(data);
        new_stat.calc_median(data);

        new_stat
    }




}


