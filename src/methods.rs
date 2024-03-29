// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp::max;
use std::collections::BTreeMap;

use ndarray::{array, Array2};

use crate::dices::{n_d, n_d_drop, n_d_plus, n_d_reroll_drop, IntValue};

use crate::errors::DiceError;

use crate::method::{Method, Tags};

use crate::method_comments::*;
use crate::method_descs::*;
use crate::method_descs_long::*;

static ONE_ARRAY: [usize; 1] = [1];
static ALL25_ARRAY: [IntValue; 6] = [25, 25, 25, 25, 25, 25];
static DND35_ARRAY: [IntValue; 6] = [15, 14, 13, 12, 10, 8];
static DND4_ARRAY: [IntValue; 6] = [16, 14, 13, 12, 11, 10];
static SFFOCUSED_ARRAY: [IntValue; 6] = [18, 14, 11, 10, 10, 10];
static SFSPLIT_ARRAY: [IntValue; 6] = [16, 16, 11, 10, 10, 10];
static SFVERSATILE_ARRAY: [IntValue; 6] = [14, 14, 14, 11, 10, 10];
static SWN_ARRAY: [IntValue; 6] = [14, 14, 14, 11, 10, 10];

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
        _ => 0,
    }
}

/// Returns given array as result
fn method_array(arr: &[IntValue], stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, arr.len());

    *stats = arr.to_vec();

    Ok(())
}

/// Returns random array from given set as result
fn method_rnd_array(arr: &Array2<IntValue>, stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, arr.ncols());

    *stats = arr.row(n_d_plus(1, arr.nrows(), -1)? as usize).to_vec();

    Ok(())
}

/// N dices without choice
fn method_nd_wochoice(
    statnum: usize,
    n: usize,
    d: usize,
    stats: &mut Vec<IntValue>,
) -> Result<(), DiceError> {
    vec_checksize(stats, statnum);

    for i in 0..statnum {
        stats[i] = n_d(n, d)?;
    }

    Ok(())
}

/// N dices with choice
fn method_nd_wchoice(
    statnum: usize,
    n: usize,
    d: usize,
    stats: &mut Vec<IntValue>,
) -> Result<(), DiceError> {
    vec_checksize(stats, statnum);

    for i in 0..statnum {
        stats[i] = n_d(n, d)?;
    }

    stats[0..statnum].sort();
    stats[0..statnum].reverse();

    Ok(())
}

/// Best of 2 N dices without choice
fn method_best_nd_wochoice(
    statnum: usize,
    n: usize,
    d: usize,
    stats: &mut Vec<IntValue>,
) -> Result<(), DiceError> {
    vec_checksize(stats, statnum);

    for i in 0..statnum {
        stats[i] = max(n_d(n, d)?, n_d(n, d)?);
    }

    Ok(())
}

/// Best statnum of double statnum N dices with choice
fn method_bestlist_nd_wchoice(
    statnum: usize,
    n: usize,
    d: usize,
    stats: &mut Vec<IntValue>,
) -> Result<(), DiceError> {
    vec_checksize(stats, statnum);

    let mut dices: Vec<IntValue> = (0..(statnum as IntValue) * 2).collect();
    for i in 0..statnum * 2 {
        dices[i] = n_d(n, d)?;
    }

    dices.sort();
    dices.reverse();

    stats[0..statnum].clone_from_slice(&dices[0..statnum]);

    Ok(())
}

/// N dices plus without choice
fn method_nd_plus_wochoice(
    statnum: usize,
    n: usize,
    d: usize,
    plus: IntValue,
    stats: &mut Vec<IntValue>,
) -> Result<(), DiceError> {
    vec_checksize(stats, statnum);

    for i in 0..statnum {
        stats[i] = n_d_plus(n, d, plus)?;
    }

    Ok(())
}

