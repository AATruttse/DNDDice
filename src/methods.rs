use std::cmp::max;
use std::collections::BTreeMap;

use crate::dices::IntValue;
use crate::dices::n_d;
use crate::dices::n_d_drop;
use crate::dices::n_d_plus;

use crate::errors::DiceError;

use crate::method::Method;

use crate::method_descs::*;
use crate::method_descs_long::*;


/// Type for generation methods' BTreeMap 
type GenMethodsMap = BTreeMap<&'static str, Method>;

/// Checks, if size of vector vec is not lesser than n and resizes if needed.
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
/// Best 6 of 12 3d6 with choice
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

/// Stat generation method for CP2020 - Method 1
/// 9d10 character points
pub fn method_cp1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 1);

    stats[0] = n_d(9, 10)?;

    Ok(())
}

/// Stat generation method for CP2020 - Method 2
/// 1d10 reroll 1,2 with choice
pub fn method_cp2(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 9);

    for i in 0..9 {
        while stats[i] < 3 {
            stats[i] = n_d(1, 10)?;
        }
    }

    stats[0..9].sort();
    stats[0..9].reverse();

    Ok(())
}

/// Stat generation method for CP2013 - Method 1
/// 9d10 character points
pub fn method_cp3(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 1);

    stats[0] = n_d_plus(6, 10, 30)?;

    Ok(())
}

/// Stat generation method for CP2020 - Method 3
/// 1d10 reroll 1,2 without choice
pub fn method_cp4(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 9);

    for i in 0..9 {
        while stats[i] < 3 {
            stats[i] = n_d(1, 10)?;
        }
    }

    Ok(())
}

/// Stat generation method for Ars Magica - Method 1
/// 1d10-1d10 for each pair of characteristics with choice
pub fn method_arm1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 4);

    for i in 0..4 {
        stats[i] = n_d(1, 10)? - n_d(1, 10)?;
    }

    stats[0..4].sort();
    stats[0..4].reverse();

    Ok(())
}

/// Stat generation method for Ars Magica - Method 2
/// 1d10-1d10 for each pair of characteristics without choice
pub fn method_arm2(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 4);

    for i in 0..4 {
        stats[i] = n_d(1, 10)? - n_d(1, 10)?;
    }

    Ok(())
}

/// Stat generation method for Pathfinder - Heroic method
/// 2d6+6 with choice
pub fn method_pf1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    for i in 0..6 {
        stats[i] = n_d_plus(2, 6, 6)?;
    }

    stats[0..6].sort();
    stats[0..6].reverse();

    Ok(())
}


lazy_static! {
    pub static ref METHODSMAP: GenMethodsMap = {
        let mut m = GenMethodsMap::new();
        
        m.insert("adnd1", Method::new("d&d", true, ADND1_DESC, ADND1_HELP, method_adnd1));
        m.insert("adnd2", Method::new("d&d", true, ADND2_DESC, ADND2_HELP, method_adnd2));
        m.insert("adnd3", Method::new("d&d", false, ADND3_DESC, ADND3_HELP, method_adnd3));
        m.insert("adnd4", Method::new("d&d", false, ADND4_DESC, ADND4_HELP, method_adnd4));
        m.insert("adnd5", Method::new("d&d", false, ADND5_DESC, ADND5_HELP, method_adnd5));

        m.insert("cp2020_1", Method::new("cyberpunk-cp", true, CP2020_1_DESC, CP2020_1_HELP, method_cp1));
        m.insert("cp2020_2", Method::new("cyberpunk-stat", false, CP2020_2_DESC, CP2020_2_HELP, method_cp2));
        m.insert("cp2020_3", Method::new("cyberpunk-stat", true, CP2020_3_DESC, CP2020_3_HELP, method_cp4));
        m.insert("cp2013_1", Method::new("cyberpunk-cp", true, CP2013_1_DESC, CP2013_1_HELP, method_cp1));
        m.insert("cp2013_2", Method::new("cyberpunk-cp", true, CP2013_2_DESC, CP2013_2_HELP, method_cp3));

        m.insert("arsmagica1", Method::new("arsmagica", false, ARM1_DESC, ARM1_HELP, method_arm1));
        m.insert("arsmagica2", Method::new("arsmagica", true, ARM2_DESC, ARM2_HELP, method_arm2));

        m.insert("pfstandard", Method::new("d&d", false, PFSTANDARD_DESC, PFSTANDARD_HELP, method_adnd5));
        m.insert("pfclassic", Method::new("d&d", false, PFCLASSIC_DESC, PFCLASSIC_HELP, method_adnd3));
        m.insert("pfheroic", Method::new("d&d", false, PFHEROIC_DESC, PFHEROIC_HELP, method_pf1));

        m.insert("pathfinder2", Method::new("d&d", false, PF2_DESC, PF2_HELP, method_adnd5));

        m
    };
}


