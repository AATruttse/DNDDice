// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::commands::{CommandFunc, show_bad_command, COMMANDS_LONG_MAP, COMMANDS_SHORT_MAP};

fn get_command_func(code: &str) -> Option<CommandFunc> {
    match COMMANDS_LONG_MAP.get(code) {
        Some(func) => Some(*func),
        None => match COMMANDS_SHORT_MAP.get(code) {
            Some(func) => Some(*func),
            None => None 
        }
    }
}

/// process input from stdin
pub fn process_input(input: &str) {
    let (mut command, args) = match input.split_once(" ") {
        Some((c, a)) => (c, a),
        None => (input, "")
    };

    match command.chars().nth(0) {
        Some('.') => {},
        Some(_) | None => show_bad_command()
    };

    command = &command[1..command.len()];
    
    match get_command_func(command) {
        Some(func) => func(&args.split(" ").collect::<Vec<&str>>()),
        None => show_bad_command()
    }
}

