// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;

use crate::dices::IntValue;
use crate::errors::cant_find_method;
use crate::help::{help_dicecodes, help_method, help_methods, help_tags, find_tags};
use crate::processes::{process_codes, process_method};
use crate::strings::{BADCOMMAND_ERROR_MSG, COMMAND_HELP_MSG};

/// Type for interactive-mode command's execution functions
pub type CommandFunc = fn (&Vec<&str>);

/// Type for  interactive-mode command's execution functions' BTreeMap 
type CommandFuncsMap = BTreeMap<&'static str, CommandFunc>;

/// Array for interactive mode commands - short, long, function, help message
const COMMANDS: &'static [(&str, &str, CommandFunc, &str)] = &[
    ("h", "help", command_help, " 			- this help"),
    ("hd", "help-dice-codes", command_helpdicecodes, " 	- see help about dice codes' format description"),
    ("hm", "help-method", command_helpmethod, " METHODNAME	- see help about generation method METHODNAME"),
    ("hms", "help-methods", command_helpmethods, " 		- see help about generation methods"),
    ("ht", "help-tags", command_helptags, " 		- see tags' list"),
    ("ft", "find-tags", command_findtags, " TAG1,TAG2,... - see help about generation method by tags (for example: \"DnD,ordered\"). See .help-tags to see tags' list"),
    ("m", "method", command_method, " METHODNAME NUM	- execute generation method METHODNAME NUM times"),
    ("q", "quit", command_quit, " 			- exit program")
];

fn command_help(_args: &Vec<&str>) {
    println!("{}", COMMAND_HELP_MSG);

    for comm in COMMANDS {
        println!(".{} or .{}{}", comm.1, comm.0, comm.3);
    }
}


fn command_helpmethod(_args: &Vec<&str>) {
    if _args.len() < 1 || _args[0].len() == 0 {
        show_bad_command();
        return;
    }

    help_method(_args[0], false);
}


fn command_helpmethods(_args: &Vec<&str>) {
    help_methods();
}

fn command_helpdicecodes(_args: &Vec<&str>) {
    help_dicecodes();
}

fn command_helptags(_args: &Vec<&str>) {
    help_tags();
}

fn command_findtags(_args: &Vec<&str>) {
    if _args.len() < 1 || _args[0].len() == 0 {
        show_bad_command();
        return;
    }

    find_tags(_args[0]);
}

fn command_quit(_args: &Vec<&str>) {
    std::process::exit(0);
}

fn command_method(_args: &Vec<&str>) {
    if _args.len() < 1 || _args[0].len() == 0 {
        show_bad_command();
        return;
    }

    let n: usize = match _args.len() {
        1 => 1,
        _ => match _args[1].parse() {
            Ok(n)   => n,
            Err(_)  => {
                show_bad_command();
                return;
            }
        }
    };

    let mut all_stats: Vec<IntValue> = Vec::new();

    for _i in 0..n {
        match process_method(_args[0], &mut all_stats, _i, n){
            Some(_) => {},
            None => {
                cant_find_method(_args[0], false);
            }
        }
    }    
}

pub fn command_codes(input: &str, n: usize) {

    let mut input_string: String = input.to_owned();

    while input_string.find("  ") != None {
        input_string = input_string.replace("  ", " ");
    }

    let codes: Vec<String> = input_string.split(' ').map(|code| code.to_owned()).collect();

    let mut all_stats: Vec<IntValue> = Vec::new();
    process_codes(&codes, &mut all_stats);

}

pub fn show_bad_command() {
    println!("{}", BADCOMMAND_ERROR_MSG);
    command_help(&Vec::new());
}

lazy_static! {
    // BTreeMap with all methods
    pub static ref COMMANDS_LONG_MAP: CommandFuncsMap = {
        let mut c = CommandFuncsMap::new();

        for comm in COMMANDS {
            c.insert(comm.1, comm.2);
        }        
            
        c
    };

    pub static ref COMMANDS_SHORT_MAP: CommandFuncsMap = {
        let mut c = CommandFuncsMap::new();

        for comm in COMMANDS {
            c.insert(comm.0, comm.2);
        }        

        c
    };    
}


