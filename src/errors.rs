use custom_error::custom_error;

use crate::help::help_methods;
use crate::init::OPT;
use crate::strings::UNKNOWNMETHOD_ERROR_MSG;

// Custom errors type for dice throwing functions
custom_error!{pub DiceError
    Dices0                       = "Can't throw 0 dices!",
    Sides0                       = "Can't throw 0-sided dice!",
    BadDrop{n:usize, drop:usize} = "Can't drop {drop} dices from {n} dices!",
    BadCrop{n:usize, crop:usize} = "Can't crop {crop} dices from {n} dices!"
}

pub fn cant_find_method(method: &str) {
    eprintln!("{} {}.", UNKNOWNMETHOD_ERROR_MSG, method);
    if !OPT.no_help {
        help_methods();
    }

    std::process::exit(1);    

}