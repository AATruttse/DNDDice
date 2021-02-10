use crate::init::OPT;

use crate::strings::DICECODES_HELP_MSG;

pub fn help() {

    if OPT.help_dice_codes {
        help_dicecodes();
    }

    std::process::exit(0);
}

fn help_dicecodes() {
    println!("{}", DICECODES_HELP_MSG);
}