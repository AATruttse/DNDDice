// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use num::iter::range;
use std::cmp::{max, min};
use regex::Regex;

use crate::arithmetic::{Arythmetic, process_arithmetic};

use crate::dices::{IntValue, n_d_reroll_drop_crop_plus};

use crate::errors::{DiceError, errorln, process_dice_code_error};

use crate::init::OPT;
use crate::log::{log, logln, log_codes, log_method, log_roll};
use crate::methods::METHODSMAP;
use crate::output::{output, outputln};
use crate::render::{format_dice_str, render_codes, render_roll, render_stats};
use crate::statlists::{StatList, STATLISTSMAP};
use crate::strings::{DELIMITER, ADVDISADV_ERROR_MSG, UNKNOWNSTATLIST_ERROR_MSG};

/// process stat generation method, uses all_stat for statistics
pub fn process_method(method_name: &str, all_stats: &mut Vec<IntValue>, idx: usize, num: usize) -> Option<()> {
    let mut stat : Vec<IntValue> = Vec::<IntValue>::new();

    if OPT.is_debug() {
        println!("Method: {}", method_name);
    }

    match METHODSMAP.get(method_name) {
        Some(method) => {
            let statlist: &StatList = STATLISTSMAP.
                get(method.get_statlist()).
                expect(UNKNOWNSTATLIST_ERROR_MSG);

            stat.resize(statlist.len(), 0);

            log_and_render_title(idx, num, method.get_desc());

            let n = method.get_num();

            for i in 1..(n + 1) {
                method.get_method()(&mut stat).unwrap();

                if OPT.log > 0 {
                    log_method(n > 1, method.is_ordered(), i, &stat, statlist);
                }

                if OPT.is_debug() || OPT.verbose > 0 || !OPT.is_collect_stat() {
                    render_stats(n > 1, method.is_ordered(), i, &stat, statlist);
                }

                all_stats.extend(stat.clone());
            }

            if (OPT.debug || OPT.verbose > 0 || !OPT.is_collect_stat()) && !OPT.numbers_only && idx == num - 1 {
                outputln(method.get_comment());
            }
        },
        None => {
            if OPT.is_debug() {
                println!("Can't found method {}", method_name);
            }
            return None
        }
    }

    Some(())
}

/// process any dice roll, uses all_stat for statistics
pub fn process_dices(all_stats: &mut Vec<IntValue>, idx: usize, num: usize) {

    log_and_show_dice_num(idx, num);

    if OPT.dicecodes.is_empty() ||
       OPT.dicecodes[0].is_empty()
    {
        process_keys(all_stats);
    }
    else {
        match process_codes(&OPT.dicecodes, all_stats) {
            Ok(_) => {},
            Err(e) => process_dice_code_error(&OPT.dicecodes, e, true)
        }
    }
}

/// process dice roll from command line keys
fn process_keys(all_stats: &mut Vec<IntValue>) {

    let reroll = match parse_reroll_code(&OPT.reroll) {
        Some(r) => r,
        None => {
           errorln(&DiceError::BadRerollCode.to_string());
            std::process::exit(1);  
        }
    };

    let res = match (OPT.advantage, OPT.disadvantage) {
        (true, true) => {
            errorln(&ADVDISADV_ERROR_MSG);
            std::process::exit(1);
        },
        (true, false) => max(
                process_roll(false, true, OPT.dices_num, OPT.dice, &reroll, OPT.plus, OPT.minus, OPT.drop, OPT.crop),
                process_roll(false, true, OPT.dices_num, OPT.dice, &reroll, OPT.plus, OPT.minus, OPT.drop, OPT.crop)
        ),
        (false, true) => min(
            process_roll(false, true, OPT.dices_num, OPT.dice, &reroll, OPT.plus, OPT.minus, OPT.drop, OPT.crop),
            process_roll(false, true, OPT.dices_num, OPT.dice, &reroll, OPT.plus, OPT.minus, OPT.drop, OPT.crop)
        ),
        (false, false) => {
            process_roll(false, false, OPT.dices_num, OPT.dice, &reroll, OPT.plus, OPT.minus, OPT.drop, OPT.crop)
        }                
    };

    if OPT.advantage || OPT.disadvantage {
        if OPT.is_debug() || OPT.verbose > 0
        {
            log_and_render_roll(false,
             false,
             OPT.dices_num,
             OPT.dice,
             &reroll,
             OPT.plus as i32 - OPT.minus as i32,
             OPT.drop,
             OPT.crop,
             OPT.advantage,
             OPT.disadvantage
            );
        }

        render_roll(false, !OPT.numbers_only && OPT.verbose > 0, false, res, true);
        log_roll(false, false, res);
    }

    all_stats.push(res);
}

