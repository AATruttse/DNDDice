// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// use itertools::Itertools;
use num::iter::range;
use regex::Regex;

use crate::arithmetic::{Arythmetic, process_arithmetic};

use crate::dices::{IntValue, n_d_reroll_drop_crop_plus};

use crate::errors::DiceError;

use crate::init::OPT;
use crate::log::{log, logln, log_codes, log_method, log_roll};
use crate::methods::METHODSMAP;
use crate::render::{format_dice_str, render_codes, render_roll, render_stats};
use crate::statlists::{StatList, STATLISTSMAP};
use crate::strings::{DELIMITER, DICECODES_HELP_MSG, UNKNOWNSTATLIST_ERROR_MSG};

/// process stat generation method, uses all_stat for statistics
pub fn process_method(method_name: &str, all_stats: &mut Vec<IntValue>, idx: usize, num: usize) -> Option<()> {
    let mut stat : Vec<IntValue> = Vec::<IntValue>::new();

    if OPT.debug {
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

                if OPT.debug || OPT.verbose > 0 || !OPT.is_collect_stat() {
                    render_stats(n > 1, method.is_ordered(), i, &stat, statlist);
                }

                all_stats.extend(stat.clone());
            }

            if (OPT.debug || OPT.verbose > 0 || !OPT.is_collect_stat()) && !OPT.numbers_only && idx == num - 1 {
                println!("{}", method.get_comment());
            }
        },
        None => {
            if OPT.debug {
                println!("Can't found method {}", method_name);
            }
            return None
            //cant_find_method(&OPT.method)
        }
    }

    Some(())
}

/// process any dice roll, uses all_stat for statistics
pub fn process_dices(all_stats: &mut Vec<IntValue>, idx: usize, num: usize) {

    let idx_str = format!("{}) ", idx + 1);
    if num > 1 {
        log(&idx_str);
    }

    if OPT.show_number {
        print!("{}", idx_str);
    }

    if OPT.dicecodes.is_empty() ||
       OPT.dicecodes[0].is_empty()
    {
        process_keys(all_stats);
    }
    else {
        process_codes(&OPT.dicecodes, all_stats);
    }
}

/// process dice roll from command line keys
fn process_keys(all_stats: &mut Vec<IntValue>) {

    let reroll = match parse_reroll_code(&OPT.reroll) {
        Some(r) => r,
        None => {
            eprintln!("{}", DiceError::BadRerollCode);
            std::process::exit(1);  
        }
    };

    let res = process_roll(
        false,
        OPT.dices_num,
        OPT.dice,
        &reroll,
        OPT.plus,
        OPT.minus,
        OPT.drop,
        OPT.crop
    ); 
    all_stats.push(res);
}

/// process dice roll from dice codes
fn process_codes(dicecodes: &'static Vec<String>, all_stats: &mut Vec<IntValue>) {
    for dicecode in dicecodes {
        if !dicecode.is_empty() {
            // parse dice codes with regular expression
            let dice_regex: &str = r"([-+/*%^])?(?:(\d*)d(\d*)(?:(?:reroll|r)((?:\d+)(?:(?:(?:(?:,|..)(?:\d+))+)?)))?(?:(?:drop|d)(\d+)(?:crop|c)(\d+)|(?:(?:drop|d)(\d+)|(?:crop|c)(\d+)|(?:greatest|g)(\d+)|(?:lowest|l)(\d+)))?(?:(?:plus|p)(\d+))?(?:(?:minus|m)(\d+))?|(\d+))";
            let re = Regex::new(
                dice_regex).
                unwrap();

            // process regexp;
            let dices = re.captures_iter(&dicecode).into_iter();
            let dices_num = re.captures_iter(&dicecode).into_iter().count();

            // process all individual codes
            let dices_vec: Vec<Arythmetic> = dices.
                enumerate().
                map(|(num, it)| 
                        match process_code(dices_num > 1,
                                           num,
                                           &it.iter().
                                                map(|p| p.map_or("", |m| m.as_str())).
                                                collect::<Vec<&str>>()[..]) {
                            Ok(x) => x,
                            Err(e) => {
                                match e {  
                                    DiceError::BadCode => {
                                        eprintln!("{} {}!", e, dicecode);
                                        if !OPT.no_help {
                                            println!("{}", DICECODES_HELP_MSG);
                                        }
                                    }
                                    _ => eprintln!("{}", e)
                                };
                                std::process::exit(1);                                
                            }
                        }
                    ).
                collect();

            if OPT.debug {
                println!("{:?}", dices_vec);
            }                

            // process parsed dice codes
            let res = process_arithmetic(&dices_vec);

            render_codes(dices_num, dicecode, res);
            log_codes(dices_num, dicecode, res);

            all_stats.push(res);
        }
    }
}

/// process single code from parsed regular expression
fn process_code(
     is_several_rolls: bool,
     idx: usize,
     params: &[&'static str])
      -> Result<(&'static str, IntValue), DiceError> {
    if OPT.debug {
        println!("dice: {} - {:?}", idx, params);
    }

    if params.len() != 14
    {
        return Err(DiceError::BadDecryption);
    }

    if (idx == 0 && !params[1].is_empty()) ||
       (idx > 0 && params[1].is_empty()) {
        return Err(DiceError::BadCode);
    }

    if !params[13].is_empty()
    {
        let c: IntValue = params[13].parse().unwrap();
        return Ok((params[1], c));
    }


    let n: usize = match params[2] {"" => 1, x => x.parse().unwrap()};
    let d: usize = match params[3] {"" => 6, x => x.parse().unwrap()};

    let reroll = match parse_reroll_code(params[4]) {
        Some(r) => r,
        None => return Err(DiceError::BadCode)
    };

    let plus: usize = match params[11] {"" => 0, x => x.parse().unwrap()};
    let minus: usize = match params[12] {"" => 0, x => x.parse().unwrap()};    

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

    Ok((params[1], process_roll(is_several_rolls, n, d, &reroll, plus, minus, drop, crop)))
}

/// process dice roll with given parameters
fn process_roll(
    is_several_rolls: bool, 
    n: usize,
    d: usize,
    reroll: &[usize],
    plus: usize,
    minus: usize,
    drop: usize,
    crop: usize) -> IntValue {

    let add = plus as IntValue - minus as IntValue;

    let show_dice_code = log_and_render_roll(is_several_rolls, n, d, reroll, add, drop, crop);
    
    let res = match n_d_reroll_drop_crop_plus(n,
        d,
        reroll,
        add,
        drop,
        crop
    ) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    render_roll(is_several_rolls, show_dice_code, res);
    log_roll(is_several_rolls, res);

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
    n: usize,
    d: usize,
    reroll: &[usize],
    add: IntValue,
    drop: usize,
    crop: usize) -> bool {

    let dice_str = format_dice_str(is_several_rolls, n, d, reroll, add, drop, crop);

    let mut show_dice_code = false;
    if OPT.debug ||
        (OPT.verbose > 0 && !is_several_rolls) ||
        OPT.verbose > 1 {
        show_dice_code = true;
        print!("{}", dice_str);
    }

    if (OPT.log > 0 && !is_several_rolls) ||
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
            println!("{}", desc);
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
        print!("{}) ", idx + 1);
    }
}