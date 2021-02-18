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

/// Calculates modifier for stat according to D&D3 and later rules
#[inline(always)]
fn dnd3mod(stat: IntValue) -> IntValue {
    match stat {
        stat if stat > 10 => (stat - 10) / 2,
        stat if stat < 10 => (stat - 11) / 2,
        _ => 0
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

/// Stat generation method for Cyberspace - Method 1
/// 1d100 with choice
pub fn method_cyberspace1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 11);

    for i in 0..11 {
            stats[i] = n_d(1, 100)?;
    }

    stats[0..11].sort();
    stats[0..11].reverse();

    Ok(())
}

/// Stat generation method for Cyberspace - Method 2
/// 1d100 without choice
pub fn method_cyberspace2(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 11);

    for i in 0..11 {
            stats[i] = n_d(1, 100)?;
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

/// Stat generation method for D&D 3rd ed - Organic method
/// 4d6 drop lowest  without choice, reroll one, switch any two.
pub fn method_dnd3_1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 7);

    for i in 0..7 {
        stats[i] = n_d_drop(4, 6, 1)?;
    }

    Ok(())
}

/// Stat generation method for D&D 3rd ed - Custom average method
/// Six 3d6 with choice. Reroll very bad stats.
pub fn method_dnd3_2(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    while *stats[0..6].iter().max().unwrap() < 12 ||
            stats[0..6].iter().fold (0, |sum, val| sum + dnd3mod(*val)) < -2 {
            for i in 0..6 {
                stats[i] = n_d(3, 6)?;
            }
    }

    stats[0..6].sort();
    stats[0..6].reverse();

    Ok(())
}

/// Stat generation method for D&D 3rd ed - Random average method
/// Six 3d6 without choice. Reroll very bad stats.
pub fn method_dnd3_3(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    while *stats[0..6].iter().max().unwrap() < 12 ||
            stats[0..6].iter().fold (0, |sum, val| sum + dnd3mod(*val)) < -2 {
            for i in 0..6 {
                stats[i] = n_d(3, 6)?;
            }
    }

    Ok(())
}

/// Stat generation method for D&D 3rd ed - High-powered method
/// Six 5d6 drop lowest two with choice. Reroll below average stats.
pub fn method_dnd3_4(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    while *stats[0..6].iter().max().unwrap() < 15 ||
            stats[0..6].iter().fold (0, |sum, val| sum + dnd3mod(*val)) < 2 {
            for i in 0..6 {
                stats[i] = n_d_drop(4, 6, 1)?;
            }
    }

    stats[0..6].sort();
    stats[0..6].reverse();

    Ok(())
}

/// Stat generation method for D&D 3.5 ed - Elite array method
/// 15,14,13,12,10,8 with choice
pub fn method_dnd35(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    *stats = [15, 14, 13, 12, 10, 8].to_vec();

    Ok(())
}

/// Stat generation method for D&D 4th ed - Rolling scores method
/// Six 4d6 drop 1 with choice. Reroll bad stats.
pub fn method_dnd4_1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    while stats[0..6].iter().fold (0, |sum, val| sum + dnd3mod(*val)) < 4 {
            for i in 0..6 {
                stats[i] = n_d_drop(4, 6, 1)?;
            }
    }

    stats[0..6].sort();
    stats[0..6].reverse();

    Ok(())
}

/// Stat generation method for D&D 4th ed - Standard method
/// 16,14,13,12,11,10 with choice
pub fn method_dnd4_2(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    *stats = [16, 14, 13, 12, 11, 10].to_vec();

    Ok(())
}

/// Stat generation method for WH40K - Method 1
/// 2d10 for each characteristic without choice
pub fn method_wh40k1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 9);

    for i in 0..9 {
        stats[i] = n_d(2, 10)?;
    }

    Ok(())
}

/// Stat generation method for WH40K - Method 2
/// 2d10 for each characteristic without choice, reroll one
pub fn method_wh40k2(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 10);

    for i in 0..10 {
        stats[i] = n_d(2, 10)?;
    }

    Ok(())
}

/// Stat generation method for WH40K - Method 3
/// 2d10 for each characteristic with choice
pub fn method_wh40k3(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 9);

    for i in 0..9 {
        stats[i] = n_d(2, 10)?;
    }

    stats[0..9].sort();
    stats[0..9].reverse();

    Ok(())
}

/// Stat generation method for WH40K - Method 4
/// 2d10 for each characteristic with choice, reroll one
pub fn method_wh40k4(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 10);

    for i in 0..10 {
        stats[i] = n_d(2, 10)?;
    }

    stats[0..10].sort();
    stats[0..10].reverse();

    Ok(())
}