/// N dices plus with choice
fn method_nd_plus_wchoice(
    statnum: usize,
    n: usize,
    d: usize,
    plus: IntValue,
    stats: &mut Vec<IntValue>,
) -> Result<(), DiceError> {
    vec_checksize(stats, statnum);

    for i in 0..statnum {
        stats[i] = n_d_plus(n, d, plus)?;
    }

    stats[0..statnum].sort();
    stats[0..statnum].reverse();

    Ok(())
}

/// N dices drop 1 with choice
fn method_nddrop1_wchoice(
    statnum: usize,
    n: usize,
    d: usize,
    stats: &mut Vec<IntValue>,
) -> Result<(), DiceError> {
    vec_checksize(stats, statnum);

    for i in 0..statnum {
        stats[i] = n_d_drop(n, d, 1)?;
    }

    stats[0..statnum].sort();
    stats[0..statnum].reverse();

    Ok(())
}

/// N dices drop 1 without choice
fn method_nddrop1_wochoice(
    statnum: usize,
    n: usize,
    d: usize,
    stats: &mut Vec<IntValue>,
) -> Result<(), DiceError> {
    vec_checksize(stats, statnum);

    for i in 0..statnum {
        stats[i] = n_d_drop(n, d, 1)?;
    }

    Ok(())
}

/// Stat generation method for AD&D 2nd ed from CBoN - Method 8 without choice
/// Six 10+1d8 without choice, +1 to INT&WIS (max 18)
fn method_adnd_cbon_8wochoice(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    for i in 0..6 {
        stats[i] = n_d_plus(1, 8, 10)?;
    }

    stats[3] = max(stats[3] + 1, 18);
    stats[4] = max(stats[4] + 1, 18);

    Ok(())
}

/// Stat generation method for AD&D 1st ed - Method 3
/// Best of 6 3d6 for each ability
fn method_adnd1_3(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    for i in 0..6 {
        stats[i] = (0..6)
            .map(|_| n_d(3, 6).unwrap())
            .fold(std::i32::MIN, |a, b| a.max(b));
    }

    Ok(())
}

/// Stat generation method for D&D - Method for Evil Champions
/// 4d6 drop lowest reroll 1's with choice
fn method_dndevilchampion(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    for i in 0..6 {
        stats[i] = n_d_reroll_drop(4, 6, &ONE_ARRAY, 1)?;
    }

    Ok(())
}

/// Stat generation method for CP2020 - Method 2
/// 1d10 reroll 1,2 with choice
fn method_cp2(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
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

/// Stat generation method for CP2020 - Method 3
/// 1d10 reroll 1,2 without choice
fn method_cp4(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
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
fn method_arm1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
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
fn method_arm2(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 4);

    for i in 0..4 {
        stats[i] = n_d(1, 10)? - n_d(1, 10)?;
    }

    Ok(())
}

/// Stat generation method for D&D 3rd ed - Custom average method
/// Six 3d6 with choice. Reroll very bad stats.
fn method_dnd3_2(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    while *stats[0..6].iter().max().unwrap() < 12
        || stats[0..6].iter().fold(0, |sum, val| sum + dnd3mod(*val)) < -2
    {
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
fn method_dnd3_3(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    while *stats[0..6].iter().max().unwrap() < 12
        || stats[0..6].iter().fold(0, |sum, val| sum + dnd3mod(*val)) < -2
    {
        for i in 0..6 {
            stats[i] = n_d(3, 6)?;
        }
    }

    Ok(())
}

/// Stat generation method for D&D 3rd ed - High-powered method
/// Six 5d6 drop lowest two with choice. Reroll below average stats.
fn method_dnd3_4(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    while *stats[0..6].iter().max().unwrap() < 15
        || stats[0..6].iter().fold(0, |sum, val| sum + dnd3mod(*val)) < 2
    {
        for i in 0..6 {
            stats[i] = n_d_drop(4, 6, 1)?;
        }
    }

    stats[0..6].sort();
    stats[0..6].reverse();

    Ok(())
}

/// Stat generation method for D&D 4th ed - Rolling scores method
/// Six 4d6 drop 1 with choice. Reroll bad stats.
fn method_dnd4_1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    while stats[0..6].iter().fold(0, |sum, val| sum + dnd3mod(*val)) < 4 {
        for i in 0..6 {
            stats[i] = n_d_drop(4, 6, 1)?;
        }
    }

    stats[0..6].sort();
    stats[0..6].reverse();

    Ok(())
}

/// Stat generation method for Runequest 6 - Method 1
/// 3d6 for STR, CON, DEX, POW and CHA; 2d6+6 for SIZ and INT
fn method_rq6_1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 7);

    for i in 0..7 {
        stats[i] = match i {
            2 | 4 => n_d_plus(2, 6, 6)?,
            _ => n_d(3, 6)?,
        };
    }

    Ok(())
}

