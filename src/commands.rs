// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;

use crate::dices::IntValue;
use crate::processes::{process_dices, process_method};
use crate::strings::{BADCOMMAND_ERROR_MSG, COMMAND_HELP_MSG};

static EXIT_COMMANDS: (&str, &str) = ("q", "quit");
static HELP_COMMANDS: (&str, &str) = ("h", "help");
static METHOD_COMMANDS: (&str, &str) = ("m", "method");

/// Type for interactive-mode command's execution functions
pub type CommandFunc = fn (&Vec<&str>);

/// Type for  interactive-mode command's execution functions' BTreeMap 
type CommandFuncsMap = BTreeMap<&'static str, CommandFunc>;

fn command_help(_args: &Vec<&str>) {
    println!("{}", COMMAND_HELP_MSG);
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
        process_method(_args[0], &mut all_stats, _i, n);
    }    
}

pub fn show_bad_command() {
    println!("{}", BADCOMMAND_ERROR_MSG);
    command_help(&Vec::new());
}

lazy_static! {
    // BTreeMap with all methods
    pub static ref COMMANDS_LONG_MAP: CommandFuncsMap = {
        let mut c = CommandFuncsMap::new();

        c.insert(EXIT_COMMANDS.1, command_quit);
        c.insert(HELP_COMMANDS.1, command_help);
        c.insert(METHOD_COMMANDS.1, command_method);
            
        c
    };

    pub static ref COMMANDS_SHORT_MAP: CommandFuncsMap = {
        let mut c = CommandFuncsMap::new();

        c.insert(EXIT_COMMANDS.0, command_quit);
        c.insert(HELP_COMMANDS.0, command_help);            
        c.insert(METHOD_COMMANDS.0, command_method);

        c
    };    
}


