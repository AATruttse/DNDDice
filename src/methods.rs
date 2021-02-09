use std::cmp::max;
use std::collections::BTreeMap;

use crate::dices::IntValue;
use crate::dices::n_d;
use crate::dices::n_d_drop;

use crate::errors::DiceError;

/// Type for generation methods
type GenMethod = fn (&mut Vec<IntValue>) -> Result<(), DiceError>;
/// Type for generation methods' BTreeMap 
type GenMethodsMap = BTreeMap<&'static str, GenMethod>;

/// Checks, if size of vector vec is not lesser than n and resize if needed.
#[inline(always)]
fn vec_checksize(vec: &mut Vec<IntValue>, n: usize) {
    if vec.len() < n {
        vec.resize(n, 0);
    }
}

/// Stat generation method for AD&D 2ne ed - Method 1
/// 3d6 without choice
pub fn method_adnd1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    for i in 0..6 {
        stats[i] = n_d(3, 6)?;
    }

    Ok(())
}

/// Stat generation method for AD&D 2ne ed - Method 2
/// Best of two 3d6 without choice
pub fn method_adnd2(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    for i in 0..6 {
        stats[i] = max(n_d(3, 6)?, n_d(3, 6)?);
    }

    Ok(())
}

/// Stat generation method for AD&D 2ne ed - Method 3
/// 3d6 with choice
pub fn method_adnd3(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    for i in 0..6 {
        stats[i] = n_d(3, 6)?;
    }

    stats[0..6].sort();
    stats[0..6].reverse();

    Ok(())
}

/// Stat generation method for AD&D 2ne ed - Method 4
/// Best of 12 3d6
pub fn method_adnd4(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    let mut dices: Vec<IntValue> = (0..12).collect();
    for i in 0..12 {
        dices[i] = n_d(3, 6)?;
    }   
    dices.sort();
    dices.reverse();
    
    stats[0..6].clone_from_slice(&dices[0..6]);

    Ok(())
}

/// Stat generation method for AD&D 2ne ed - Method 5
/// 4d6 drop lowest with choice
pub fn method_adnd5(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    for i in 0..6 {
        stats[i] = n_d_drop(4, 6, 1)?;
    }

    stats[0..6].sort();
    stats[0..6].reverse();

    Ok(())
}

lazy_static! {
    pub static ref METHODSMAP: GenMethodsMap = {
        let mut m = GenMethodsMap::new();
        
        m.insert("adnd1", method_adnd1);
        m.insert("adnd2", method_adnd2);
        m.insert("adnd3", method_adnd3);
        m.insert("adnd4", method_adnd4);
        m.insert("adnd5", method_adnd5);

        m
    };
}


