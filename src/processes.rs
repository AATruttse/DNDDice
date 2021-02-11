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

/// process dice roll with given parameters, use all_stat for statistics
fn process_roll(all_stats: &mut Vec<IntValue>, 
    n: usize,
    d: usize,
    plus: usize,
    minus: usize,
    drop: usize,
    crop: usize) {

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

    all_stats.push(res);
}

/// process dice roll from keys
fn process_dices_from_keys(all_stats: &mut Vec<IntValue>) {
    process_roll(all_stats,
        OPT.dices_num,
        OPT.dice,
        OPT.plus,
        OPT.minus,
        OPT.drop,
        OPT.crop
        );
}

/// processes error when dice code can't be parsed
fn process_bad_code(dicecode: &String) -> regex::Captures<'static> {
    eprintln!("{} {}", BADDICECODE_ERROR_MSG, dicecode);
    if !OPT.no_help {
        println!("{}", DICECODES_HELP_MSG);
    }
    std::process::exit(1);
}

/// process dice roll from dice codes
fn process_dices_from_codes(all_stats: &mut Vec<IntValue>) {
    for dicecode in &OPT.dicecodes {
        if !dicecode.is_empty() {
            let re = Regex::new(
                r"(\d{0,})d(\d{0,})(drop(\d{1,}))?(crop(\d{1,}))?([+](\d{1,}))?([-](\d{1,}))?").
                unwrap();

            let params = match re.captures(&dicecode) {
                Some(x) => x,
                None => process_bad_code(dicecode)
            };

            // bad string such as gffdffgg
            if dicecode != "d"
                && &params[0] == "d"
                && &params[1] == ""
                && &params[2] == "" {
                    process_bad_code(dicecode);
            }

            if OPT.debug {
                println!("{:?}", params);
            }

            let n : usize = params.get(1).map_or(
                0,
                |m| match m.as_str() {
                    "" => 1,
                    x => x.parse().unwrap()
                });

            let d : usize = params.get(2).map_or(
                6,
                |m| match m.as_str() {
                    "" => 6,
                    x => x.parse().unwrap()
                });

            let drop : usize = params.get(4).map_or(
                0,
                |m| match m.as_str() {
                    "" => 0,
                    x => x.parse().unwrap()
                });

            let crop : usize = params.get(6).map_or(
                0,
                |m| match m.as_str() {
                    "" => 0,
                    x => x.parse().unwrap()
                });

            let plus : usize = params.get(8).map_or(
                0,
                |m| match m.as_str() {
                    "" => 0,
                    x => x.parse().unwrap()
                });

            let minus : usize = params.get(10).map_or(
                0,
                |m| match m.as_str() {
                    "" => 0,
                    x => x.parse().unwrap()
                });

            process_roll(all_stats,
                n,
                d,
                plus,
                minus,
                drop, 
                crop
            );
        }
    }
}
