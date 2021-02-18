// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use regex::Regex;

use crate::dices::IntValue;
use crate::dices::n_d_drop_crop_plus;

use crate::errors::cant_find_method;

use crate::init::OPT;
use crate::methods::METHODSMAP;

use crate::statlists::StatList;
use crate::statlists::STATLISTSMAP;

use crate::strings::BADDICECODE_ERROR_MSG;
use crate::strings::DICECODES_HELP_MSG;
use crate::strings::UNKNOWNSTATLIST_ERROR_MSG;

/// process stat generation method
pub fn process_method(all_stats: &mut Vec<IntValue>) {
    let mut stat : Vec<IntValue> = Vec::<IntValue>::new();

    match METHODSMAP.get(&OPT.method[..]) {
        Some(method) => {
            let statlist: &StatList = STATLISTSMAP.
                get(method.get_statlist()).
                expect(UNKNOWNSTATLIST_ERROR_MSG);

            stat.resize(statlist.len(), 0);
            method.get_method()(&mut stat).unwrap();

            if OPT.debug || OPT.verbose > 0 || !OPT.is_collect_stat() {
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
        },
        None => {
            cant_find_method(&OPT.method)
        }
    }

    all_stats.extend(stat.clone());

}

/// process any dice roll, use all_stat for statistics
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
    all_stats.push(
        process_roll(
            OPT.dices_num,
            OPT.dice,
            OPT.plus,
            OPT.minus,
            OPT.drop,
            OPT.crop
        )
    );
}

/// process dice roll from dice codes
fn process_dices_from_codes(all_stats: &mut Vec<IntValue>) {
    for dicecode in &OPT.dicecodes {
        if !dicecode.is_empty() {

            let dice_regex: &str = r"(?:(\d*)d(\d*)(?:(?:drop|d)(\d+)(?:crop|c)(\d+)|(?:(?:drop|d)(\d+)|(?:crop|c)(\d+)|(?:greatest|g)(\d+)|(?:lowest|l)(\d+)))?(?:(?:plus|p)(\d+))?(?:(?:minus|m)(\d+))?|(\d+))";
            let dices_regex = ["^", dice_regex, "$"].concat();
            //regular expression for dice codes parsing
            let re = Regex::new(
                dices_regex.as_str()).
                unwrap();

            let params_vec: Vec<&str> = match re.captures(&dicecode) {
                Some(x) => x.iter().map(|p| p.map_or("", |m| m.as_str())).collect(),
                None => process_bad_code(dicecode)
            };

            if OPT.debug {
                println!("{:?}", params_vec);
            }
            
            all_stats.push(process_params(&params_vec[1..]));
        }
    }
}

/// process dice params from code parsing
fn process_params(params: &[&str]) -> IntValue {

    if params.len() >= 11 && !params[10].is_empty()
    {
        let c: IntValue = params[10].parse().unwrap();
        return c;
    }

    let n: usize = match params[0] {
            "" => 1,
            x => x.parse().unwrap()
        };

    let d: usize = match params[1] {
        "" => 6,
        x => x.parse().unwrap()
    };

    let drop: usize = match params[2] {
        "" => match params[4] {
            "" => match params[6] {
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

    let crop: usize = match params[3] {
        "" => match params[5] {
            "" => match params[7] {
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

    let plus: usize = match params[8] {
        "" => 0,
        x => x.parse().unwrap()
    };

    let minus: usize = match params[9] {
        "" => 0,
        x => x.parse().unwrap()
    };

    process_roll(n, d, plus, minus, drop, crop)
}

/// process dice roll with given parameters
fn process_roll( 
    n: usize,
    d: usize,
    plus: usize,
    minus: usize,
    drop: usize,
    crop: usize) -> IntValue {

    let add = plus as IntValue - minus as IntValue;

    if OPT.debug || (OPT.verbose > 0 && !OPT.numbers_only) {
        let drop_str: String = match drop {
            0 => "".to_string(),
            x => format!(" drop {}", x)};

        let crop_str: String = match crop {
            0 => "".to_string(),
            x => format!(" crop {}", x)};
            
        let add_str: String = match add {
            x if x > 0 => format!("+{}", x),
            x if x < 0 => format!("-{}", -x),
            _  => "".to_string()
        };
                        
        //{ let s = String::from("testing"); &s }
        print!("{}d{}{}{}{}: ",
         n,
         d,
         add_str,
         drop_str,
         crop_str
        );
    }
    
    let res = match n_d_drop_crop_plus(n,
        d,
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

    if OPT.debug || OPT.verbose > 0 || !OPT.is_collect_stat() {
        println!("{}", res);
    }

    res
}

/// processes error when dice code can't be parsed
fn process_bad_code(dicecode: &String) -> Vec<&str> {
    eprintln!("{} {}", BADDICECODE_ERROR_MSG, dicecode);
    if !OPT.no_help {
        println!("{}", DICECODES_HELP_MSG);
    }
    std::process::exit(1);
}