/// process dice roll from dice codes
pub fn process_codes(dicecodes: &Vec<String>, all_stats: &mut Vec<IntValue>)-> Result<(), DiceError> {
    for dicecode in dicecodes {
        if !dicecode.is_empty() {
            // parse dice codes with regular expression
            let dice_regex: &str = r"([-+/*%^])?(?:(\d*)d(\d*)(?:(?:reroll|r)((?:\d+)(?:(?:(?:(?:,|..)(?:\d+))+)?)))?(?:(?:drop|d)(\d+)(?:crop|c)(\d+)|(?:(?:drop|d)(\d+)|(?:crop|c)(\d+)|(?:greatest|g)(\d+)|(?:lowest|l)(\d+)))?(A|D|advantage|disadvantage)?(?:(?:plus|p)(\d+))?(?:(?:minus|m)(\d+))?|(\d+))";
            let re = Regex::new(
                dice_regex).
                unwrap();

            // process regexp;
            let dices = re.captures_iter(&dicecode).into_iter();
            let dices_num = re.captures_iter(&dicecode).into_iter().count();

            // parse individual dice codes
            let results_vec: Vec<(&str, IntValue, bool)> = dices.
                enumerate().
                map(|(num, it)| 
                    process_code(dices_num > 1,
                                num,
                                &it.iter().
                                    map(|p| p.map_or("", |m| m.as_str())).
                                    collect::<Vec<&str>>())
                ).
                collect::<Result<Vec<(&str, IntValue, bool)>, DiceError>>()?;

            let has_advantages = results_vec.iter().fold(false, |res, (_, _, val)| res || *val);

            let dices_vec: Vec<Arythmetic> = results_vec.
                iter().
                map(|it| {
                    match it {
                        &(s, v, _) => (s, v)
                    }
                }).collect();

            if OPT.is_debug() {
                println!("{:?}", dices_vec);
            }
            
            if dices_vec.is_empty() {
                return Err(DiceError::BadCode);
            }

            // process parsed dice codes
            let res = process_arithmetic(&dices_vec);

            render_codes(dices_num, has_advantages, dicecode, res);
            log_codes(dices_num, dicecode, res);

            all_stats.push(res);
        }
    }

    Ok(())
}

/// process single code from parsed regular expression
fn process_code<'a>(
     is_several_rolls: bool,
     idx: usize,
     params: &Vec<&'a str>)
      -> Result<(&'a str, IntValue, bool), DiceError> {
    if OPT.is_debug() {
        println!("dice: {} - {:?}", idx, params);
    }

    if params.len() != 15
    {
        return Err(DiceError::BadDecryption);
    }

    if (idx == 0 && !params[1].is_empty()) ||
       (idx > 0 && params[1].is_empty()) {
        return Err(DiceError::BadCode);
    }

    if !params[14].is_empty()
    {
        let c: IntValue = params[14].parse().unwrap();
        return Ok((params[1], c, false));
    }


    let n: usize = match params[2] {"" => 1, x => x.parse().unwrap()};
    let d: usize = match params[3] {"" => 6, x => x.parse().unwrap()};

    let reroll = match parse_reroll_code(params[4]) {
        Some(r) => r,
        None => return Err(DiceError::BadCode)
    };

    let plus: usize = match params[12] {"" => 0, x => x.parse().unwrap()};
    let minus: usize = match params[13] {"" => 0, x => x.parse().unwrap()};    

    let drop: usize = match params[5] {
        "" => match params[7] {
            "" => match params[9] {
                "" => 0,
                x => {
                    let g: usize = x.parse().unwrap();
                    n - g
                }
            },
            x => x.parse().unwrap()
        },
        x => x.parse().unwrap()
    };

    let crop: usize = match params[6] {
        "" => match params[8] {
            "" => match params[10] {
                "" => 0,
                x => {
                    let l: usize = x.parse().unwrap();
                    n - l
                }
            },
            x => x.parse().unwrap()
        },
        x => x.parse().unwrap()
    };
    
    match params[11] {
        "A" | "advantage" => {
            Ok((params[1], max(
                process_roll(is_several_rolls, true, n, d, &reroll, plus, minus, drop, crop),
                process_roll(is_several_rolls, true, n, d, &reroll, plus, minus, drop, crop)
            ), true))
        },
        "D" | "disadvantage" => {
            Ok((params[1], min(
                process_roll(is_several_rolls, true, n, d, &reroll, plus, minus, drop, crop),
                process_roll(is_several_rolls, true, n, d, &reroll, plus, minus, drop, crop)
            ), true))
        },
        _ => Ok((params[1], process_roll(is_several_rolls, false, n, d, &reroll, plus, minus, drop, crop), false))
    }
}