// BTreeMap with all methods
lazy_static! {
    pub static ref METHODSMAP: GenMethodsMap = {
        let mut m = GenMethodsMap::new();

        m.insert("dnd", Method::new("d&d", true, DND_DESC, DND_HELP, method_adnd1));
        
        m.insert("adnd1", Method::new("d&d", true, ADND1_DESC, ADND1_HELP, method_adnd1));
        m.insert("adnd2", Method::new("d&d", true, ADND2_DESC, ADND2_HELP, method_adnd2));
        m.insert("adnd3", Method::new("d&d", false, ADND3_DESC, ADND3_HELP, method_adnd3));
        m.insert("adnd4", Method::new("d&d", false, ADND4_DESC, ADND4_HELP, method_adnd4));
        m.insert("adnd5", Method::new("d&d", false, ADND5_DESC, ADND5_HELP, method_adnd5));

        m.insert("dnd3", Method::new("d&d", false, DND3_DESC, DND3_HELP, method_adnd5));
        m.insert("dnd3organic", Method::new("d&dreroll", true, DND3ORGANIC_DESC, DND3ORGANIC_HELP, method_dnd3_1));
        m.insert("dnd3customavg", Method::new("d&d", false, DND3CA_DESC, DND3CA_HELP, method_dnd3_2));
        m.insert("dnd3randomavg", Method::new("d&d", true, DND3RA_DESC, DND3RA_HELP, method_dnd3_3));
        m.insert("dnd3highpow", Method::new("d&d", false, DND3HP_DESC, DND3HP_HELP, method_dnd3_4));

        m.insert("dnd35", Method::new("d&d", false, DND35_DESC, DND35_HELP, method_adnd5));
        m.insert("dnd35organic", Method::new("d&dreroll", true, DND35ORGANIC_DESC, DND35ORGANIC_HELP, method_dnd3_1));
        m.insert("dnd35customavg", Method::new("d&d", false, DND35CA_DESC, DND35CA_HELP, method_dnd3_2));
        m.insert("dnd35randomavg", Method::new("d&d", true, DND35RA_DESC, DND35RA_HELP, method_dnd3_3));
        m.insert("dnd35highpow", Method::new("d&d", false, DND35HP_DESC, DND35HP_HELP, method_dnd3_4));
        m.insert("dnd35elite", Method::new("d&d", false, DND35ELITE_DESC, DND35ELITE_HELP, method_dnd35));

        m.insert("dnd4", Method::new("d&d", false, DND4_DESC, DND4_HELP, method_dnd4_1));
        m.insert("dnd4standard", Method::new("d&d", false, DND4STANDARD_DESC, DND4STANDARD_HELP, method_dnd4_2));

        m.insert("dnd5", Method::new("d&d", false, DND5_DESC, DND5_HELP, method_adnd5));
        m.insert("dnd5standard", Method::new("d&d", false, DND5STANDARD_DESC, DND5STANDARD_HELP, method_dnd35));

        m.insert("pfstandard", Method::new("d&d", false, PFSTANDARD_DESC, PFSTANDARD_HELP, method_adnd5));
        m.insert("pfclassic", Method::new("d&d", false, PFCLASSIC_DESC, PFCLASSIC_HELP, method_adnd3));
        m.insert("pfheroic", Method::new("d&d", false, PFHEROIC_DESC, PFHEROIC_HELP, method_pf1));

        m.insert("pathfinder2", Method::new("d&d", false, PF2_DESC, PF2_HELP, method_adnd5));

        m.insert("cp2013_1", Method::new("cyberpunk-cp", true, CP2013_1_DESC, CP2013_1_HELP, method_cp1));
        m.insert("cp2013_2", Method::new("cyberpunk-cp", true, CP2013_2_DESC, CP2013_2_HELP, method_cp3));
        m.insert("cp2020_1", Method::new("cyberpunk-cp", true, CP2020_1_DESC, CP2020_1_HELP, method_cp1));
        m.insert("cp2020_2", Method::new("cyberpunk-stat", false, CP2020_2_DESC, CP2020_2_HELP, method_cp2));
        m.insert("cp2020_3", Method::new("cyberpunk-stat", true, CP2020_3_DESC, CP2020_3_HELP, method_cp4));

        m.insert("cyberspace1", Method::new("cyberspace", false, CYBERSPACE_1_DESC, CYBERSPACE_1_HELP, method_cyberspace1));
        m.insert("cyberspace2", Method::new("cyberspace", true, CYBERSPACE_2_DESC, CYBERSPACE_2_HELP, method_cyberspace2));

        m.insert("arsmagica1", Method::new("arsmagica", false, ARM1_DESC, ARM1_HELP, method_arm1));
        m.insert("arsmagica2", Method::new("arsmagica", true, ARM2_DESC, ARM2_HELP, method_arm2));

        m.insert("wh40k", Method::new("warhammer", true, WH40K_DESC, WH40K_HELP, method_wh40k1));
        m.insert("wh40k_reroll", Method::new("warhammer-reroll", true, WH40KREROLL_DESC, WH40KREROLL_HELP, method_wh40k2));
        m.insert("wh40k_choice", Method::new("warhammer", false, WH40KCHOICE_DESC, WH40KCHOICE_HELP, method_wh40k3));
        m.insert("wh40k_choicereroll", Method::new("warhammer-reroll", false, WH40KCHOICEREROLL_DESC, WH40KCHOICEREROLL_HELP, method_wh40k4));
        
        m
    };
}


