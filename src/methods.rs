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

use crate::dices::IntValue;
use crate::dices::n_d;
use crate::dices::n_d_drop;
use crate::dices::n_d_plus;
use crate::dices::n_d_reroll_drop;

use crate::errors::DiceError;

use crate::method::Method;

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
        _ => 0
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


/// Stat generation method for AD&D 2nd ed - Method 1
/// 3d6 without choice
fn method_adnd1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    for i in 0..6 {
        stats[i] = n_d(3, 6)?;
    }

    Ok(())
}

/// Stat generation method for AD&D 2nd ed - Method 2
/// Best of two 3d6 without choice
fn method_adnd2(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    for i in 0..6 {
        stats[i] = max(n_d(3, 6)?, n_d(3, 6)?);
    }

    Ok(())
}

/// Stat generation method for AD&D 2nd ed - Method 3
/// 3d6 with choice
fn method_adnd3(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    for i in 0..6 {
        stats[i] = n_d(3, 6)?;
    }

    stats[0..6].sort();
    stats[0..6].reverse();

    Ok(())
}

/// Stat generation method for AD&D 2nd ed - Method 4
/// Best 6 of 12 3d6 with choice
fn method_adnd4(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
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

/// Stat generation method for AD&D 2nd ed - Method 5
/// 4d6 drop lowest with choice
fn method_adnd5(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    for i in 0..6 {
        stats[i] = n_d_drop(4, 6, 1)?;
    }

    stats[0..6].sort();
    stats[0..6].reverse();

    Ok(())
}

/// Stat generation method for AD&D 2nd ed from CBoN - Method 7
/// Six 10+1d8 with choice
fn method_adnd_cbon_7(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    for i in 0..6 {
        stats[i] = n_d_plus(1, 8, 10)?;
    }

    stats[0..6].sort();
    stats[0..6].reverse();

    Ok(())
}

/// Stat generation method for AD&D 2nd ed from CBoN - Method 7 without choice
/// Six 10+1d8 without choice
fn method_adnd_cbon_7wochoice(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    for i in 0..6 {
        stats[i] = n_d_plus(1, 8, 10)?;
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
        stats[i] = (0..6).map(|_| n_d(3, 6).unwrap()).fold(std::i32::MIN, |a,b| a.max(b));
    }

    Ok(())
}

/// Stat generation method for D&D - Method for Crazy Loonies
/// d20 without choice
fn method_dndloonie(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 6);

    for i in 0..6 {
        stats[i] = n_d(1, 20)?;
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

/// Stat generation method for CP2020 - Method 1
/// 9d10 character points
fn method_cp1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 1);

    stats[0] = n_d(9, 10)?;

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

/// Stat generation method for CP2013 - Method 1
/// 9d10 character points
fn method_cp3(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 1);

    stats[0] = n_d_plus(6, 10, 30)?;

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

/// Stat generation method for Cyberspace - Method 1
/// 1d100 with choice
fn method_cyberspace1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
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
fn method_cyberspace2(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 11);

    for i in 0..11 {
            stats[i] = n_d(1, 100)?;
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

/// Stat generation method for Pathfinder - Heroic method
/// 2d6+6 with choice
fn method_pf1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
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
fn method_dnd3_1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 7);

    for i in 0..7 {
        stats[i] = n_d_drop(4, 6, 1)?;
    }

    Ok(())
}

/// Stat generation method for D&D 3rd ed - Custom average method
/// Six 3d6 with choice. Reroll very bad stats.
fn method_dnd3_2(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
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
fn method_dnd3_3(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
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
fn method_dnd3_4(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
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

/// Stat generation method for D&D 4th ed - Rolling scores method
/// Six 4d6 drop 1 with choice. Reroll bad stats.
fn method_dnd4_1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
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

/// Stat generation method for WH40K - Method 1
/// 2d10 for each characteristic without choice
fn method_wh40k1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 9);

    for i in 0..9 {
        stats[i] = n_d(2, 10)?;
    }

    Ok(())
}

/// Stat generation method for WH40K - Method 2
/// 2d10 for each characteristic without choice, reroll one
fn method_wh40k2(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 10);

    for i in 0..10 {
        stats[i] = n_d(2, 10)?;
    }

    Ok(())
}

