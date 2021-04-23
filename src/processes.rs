// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// use itertools::Itertools;
use regex::Regex;

use crate::arithmetic::{Arythmetic, process_arithmetic};

use crate::dices::{IntValue, n_d_reroll_drop_crop_plus};

use crate::errors::{cant_find_method, DiceError};

use crate::init::OPT;
use crate::log::{log, logln, log_method};
use crate::methods::METHODSMAP;
use crate::render::render_dice_str;
use crate::statlists::{StatList, STATLISTSMAP};
use crate::strings::{DELIMITER, DICECODES_HELP_MSG, UNKNOWNSTATLIST_ERROR_MSG};

/// process stat generation method, uses all_stat for statistics
pub fn process_method(all_stats: &mut Vec<IntValue>, is_first: bool, is_last: bool) {
    let mut stat : Vec<IntValue> = Vec::<IntValue>::new();

    match METHODSMAP.get(&OPT.method[..]) {
        Some(method) => {
            let statlist: &StatList = STATLISTSMAP.
                get(method.get_statlist()).
                expect(UNKNOWNSTATLIST_ERROR_MSG);

            stat.resize(statlist.len(), 0);

            let n = method.get_num();

            show_title(is_first, method.get_desc());

            for i in 1..(n + 1) {
                method.get_method()(&mut stat).unwrap();

                if OPT.log > 0 {
                    log_method(n > 1, method.is_ordered(), i, &stat, statlist);
                }

                if OPT.debug || OPT.verbose > 0 || !OPT.is_collect_stat() {
                    show_stats(n > 1, method.is_ordered(), i, &stat, statlist);
                }

                all_stats.extend(stat.clone());
            }

            if (OPT.debug || OPT.verbose > 0 || !OPT.is_collect_stat()) && !OPT.numbers_only && is_last {
                println!("{}", method.get_comment());
            }
        },
        None => {
            cant_find_method(&OPT.method)
        }
    }
}

/// show title
fn show_title(is_first: bool, desc: &str) {
    if OPT.log > 0 {
        logln(DELIMITER);
        log(&OPT.method);
    }

    if OPT.show_method { 
        if is_first {
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
}


/// show stats generated in method
fn show_stats(is_shownumber: bool,
        is_ordered: bool,
        i: usize,
        stat : &Vec<IntValue>,
        statlist: &StatList) {
    if !OPT.numbers_only && is_shownumber {
        print!("{}: ", i);
    }

    if is_ordered && !OPT.numbers_only {
        for i in 0..statlist.len() {
            print!("{}: {}  ", statlist[i], stat[i]);
        }
        println!("");
    }
    else {
        println!("{:?}", stat);
    }
}

/// process any dice roll, uses all_stat for statistics
pub fn process_dices(all_stats: &mut Vec<IntValue>) {

    if OPT.dicecodes.is_empty() ||
       OPT.dicecodes[0].is_empty()
    {
        process_dices_from_keys(all_stats);
    }
    else {
        process_dices_from_codes(all_stats);
    }
}

/// process dice roll from keys
fn process_dices_from_keys(all_stats: &mut Vec<IntValue>) {

    let reroll : Vec<usize> = match OPT.reroll.is_empty() {
        true => Vec::new(),
        false => OPT.reroll.split(",").map(|s| s.parse().unwrap()).collect()
    };

    let res = process_roll(
        false,
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

    if OPT.debug || OPT.verbose > 0 || !OPT.is_collect_stat() {
        println!("{}", res);
    }    
}

/// process dice roll from dice codes
fn process_dices_from_codes(all_stats: &mut Vec<IntValue>) {
    let dicecodes_num = OPT.dicecodes.len();

    for dicecode in &OPT.dicecodes {
        if !dicecode.is_empty() {
            // parse dice codes with regular expression
            let dice_regex: &str = r"([-+/*%^])?(?:(\d*)d(\d*)(?:(?:reroll|r)((?:\d+)(?:(?:(?:,(?:\d+))+)?)))?(?:(?:drop|d)(\d+)(?:crop|c)(\d+)|(?:(?:drop|d)(\d+)|(?:crop|c)(\d+)|(?:greatest|g)(\d+)|(?:lowest|l)(\d+)))?(?:(?:plus|p)(\d+))?(?:(?:minus|m)(\d+))?|(\d+))";
            let re = Regex::new(
                dice_regex).
                unwrap();

            // process regexp;
            let dices = re.captures_iter(&dicecode).into_iter();
            let dices_num = re.captures_iter(&dicecode).into_iter().count();

            // roll needed dices
            let dices_vec: Vec<Arythmetic> = dices.
                enumerate().
                map(|(num, it)| 
                        match process_dice(dicecodes_num > 1,
                                           dices_num > 1,
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
    
            if OPT.debug || 
               (OPT.verbose == 0 && dices_num > 1 && !OPT.is_collect_stat()) ||
               (OPT.verbose > 0 && dices_num > 1) {
                if !OPT.numbers_only {
                    print!("{}: ", dicecode);
                }
                println!("{}", res);
            }

            all_stats.push(res);
        }
    }
}

/// process single dice roll from parsed regular expression
fn process_dice(
     is_several_codes: bool,
     is_several_dices: bool,
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

    let reroll : Vec<usize> = match params[4].is_empty() {
        true => Vec::new(),
        false => params[4].split(",").map(|s| s.parse().unwrap()).collect()
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

    Ok((params[1], process_roll(is_several_codes, is_several_dices, n, d, &reroll, plus, minus, drop, crop)))
}

/// process dice roll with given parameters
fn process_roll(
    is_several_codes: bool,
    is_several_dices: bool, 
    n: usize,
    d: usize,
    reroll: &[usize],
    plus: usize,
    minus: usize,
    drop: usize,
    crop: usize) -> IntValue {

    let add = plus as IntValue - minus as IntValue;

    let show_dice_code = log_and_show_dice(is_several_codes, is_several_dices, n, d, reroll, add, drop, crop);
    
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

    if OPT.debug || ((OPT.verbose > 0 || !is_several_dices) && !OPT.is_collect_stat()) {
        println!("{}{}",
         match show_dice_code && !OPT.numbers_only { true => ": ", _ => ""},
         res);
    }

    if OPT.log > 0 {
        logln(&res.to_string());
    }

    res
}

fn log_and_show_dice(
    is_several_codes: bool,
    is_several_dices: bool,
    n: usize,
    d: usize,
    reroll: &[usize],
    add: IntValue,
    drop: usize,
    crop: usize) -> bool {

    let dice_str = render_dice_str(is_several_dices, n, d, reroll, add, drop, crop);

    let mut show_dice_code = false;
    if OPT.debug || ((OPT.verbose > 0 || (!is_several_dices && is_several_codes))/* && !OPT.numbers_only*/) {
        show_dice_code = true;
        print!("{}", dice_str);
    }

    if OPT.log > 0 {
        log(&dice_str);
    }

    return show_dice_code;
}