use crate::errors::cant_find_method;
use crate::init::OPT;
use crate::methods::METHODSMAP;
use crate::strings::DICECODES_HELP_MSG;

pub fn help() {

    if OPT.help_dice_codes {
        help_dicecodes();
    }

    if OPT.help_methods {
        help_methods();
    }

    if !OPT.help_method.is_empty() {
        help_method(&OPT.help_method);
    }


    std::process::exit(0);
}

fn help_dicecodes() {
    println!("{}", DICECODES_HELP_MSG);
}

pub fn help_methods() {
    println!("Generation methods:");
    for (key, val) in METHODSMAP.iter() {
        println!("  {} - {}", key, val.get_desc());
    }
}

fn help_method(name: &str) {
    match METHODSMAP.get(name) {
        Some(method) => {
            println!("{}", method.get_desc_long());
        },
        None => {
            cant_find_method(name)
        }
    }
}