/// Stat generation method for WH40K - Method 3
/// 2d10 for each characteristic with choice
fn method_wh40k3(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
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
fn method_wh40k4(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 10);

    for i in 0..10 {
        stats[i] = n_d(2, 10)?;
    }

    stats[0..10].sort();
    stats[0..10].reverse();

    Ok(())
}

/// Stat generation method for Runequest 6 - Method 1
/// 3d6 for STR, CON, DEX, POW and CHA; 2d6+6 for SIZ and INT
fn method_rq6_1(stats: &mut Vec<IntValue>) -> Result<(), DiceError> {
    vec_checksize(stats, 7);

    for i in 0..7 {
        stats[i] = match i {
            2 | 4 => n_d_plus(2, 6, 6)?,
            _ => n_d(3, 6)?
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

lazy_static! {
    // BTreeMap with all methods
    pub static ref METHODSMAP: GenMethodsMap = {
        let mut m = GenMethodsMap::new();

        m.insert("dnd", Method::new("d&d", true, DND_DESC, DND_HELP, method_adnd1));

        m.insert("dndrealman", Method::new("d&d", true, DNDREALMAN_DESC,  DNDREALMAN_HELP, method_adnd5));
        m.insert("dndcrazieloonie", Method::new("d&d", true, DNDCRAZYLOONIE_DESC, DNDCRAZYLOONIE_HELP, method_dndloonie));
        m.insert("dndmunchkin", Method::new("d&d", true, DNDMUNCHKIN_DESC, DNDMUNCHKIN_HELP,  |stats| method_array(&ALL25_ARRAY, stats)));
        m.insert("dndevilchampion", Method::new("d&d", true, DNDEVILCHAMPION_DESC, DNDEVILCHAMPION_HELP, method_dndevilchampion));
        m.insert("dndnewbie", Method::new("d&d", true, DNDNEWBIE_DESC, DNDNEWBIE_HELP, method_adnd1));
        
        m.insert("adnd1", Method::new("d&d", true, ADND1_DESC, ADND1_HELP, method_adnd1));
        m.insert("adnd2", Method::new("d&d", true, ADND2_DESC, ADND2_HELP, method_adnd2));
        m.insert("adnd3", Method::new("d&d", false, ADND3_DESC, ADND3_HELP, method_adnd3));
        m.insert("adnd4", Method::new("d&d", false, ADND4_DESC, ADND4_HELP, method_adnd4));
        m.insert("adnd5", Method::new("d&d", false, ADND5_DESC, ADND5_HELP, method_adnd5));

        m.insert("adnd_cbon1", Method::new("d&d", true, ADND_CBON1_DESC, ADND_CBON1_HELP,  |stats| method_rnd_array(&CBON_1_ARRAY, stats)));
        m.insert("adnd_cbon2", Method::new("d&d", true, ADND_CBON2_DESC, ADND_CBON2_HELP,  |stats| method_rnd_array(&CBON_2_ARRAY, stats)));
        m.insert("adnd_cbon3", Method::new("d&d", true, ADND_CBON3_DESC, ADND_CBON3_HELP,  |stats| method_rnd_array(&CBON_3_ARRAY, stats)));
        m.insert("adnd_cbon4", Method::new("d&d", true, ADND_CBON4_DESC, ADND_CBON4_HELP,  |stats| method_rnd_array(&CBON_4_ARRAY, stats)));
        m.insert("adnd_cbon5", Method::new("d&d", true, ADND_CBON5_DESC, ADND_CBON5_HELP,  |stats| method_rnd_array(&CBON_5_ARRAY, stats)));
        m.insert("adnd_cbon6", Method::new("d&d", true, ADND_CBON6_DESC, ADND_CBON6_HELP,  |stats| method_rnd_array(&CBON_6_ARRAY, stats)));
        m.insert("adnd_cbon7", Method::new("d&d", false, ADND_CBON7_DESC, ADND_CBON7_HELP, method_adnd_cbon_7));
        m.insert("adnd_cbon8", Method::new_w_comment("d&d", false, ADND_CBON8_DESC, ADND_CBON8_HELP, ADND_CBON8_COMMENT, method_adnd_cbon_7));
        m.insert("adnd_cbon7wochoice", Method::new("d&d", true, ADND_CBON7WOCHOICE_DESC, ADND_CBON7WOCHOICE_HELP, method_adnd_cbon_7wochoice));
        m.insert("adnd_cbon8wochoice", Method::new("d&d", true, ADND_CBON8WOCHOICE_DESC, ADND_CBON8WOCHOICE_HELP, method_adnd_cbon_8wochoice));

        m.insert("adnd1_1", Method::new("d&d", false, ADND1_1_DESC, ADND1_1_HELP, method_adnd5));
        m.insert("adnd1_2", Method::new("d&d", false, ADND1_2_DESC, ADND1_2_HELP, method_adnd4));
        m.insert("adnd1_3", Method::new("d&d", true, ADND1_3_DESC, ADND1_3_HELP, method_adnd1_3));
        m.insert("adnd1_4", Method::new_w_num_comment("d&d", true, ADND1_4_DESC, ADND1_4_HELP, ADND1_4_COMMENT, method_adnd1, 12));

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
        m.insert("dnd35elite", Method::new("d&d", false, DND35ELITE_DESC, DND35ELITE_HELP, |stats| method_array(&DND35_ARRAY, stats)));

        m.insert("dnd4", Method::new("d&d", false, DND4_DESC, DND4_HELP, method_dnd4_1));
        m.insert("dnd4standard", Method::new("d&d", false, DND4STANDARD_DESC, DND4STANDARD_HELP, |stats| method_array(&DND4_ARRAY, stats)));

        m.insert("dnd5", Method::new("d&d", false, DND5_DESC, DND5_HELP, method_adnd5));
        m.insert("dnd5standard", Method::new("d&d", false, DND5STANDARD_DESC, DND5STANDARD_HELP, |stats| method_array(&DND35_ARRAY, stats)));

        m.insert("pfstandard", Method::new("d&d", false, PFSTANDARD_DESC, PFSTANDARD_HELP, method_adnd5));
        m.insert("pfclassic", Method::new("d&d", false, PFCLASSIC_DESC, PFCLASSIC_HELP, method_adnd3));
        m.insert("pfheroic", Method::new("d&d", false, PFHEROIC_DESC, PFHEROIC_HELP, method_pf1));

        m.insert("pathfinder2", Method::new("d&d", false, PF2_DESC, PF2_HELP, method_adnd5));

        m.insert("starfinder", Method::new("d&d", false, SF_DESC, SF_HELP, method_adnd5));
        m.insert("sffocused", Method::new("d&d", false, SFFOCUSED_DESC, SFFOCUSED_HELP, |stats| method_array(&SFFOCUSED_ARRAY, stats)));
        m.insert("sfsplit", Method::new("d&d", false, SFSPLIT_DESC, SFSPLIT_HELP, |stats| method_array(&SFSPLIT_ARRAY, stats)));
        m.insert("sfversatile", Method::new("d&d", false, SFVERSATILE_DESC, SFVERSATILE_HELP, |stats| method_array(&SFVERSATILE_ARRAY, stats)));

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
        
        m.insert("swn", Method::new_w_comment("d&d", true, SWN_DESC, SWN_HELP, SWN_COMMENT, method_adnd1));
        m.insert("swnstandard", Method::new("d&d", false, SWNSTANDARD_DESC, SWNSTANDARD_HELP,  |stats| method_array(&SWN_ARRAY, stats)));

        m.insert("rq6", Method::new("runequest", true, RQ6_DESC, RQ6_HELP, method_rq6_1));
        m.insert("rq6_choice", Method::new_w_comment("runequest", false, RQ6CHOICE_DESC, RQ6CHOICE_HELP, RQ6CHOICE_COMMENT, method_rq6_2));


        m
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


