// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use itertools::Itertools;

use crate::dices::IntValue;
use crate::init::OPT;
use crate::strings::TAB;

pub fn render_roll(dices: &Vec<usize>) {
    if OPT.debug || OPT.verbose > 1 {
        print!("{}{:?}{}",
            match OPT.numbers_only {false => " ", _ => ""},
            dices,
            match OPT.numbers_only {true => " ", _ => ""});
    }
}

pub fn render_dice_str (
    is_several_dices: bool,
    n: usize,
    d: usize,
    reroll: &[usize],
    add: IntValue,
    drop: usize,
    crop: usize) -> String {
        if OPT.numbers_only {
            return match is_several_dices {
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
    
        format!("{}{}d{}{}{}{}{}",
            match is_several_dices {true => TAB, _ => ""},
            n,
            d,
            reroll_str,
            drop_str,
            crop_str,
            add_str
        )
}