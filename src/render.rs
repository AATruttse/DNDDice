// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use itertools::Itertools;

use crate::dices::IntValue;
use crate::errors::errorln;
use crate::init::OPT;
use crate::output::{output, outputln};
use crate::statlists::StatList;
use crate::strings::{ADVDISADV_ERROR_MSG, ADVDISADV_ADV_CODE, ADVDISADV_DISADV_CODE, TAB};

/// show roll results
pub fn render_roll(
        is_several_rolls: bool,
        show_dice_code: bool,
        is_advantage: bool,
        res: IntValue,
        force_render: bool
    ) {
    if OPT.debug ||
       OPT.verbose > 1 ||
       force_render ||
       (!is_several_rolls &&
        !is_advantage &&
            (!OPT.is_collect_stat() || OPT.verbose > 0)
        ) {
            let roll_res = format!("{}{}",
                match show_dice_code && !OPT.numbers_only { true => ": ", _ => ""},
                res);
            outputln(&roll_res);
    }
}

/// show single roll dices results
pub fn render_dices(dices: &Vec<usize>) {
    if OPT.debug || OPT.verbose > 2 {
        let dice_res = format!("{}{:?}{}",
            match OPT.numbers_only {false => " ", _ => ""},
            dices,
            match OPT.numbers_only {true => " ", _ => ""});
        output(&dice_res);
    }
}

/// show single roll results
pub fn render_dices_res(result: IntValue) {
    if !OPT.method.is_empty() && OPT.verbose > 0 {
        let dices_res = format!(": {}", result);
        outputln(&dices_res);
    }
}

/// show single roll title
pub fn render_dices_title(n: usize,
    d: usize,
    reroll: &[usize],
    add: IntValue,
    drop: usize,
    crop: usize) {
    if !OPT.method.is_empty() && OPT.verbose > 0 {
        let code_str = format_dice_str(
            false,
            n,
            d,
            reroll,
            add,
            drop,
            crop,
            false,
            false
        );
        output(&code_str);
    }
}

/// format dice codes
pub fn format_dice_str (
    is_several_rolls: bool,
    n: usize,
    d: usize,
    reroll: &[usize],
    add: IntValue,
    drop: usize,
    crop: usize,
    adv: bool,
    disadv: bool
) -> String {
        if OPT.numbers_only {
            return match is_several_rolls {
                true => TAB.to_string(),
                _ => "".to_string()
            };
        }

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
            x if x > 0 => format!("{}+{}{}", add_space, add_space, x),
            x if x < 0 => format!("{}-{}{}", add_space, add_space,-x),
            _  => "".to_string()
        };

        let adv_str: String = match (adv, disadv) {
            (false, true) => ADVDISADV_DISADV_CODE.to_string(),
            (true, false) => ADVDISADV_ADV_CODE.to_string(),
            (false, false) => "".to_string(),
            (true, true) => {
                errorln(&ADVDISADV_ERROR_MSG);
                std::process::exit(1);
            },
        };
    
        format!("{}{}d{}{}{}{}{}{}",
            match is_several_rolls {true => TAB, _ => ""},
            n,
            d,
            reroll_str,
            drop_str,
            crop_str,
            add_str,
            adv_str
        )
}

/// render stats generated in method
pub fn render_stats(is_shownumber: bool,
    is_ordered: bool,
    i: usize,
    stat : &Vec<IntValue>,
    statlist: &StatList) {
    if !OPT.numbers_only && is_shownumber {
        let num_str = format!("{}: ", i);
        output(&num_str);
    }

    if is_ordered && !OPT.numbers_only {
        for i in 0..statlist.len() {
            let stat_str = format!("{}: {}  ", statlist[i], stat[i]);
            output(&stat_str);
        }
        outputln("");
    }
    else {
        let stat_str = format!("{:?}", stat);
        outputln(&stat_str);
    }
}

/// render dice from codes
pub fn render_codes(
    dices_num: usize,
    is_advantage: bool,
    dicecode: &str,
    res: IntValue
) {
    if dices_num == 1 && is_advantage
    {
        if !OPT.numbers_only && OPT.verbose > 0 {
            let dicecode_str = format!("{}: ", dicecode);
            output(&dicecode_str);
        }
        outputln(&res.to_string());
    }
    else if OPT.debug || 
       (dices_num > 1 &&
            (OPT.verbose > 0 ||
            (OPT.verbose == 0 && !OPT.is_collect_stat() && dices_num > 1)))
    {
        if !OPT.numbers_only && OPT.verbose > 0 {
            let dicecode_str = format!("{}: ", dicecode);
            output(&dicecode_str);
        }
        outputln(&res.to_string());
    }    
}