/// process dice roll with given parameters
fn process_roll(
    is_several_rolls: bool,
    is_advantage: bool, 
    n: usize,
    d: usize,
    reroll: &[usize],
    plus: usize,
    minus: usize,
    drop: usize,
    crop: usize) -> IntValue {

    let add = plus as IntValue - minus as IntValue;

    let show_dice_code = log_and_render_roll(is_several_rolls, is_advantage, n, d, reroll, add, drop, crop, false, false);
    
    let res = match n_d_reroll_drop_crop_plus(n,
        d,
        reroll,
        add,
        drop,
        crop
    ) {
        Ok(x) => x,
        Err(e) => {
            errorln(&e.to_string());
            std::process::exit(1);
        }
    };

    render_roll(is_several_rolls, show_dice_code, is_advantage, res, false);
    log_roll(is_several_rolls, is_advantage, res);

    res
}

/// parse reroll string
fn parse_reroll_code (reroll_str: &str) -> Option<Vec<usize>> {
    let rerolls : Vec<&str> = match reroll_str.is_empty() {
        true => Vec::new(),
        false => reroll_str.split(",").collect()
    };
   
    let mut reroll : Vec<usize> = Vec::with_capacity(rerolls.len());
    for r in rerolls {
        match r.split_once("..") {
            Some ((first, last)) => match (first.parse::<usize>(), last.parse::<usize>()) {
                (Ok(f), Ok(l)) => reroll.extend(range(f, l+1)),
                _ => return None
            },
            None => match r.parse::<usize>() {
                Ok(num) => reroll.push(num),
                Err(_) => {
                    return None;
                }
            }
        }
    }

    Some(reroll)
}

/// logs and shows dice code
fn log_and_render_roll(
    is_several_rolls: bool,
    is_advantage_part: bool,
    n: usize,
    d: usize,
    reroll: &[usize],
    add: IntValue,
    drop: usize,
    crop: usize,
    adv: bool,
    disadv: bool) -> bool {

    let dice_str = format_dice_str(is_several_rolls, n, d, reroll, add, drop, crop, adv, disadv);

    let mut show_dice_code = false;
    if OPT.is_debug() ||
        (OPT.verbose > 0 && !is_several_rolls && !is_advantage_part) ||
        OPT.verbose > 1 {
        show_dice_code = true;
        output(&dice_str);
    }

    if (OPT.log > 0 && !is_several_rolls && !is_advantage_part) ||
        OPT.log > 1 {
        log(&dice_str);
    }

    return show_dice_code;
}

/// log and show title
pub fn log_and_render_title(idx: usize, num: usize, desc: &str) {
    if OPT.log > 0 {
        logln(DELIMITER);
        
        if num > 1 {
            let idx_str = format!("{}) ", idx + 1);
            log(&idx_str);
        }
        log(&OPT.method);
    }

    if OPT.show_method { 
        if idx == 0 {
            outputln(desc);
        }
        if OPT.log > 0 {
            log(" - ");
            logln(desc);
        }
    }
    else {
        logln("");
    }

    if OPT.show_number {
        let idx_str = format!("{}) ", idx + 1);
        output(&idx_str);
    }
}

pub fn log_and_show_dice_num(idx: usize, num: usize) {
    let idx_str = format!("{}) ", idx + 1);
    if num > 1 {
        log(&idx_str);
    }

    if OPT.show_number {
        output(&idx_str);
    }
}