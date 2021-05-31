// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use custom_error::custom_error;

use crate::help::help_methods;
use crate::init::OPT;

use crate::strings::{DICECODES_HELP_MSG, UNKNOWNMETHOD_ERROR_MSG};

// Custom errors type for dice throwing functions
custom_error!{pub DiceError
    Dices0                       = "Can't throw 0 dices!",
    Sides0                       = "Can't throw 0-sided dice!",
    BadDrop{n:usize, drop:usize} = "Can't drop {drop} dices from {n} dices!",
    BadCrop{n:usize, crop:usize} = "Can't crop {crop} dices from {n} dices!",
    BadCode                      = "Can't parse the dice code: ",
    BadRerollCode                = "Reroll code error",
    BadDecryption                = "Dice code decription internal error!",
    BadOperator {op: &'static str} = "Unknown operator {op} in dice code ",
}

// Shows errors that there isn't known method str and known method list
pub fn cant_find_method(method: &str, need_exit: bool) {
    eprintln!("{} {}.", UNKNOWNMETHOD_ERROR_MSG, method);
    if !OPT.no_help {
        help_methods();
    }

    if need_exit
    {
        std::process::exit(1);
    }
}

pub fn process_dice_code_error(dicecodes: &Vec<String>, err: DiceError, need_exit: bool) {
    match err {  
        DiceError::BadCode => {
            eprintln!("{} {}!", err, dicecodes.join(" "));
            if !OPT.no_help {
                println!("{}", DICECODES_HELP_MSG);
            }
        }
        _ => eprintln!("{}", err)
    };

    if need_exit {
        std::process::exit(1);  
    }
}