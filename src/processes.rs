// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use itertools::Itertools;
use regex::Regex;

use crate::arithmetic::Arythmetic;
use crate::arithmetic::process_arithmetic;

use crate::dices::IntValue;
use crate::dices::n_d_reroll_drop_crop_plus;

use crate::errors::cant_find_method;
use crate::errors::DiceError;

use crate::init::OPT;
use crate::methods::METHODSMAP;

use crate::statlists::StatList;
use crate::statlists::STATLISTSMAP;

use crate::strings::DICECODES_HELP_MSG;
use crate::strings::UNKNOWNSTATLIST_ERROR_MSG;

/// process stat generation method, uses all_stat for statistics
pub fn process_method(all_stats: &mut Vec<IntValue>) {
    let mut stat : Vec<IntValue> = Vec::<IntValue>::new();

    match METHODSMAP.get(&OPT.method[..]) {
        Some(method) => {
            let statlist: &StatList = STATLISTSMAP.
                get(method.get_statlist()).
                expect(UNKNOWNSTATLIST_ERROR_MSG);

            stat.resize(statlist.len(), 0);

            let n = method.get_num();

            if OPT.show_method {
                println!("{}", method.get_desc());
            }

            for i in 1..(n + 1) {
                method.get_method()(&mut stat).unwrap();

                if OPT.debug || OPT.verbose > 0 || !OPT.is_collect_stat() {
                    if !OPT.numbers_only && n > 1 {
                        print!("{}: ", i);
                    }

                    if method.is_ordered() && !OPT.numbers_only {
                        for i in 0..statlist.len() {
                            print!("{}: {}  ", statlist[i], stat[i]);
                        }
                        println!("");
                    }
                    else {
                        println!("{:?}", stat);
                    }
                }

                all_stats.extend(stat.clone());
            }

            if (OPT.debug || OPT.verbose > 0 || !OPT.is_collect_stat()) &&  !OPT.numbers_only {
                println!("{}", method.get_comment());
            }
        },
        None => {
            cant_find_method(&OPT.method)
        }
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
    for dicecode in &OPT.dicecodes {
        if !dicecode.is_empty() {
            // parse dice codes with regular expression
            let dice_regex: &str = r"([-+/*%^])?(?:(\d*)d(\d*)(?:(?:reroll|r)((?:\d+)(?:(?:(?:,(?:\d+))+)?)))?(?:(?:drop|d)(\d+)(?:crop|c)(\d+)|(?:(?:drop|d)(\d+)|(?:crop|c)(\d+)|(?:greatest|g)(\d+)|(?:lowest|l)(\d+)))?(?:(?:plus|p)(\d+))?(?:(?:minus|m)(\d+))?|(\d+))";
            let re = Regex::new(
                dice_regex).
                unwrap();

            // roll needed dices
            let dices_vec: Vec<Arythmetic> = re.captures_iter(&dicecode).
                into_iter().
                enumerate().
                map(|(num, it)| 
                        match process_dice(num,
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
    
            if OPT.debug || OPT.verbose > 0 || !OPT.is_collect_stat() {
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
fn process_dice(idx: usize, params: &[&'static str]) -> Result<(&'static str, IntValue), DiceError> {
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

    Ok((params[1], process_roll(n, d, &reroll, plus, minus, drop, crop)))
}

/// process dice roll with given parameters
fn process_roll( 
    n: usize,
    d: usize,
    reroll: &[usize],
    plus: usize,
    minus: usize,
    drop: usize,
    crop: usize) -> IntValue {

    let add = plus as IntValue - minus as IntValue;

    if OPT.debug || (OPT.verbose > 0 && !OPT.numbers_only) {
        let drop_str: String = match drop {
            0 => "".to_string(),
            x => format!(" drop {}", x)
        };

        let crop_str: String = match crop {
            0 => "".to_string(),
            x => format!(" crop {}", x)
        };
            
        let reroll_str: String = match reroll.len() {
            0 => "".to_string(),
            _ => format!(" reroll {}", reroll.iter().join(","))
        };

        let add_space = match drop_str.len() + crop_str.len() + reroll_str.len() {
            0 => "",
            _ => " "
        };

        let add_str: String = match add {
            x if x > 0 => format!("{}+{}", add_space, x),
            x if x < 0 => format!("{}-{}", add_space,-x),
            _  => "".to_string()
        };
        
        print!("{}d{}{}{}{}{}: ",
         n,
         d,
         reroll_str,
         drop_str,
         crop_str,
         add_str
        );
    }
    
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

    if OPT.debug || (OPT.verbose > 0 && !OPT.is_collect_stat()) {
        println!("{}", res);
    }

    res
}