/// Stat generation method for Runequest 6 - Method 1
/// 3d6 for STR, CON, DEX, POW and CHA; 2d6+6 for SIZ and INT
fn method_rq6_2(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 7);

    for i in 0..5 {
        stats[i] = n_d(3, 6)?;
    }

    for i in 5..7 {
        stats[i] = n_d_plus(2, 6, 6)?;
    }

    stats[0..5].sort();
    stats[0..5].reverse();

    stats[5..7].sort();
    stats[5..7].reverse();

    Ok(())
}

#[rustfmt::skip]
lazy_static! {
    // BTreeMap with all methods
    pub static ref METHODSMAP: GenMethodsMap = {
        let mut m = GenMethodsMap::new();

        m.insert("dnd", Method::new(
            "d&d", true, DND_DESC, DND_HELP, |stats| method_nd_wochoice(6, 3, 6, stats),
            &["d&d", "dnd", "ordered"]));
        m.insert("dndrealman", Method::new(
            "d&d", false, DNDREALMAN_DESC,  DNDREALMAN_HELP, |stats| method_nddrop1_wchoice(6, 4, 6, stats),
            &["d&d", "dnd", "choice", "humor"]));
        m.insert("dndcrazieloonie", Method::new(
            "d&d", true, DNDCRAZYLOONIE_DESC, DNDCRAZYLOONIE_HELP, |stats| method_nd_wochoice(6, 1, 20, stats),
            &["d&d", "dnd", "ordered", "humor"]));
        m.insert("dndmunchkin", Method::new(
            "d&d", true, DNDMUNCHKIN_DESC, DNDMUNCHKIN_HELP, |stats| method_array(&ALL25_ARRAY, stats),
            &["d&d", "dnd", "ordered", "pre-set", "humor"]));
        m.insert("dndevilchampion", Method::new(
            "d&d", false, DNDEVILCHAMPION_DESC, DNDEVILCHAMPION_HELP, method_dndevilchampion,
            &["d&d", "dnd", "choice", "humor"]));
        m.insert("dndnewbie", Method::new(
            "d&d", true, DNDNEWBIE_DESC, DNDNEWBIE_HELP, |stats| method_nd_wochoice(6, 3, 6, stats),
            &["d&d", "dnd", "ordered", "humor"]));
        
        m.insert("adnd1", Method::new(
            "d&d", true, ADND1_DESC, ADND1_HELP, |stats| method_nd_wochoice(6, 3, 6, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "ordered"]));
        m.insert("adnd2", Method::new(
            "d&d", true, ADND2_DESC, ADND2_HELP, |stats| method_best_nd_wochoice(6, 3, 6, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "ordered"]));
        m.insert("adnd3", Method::new(
            "d&d", false, ADND3_DESC, ADND3_HELP, |stats| method_nd_wchoice(6, 3, 6, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "choice"]));
        m.insert("adnd4", Method::new(
            "d&d", false, ADND4_DESC, ADND4_HELP, |stats| method_bestlist_nd_wchoice(6, 3, 6, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "choice"]));
        m.insert("adnd5", Method::new(
            "d&d", false, ADND5_DESC, ADND5_HELP, |stats| method_nddrop1_wchoice(6, 4, 6, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "choice"]));

        m.insert("adnd_cbon1", Method::new(
            "d&d", true, ADND_CBON1_DESC, ADND_CBON1_HELP,  |stats| method_rnd_array(&CBON_1_ARRAY, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "cbon", "pre-set", "random_pre-set", "ordered"]));
        m.insert("adnd_cbon2", Method::new(
            "d&d", true, ADND_CBON2_DESC, ADND_CBON2_HELP,  |stats| method_rnd_array(&CBON_2_ARRAY, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "cbon", "pre-set", "random_pre-set", "ordered"]));
        m.insert("adnd_cbon3", Method::new(
            "d&d", true, ADND_CBON3_DESC, ADND_CBON3_HELP,  |stats| method_rnd_array(&CBON_3_ARRAY, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "cbon", "pre-set", "random_pre-set", "ordered"]));
        m.insert("adnd_cbon4", Method::new(
            "d&d", true, ADND_CBON4_DESC, ADND_CBON4_HELP,  |stats| method_rnd_array(&CBON_4_ARRAY, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "cbon", "pre-set", "random_pre-set", "ordered"]));
        m.insert("adnd_cbon5", Method::new(
            "d&d", true, ADND_CBON5_DESC, ADND_CBON5_HELP,  |stats| method_rnd_array(&CBON_5_ARRAY, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "cbon", "pre-set", "random_pre-set", "ordered"]));
        m.insert("adnd_cbon6", Method::new(
            "d&d", true, ADND_CBON6_DESC, ADND_CBON6_HELP,  |stats| method_rnd_array(&CBON_6_ARRAY, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "cbon", "pre-set", "random_pre-set", "ordered"]));
        m.insert("adnd_cbon7", Method::new(
            "d&d", false, ADND_CBON7_DESC, ADND_CBON7_HELP, |stats| method_nd_plus_wchoice(6, 1, 8, 10, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "cbon", "choice"]));
        m.insert("adnd_cbon8", Method::new_w_comment(
            "d&d", false, ADND_CBON8_DESC, ADND_CBON8_HELP, ADND_CBON8_COMMENT, |stats| method_nd_plus_wchoice(6, 1, 8, 10, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "cbon", "choice"]));
        m.insert("adnd_cbon7wochoice", Method::new(
            "d&d", true, ADND_CBON7WOCHOICE_DESC, ADND_CBON7WOCHOICE_HELP, |stats| method_nd_plus_wochoice(6, 1, 8, 10, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "cbon", "ordered"]));
        m.insert("adnd_cbon8wochoice", Method::new(
            "d&d", true, ADND_CBON8WOCHOICE_DESC, ADND_CBON8WOCHOICE_HELP, method_adnd_cbon_8wochoice,
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "cbon", "ordered"]));

        m.insert("adnd_darksun", Method::new(
            "d&d", true, ADND_DS_DESC, ADND_DS_HELP, |stats| method_nd_plus_wochoice(6, 4, 4, 4, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "darksun", "ordered"]));
        m.insert("adnd_darksun1", Method::new(
            "d&d", true, ADND_DS1_DESC, ADND_DS1_HELP, |stats| method_best_nd_wochoice(6, 5, 4, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "darksun", "ordered"]));
        m.insert("adnd_darksun2", Method::new(
            "d&d", false, ADND_DS2_DESC, ADND_DS2_HELP, |stats| method_nd_wchoice(6, 5, 4, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "darksun", "choice"]));
        m.insert("adnd_darksun3", Method::new(
            "d&d", false, ADND_DS3_DESC, ADND_DS3_HELP, |stats| method_bestlist_nd_wchoice(6, 5, 4, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "darksun", "choice"]));
        m.insert("adnd_darksun4", Method::new(
            "d&d", false, ADND_DS4_DESC, ADND_DS4_HELP, |stats| method_nddrop1_wchoice(6, 6, 4, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd2", "ad&d2", "darksun", "choice"]));

        m.insert("adnd1_1", Method::new(
            "d&d", false, ADND1_1_DESC, ADND1_1_HELP, |stats| method_nddrop1_wchoice(6, 4, 6, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd1", "ad&d1", "choice"]));
        m.insert("adnd1_2", Method::new(
            "d&d", false, ADND1_2_DESC, ADND1_2_HELP, |stats| method_bestlist_nd_wchoice(6, 3, 6, stats),
            &["d&d", "dnd", "adnd", "ad&d", "adnd1", "ad&d1", "choice"]));
        m.insert("adnd1_3", Method::new(
            "d&d", true, ADND1_3_DESC, ADND1_3_HELP, method_adnd1_3,
            &["d&d", "dnd", "adnd", "ad&d", "adnd1", "ad&d1", "ordered"]));
        m.insert("adnd1_4", Method::new_w_num_comment(
            "d&d", true, ADND1_4_DESC, ADND1_4_HELP, ADND1_4_COMMENT, |stats| method_nd_wochoice(6, 3, 6, stats), 12,
            &["d&d", "dnd", "adnd", "ad&d", "adnd1", "ad&d1", "multiple", "ordered"]));

        m.insert("dnd3", Method::new(
            "d&d", false, DND3_DESC, DND3_HELP, |stats| method_nddrop1_wchoice(6, 4, 6, stats),
            &["d&d", "dnd", "d20", "d&d3", "dnd3", "choice"]));
        m.insert("dnd3organic", Method::new_w_comment(
            "d&dreroll", true, DND3ORGANIC_DESC, DND3ORGANIC_HELP, DND3_ORGANIC_COMMENT, |stats| method_nddrop1_wochoice(7, 4, 6, stats),
            &["d&d", "dnd", "d20", "d&d3", "dnd3", "reroll", "ordered"]));
        m.insert("dnd3customavg", Method::new(
            "d&d", false, DND3CA_DESC, DND3CA_HELP, method_dnd3_2,
            &["d&d", "dnd", "d20", "d&d3", "dnd3", "choice"]));
        m.insert("dnd3randomavg", Method::new(
            "d&d", true, DND3RA_DESC, DND3RA_HELP, method_dnd3_3,
            &["d&d", "dnd", "d20", "d&d3", "dnd3", "ordered"]));
        m.insert("dnd3highpow", Method::new(
            "d&d", false, DND3HP_DESC, DND3HP_HELP, method_dnd3_4,
            &["d&d", "dnd", "d20", "d&d3", "dnd3", "choice"]));

        m.insert("dnd35", Method::new(
            "d&d", false, DND35_DESC, DND35_HELP, |stats| method_nddrop1_wchoice(6, 4, 6, stats),
            &["d&d", "dnd", "d20", "d&d3.5", "dnd3.5", "choice"]));
        m.insert("dnd35organic", Method::new_w_comment(
            "d&dreroll", true, DND35ORGANIC_DESC, DND35ORGANIC_HELP, DND35_ORGANIC_COMMENT, |stats| method_nddrop1_wochoice(7, 4, 6, stats),
            &["d&d", "dnd", "d20", "d&d3.5", "dnd3.5", "reroll", "ordered"]));
        m.insert("dnd35customavg", Method::new(
            "d&d", false, DND35CA_DESC, DND35CA_HELP, method_dnd3_2,
            &["d&d", "dnd", "d20", "d&d3.5", "dnd3.5", "choice"]));
        m.insert("dnd35randomavg", Method::new(
            "d&d", true, DND35RA_DESC, DND35RA_HELP, method_dnd3_3,
            &["d&d", "dnd", "d20", "d&d3.5", "dnd3.5", "ordered"]));
        m.insert("dnd35highpow", Method::new(
            "d&d", false, DND35HP_DESC, DND35HP_HELP, method_dnd3_4,
            &["d&d", "dnd", "d20", "d&d3.5", "dnd3.5", "choice"]));
        m.insert("dnd35elite", Method::new(
            "d&d", false, DND35ELITE_DESC, DND35ELITE_HELP, |stats| method_array(&DND35_ARRAY, stats),
            &["d&d", "dnd", "d20", "d&d3.5", "dnd3.5", "pre-set", "choice"]));

        m.insert("dnd4", Method::new(
            "d&d", false, DND4_DESC, DND4_HELP, method_dnd4_1,
            &["d&d", "dnd", "d&d4", "dnd4", "choice"]));
        m.insert("dnd4standard", Method::new(
            "d&d", false, DND4STANDARD_DESC, DND4STANDARD_HELP, |stats| method_array(&DND4_ARRAY, stats),
            &["d&d", "dnd", "d&d4", "dnd4", "pre-set", "choice"]));

        m.insert("dnd5", Method::new(
            "d&d", false, DND5_DESC, DND5_HELP, |stats| method_nddrop1_wchoice(6, 4, 6, stats),
            &["d&d", "dnd", "d&d5", "dnd5", "choice"]));
        m.insert("dnd5standard", Method::new(
            "d&d", false, DND5STANDARD_DESC, DND5STANDARD_HELP, |stats| method_array(&DND35_ARRAY, stats),
            &["d&d", "dnd", "d&d5", "dnd5", "pre-set", "choice"]));

        m.insert("pfstandard", Method::new(
            "d&d", false, PFSTANDARD_DESC, PFSTANDARD_HELP, |stats| method_nddrop1_wchoice(6, 4, 6, stats),
            &["d&d", "dnd", "d20", "pathfinder", "pathfinder1", "pf", "pf1", "choice"]));
        m.insert("pfclassic", Method::new(
            "d&d", false, PFCLASSIC_DESC, PFCLASSIC_HELP, |stats| method_nd_wchoice(6, 3, 6, stats),
            &["d&d", "dnd", "d20", "pathfinder", "pathfinder1", "pf", "pf1", "choice"]));
        m.insert("pfheroic", Method::new(
            "d&d", false, PFHEROIC_DESC, PFHEROIC_HELP, |stats| method_nd_plus_wchoice(6, 2, 6, 6, stats),
            &["d&d", "dnd", "d20", "pathfinder", "pathfinder1", "pf", "pf1", "choice"]));

        m.insert("pathfinder2", Method::new(
            "d&d", false, PF2_DESC, PF2_HELP, |stats| method_nddrop1_wchoice(6, 4, 6, stats),
            &["pathfinder", "pathfinder2", "pf", "pf2", "choice"]));

        m.insert("starfinder", Method::new(
            "d&d", false, SF_DESC, SF_HELP, |stats| method_nddrop1_wchoice(6, 4, 6, stats),
            &["starfinder", "sf", "choice"]));
        m.insert("sffocused", Method::new(
            "d&d", false, SFFOCUSED_DESC, SFFOCUSED_HELP, |stats| method_array(&SFFOCUSED_ARRAY, stats),
            &["starfinder", "sf", "pre-set", "choice"]));
        m.insert("sfsplit", Method::new(
            "d&d", false, SFSPLIT_DESC, SFSPLIT_HELP, |stats| method_array(&SFSPLIT_ARRAY, stats),
            &["starfinder", "sf", "pre-set", "choice"]));
        m.insert("sfversatile", Method::new(
            "d&d", false, SFVERSATILE_DESC, SFVERSATILE_HELP, |stats| method_array(&SFVERSATILE_ARRAY, stats),
            &["starfinder", "sf", "pre-set", "choice"]));

        m.insert("cp2013_1", Method::new(
            "cyberpunk-cp", true, CP2013_1_DESC, CP2013_1_HELP, |stats| method_nd_wochoice(1, 9, 10, stats),
            &["cyberpunk", "cp", "cyberpunk2013", "cp2013", "character_points", "choice"]));
        m.insert("cp2013_2", Method::new(
            "cyberpunk-cp", true, CP2013_2_DESC, CP2013_2_HELP, |stats| method_nd_plus_wochoice(1, 6, 10, 30, stats),
            &["cyberpunk", "cp", "cyberpunk2013", "cp2013", "character_points", "choice"]));
        m.insert("cp2020_1", Method::new(
            "cyberpunk-cp", true, CP2020_1_DESC, CP2020_1_HELP, |stats| method_nd_wochoice(1, 9, 10, stats),
            &["cyberpunk", "cp", "cyberpunk2020", "cp2020", "character_points", "choice"]));
        m.insert("cp2020_2", Method::new(
            "cyberpunk-stat", false, CP2020_2_DESC, CP2020_2_HELP, method_cp2,
            &["cyberpunk", "cp", "cyberpunk2020", "cp2020", "choice"]));
        m.insert("cp2020_3", Method::new(
            "cyberpunk-stat", true, CP2020_3_DESC, CP2020_3_HELP, method_cp4,
            &["cyberpunk", "cp", "cyberpunk2020", "cp2020", "ordered"]));

        m.insert("cyberspace1", Method::new(
            "cyberspace", false, CYBERSPACE_1_DESC, CYBERSPACE_1_HELP, |stats| method_nd_wchoice(11, 1, 100, stats),
            &["cyberpunk", "cp", "cyberspace", "choice"]));
        m.insert("cyberspace2", Method::new(
            "cyberspace", true, CYBERSPACE_2_DESC, CYBERSPACE_2_HELP, |stats| method_nd_wochoice(11, 1, 100, stats),
            &["cyberpunk", "cp", "cyberspace", "ordered"]));

        m.insert("arsmagica1", Method::new(
            "arsmagica", false, ARM1_DESC, ARM1_HELP, method_arm1,
            &["arsmagica", "arm", "choice"]));
        m.insert("arsmagica2", Method::new(
            "arsmagica", true, ARM2_DESC, ARM2_HELP, method_arm2,
            &["arsmagica", "arm", "ordered"]));

        m.insert("wh40k", Method::new(
            "warhammer", true, WH40K_DESC, WH40K_HELP, |stats| method_nd_wochoice(9, 2, 10, stats),
            &["wh", "warhammer", "wh40k", "warhammer40k", "ordered"]));
        m.insert("wh40k_reroll", Method::new(
            "warhammer-reroll", true, WH40KREROLL_DESC, WH40KREROLL_HELP, |stats| method_nd_wochoice(10, 2, 10, stats),
            &["wh", "warhammer", "wh40k", "warhammer40k", "reroll", "ordered"]));
        m.insert("wh40k_choice", Method::new(
            "warhammer", false, WH40KCHOICE_DESC, WH40KCHOICE_HELP, |stats| method_nd_wchoice(9, 2, 10, stats),
            &["wh", "warhammer", "wh40k", "warhammer40k", "choice"]));
        m.insert("wh40k_choicereroll", Method::new(
            "warhammer-reroll", false, WH40KCHOICEREROLL_DESC, WH40KCHOICEREROLL_HELP, |stats| method_nd_wchoice(10, 2, 10, stats),
            &["wh", "warhammer", "wh40k", "warhammer40k", "reroll", "choice"]));

        m.insert("uesrpg", Method::new(
            "uesrpg", true, UESRPG_DESC, UESRPG_HELP, |stats| method_nd_wochoice(9, 2, 10, stats),
            &["uesrpg", "ordered"]));
        m.insert("uesrpg_reroll", Method::new(
            "uesrpg-reroll", true, UESRPGREROLL_DESC, UESRPGREROLL_HELP, |stats| method_nd_wochoice(10, 2, 10, stats),
            &["uesrpg", "reroll", "ordered"]));
        m.insert("uesrpg_choice", Method::new(
            "uesrpg", false, UESRPGCHOICE_DESC, UESRPGCHOICE_HELP, |stats| method_nd_wchoice(9, 2, 10, stats),
            &["uesrpg", "choice"]));
        m.insert("uesrpg_choicereroll", Method::new(
            "uesrpg-reroll", false, UESRPGCHOICEREROLL_DESC, UESRPGCHOICEREROLL_HELP, |stats| method_nd_wchoice(10, 2, 10, stats),
            &["uesrpg", "reroll", "choice"]));            
        
        m.insert("swn", Method::new_w_comment(
            "d&d", true, SWN_DESC, SWN_HELP, SWN_COMMENT, |stats| method_nd_wochoice(6, 3, 6, stats),
            &["swn", "osr", "ordered"]));
        m.insert("swnstandard", Method::new(
            "d&d", false, SWNSTANDARD_DESC, SWNSTANDARD_HELP,  |stats| method_array(&SWN_ARRAY, stats),
            &["swn", "osr", "pre-set", "choice"]));

        m.insert("rq6", Method::new(
            "runequest", true, RQ6_DESC, RQ6_HELP, method_rq6_1,
            &["runequest", "rq", "runequest6", "rq6", "ordered"]));
        m.insert("rq6_choice", Method::new_w_comment(
            "runequest", false, RQ6CHOICE_DESC, RQ6CHOICE_HELP, RQ6CHOICE_COMMENT, method_rq6_2,
            &["runequest", "rq", "runequest6", "rq6", "choice"]));

        m
    };

    // BTreeSet with all tags
    pub static ref TAGSET: Tags = {
        let mut t = Tags::new();
        for (_, m) in METHODSMAP.iter() {
            t.extend(m.get_tags());
        }

        t
    };

    // 2D Arrays for AD&D CBoN methods
    static ref CBON_1_ARRAY : Array2<IntValue> = array![
        [13, 11, 10, 10, 17, 5],
        [10, 10, 11, 9, 16, 13],
        [5, 14, 16, 9, 16, 10],
        [7, 13, 16, 12, 16, 12],
        [15, 6, 9, 9, 16, 7],
        [7, 13, 15, 11, 16, 13]
    ];

    static ref CBON_2_ARRAY : Array2<IntValue> = array![
        [17, 12, 11, 12, 17, 12],
        [13, 14, 12, 10, 16, 12],
        [12, 7, 13, 16, 16, 12],
        [11, 11, 12, 14, 16, 13],
        [8, 12, 8, 10, 18, 9],
        [7, 13, 10, 17, 16, 11]
    ];

    static ref CBON_3_ARRAY : Array2<IntValue> = array![
        [6, 9, 10, 17, 16, 6],
        [10, 13, 14, 17, 16, 10],
        [9, 13, 15, 17, 16, 11],
        [6, 11, 14, 17, 16, 9],
        [5, 9, 13, 16, 16, 6],
        [8, 10, 11, 18, 16, 13]
    ];        

    static ref CBON_4_ARRAY : Array2<IntValue> = array![
        [12, 14, 14, 16, 17, 12],
        [14, 14, 15, 17, 16, 14],
        [10, 14, 14, 17, 16, 12],
        [11, 12, 12, 16, 16, 12],
        [13, 13, 15, 18, 16, 13],
        [11, 15, 13, 16, 17, 11]
    ];
    
    static ref CBON_5_ARRAY : Array2<IntValue> = array![
        [9, 11, 14, 17, 16, 11],
        [10, 15, 16, 17, 16, 13],
        [10, 11, 13, 18, 16, 10],
        [12, 14, 15, 16, 16, 14],
        [12, 14, 14, 17, 16, 13],
        [11, 13, 15, 17, 17, 12]
    ];
    
    static ref CBON_6_ARRAY : Array2<IntValue> = array![
        [8, 10, 16, 17, 16, 8],
        [8, 17, 8, 18, 16, 8],
        [9, 11, 12, 13, 16, 10],
        [8, 16, 11, 14, 16, 8],
        [9, 15, 10, 14, 16, 11],
        [10, 8, 15, 15, 17, 9]
    ];    
}
