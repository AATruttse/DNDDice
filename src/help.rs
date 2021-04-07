// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::errors::cant_find_method;
use crate::init::OPT;
use crate::methods::{METHODSMAP,TAGSET};

use crate::strings::DICECODES_HELP_MSG;
use crate::strings::{METHODS_MESSAGE, NO_TAGS_MESSAGE, TAGS_MESSAGE};

/// shows needed help
pub fn help() {

    if OPT.help_dice_codes {
        help_dicecodes();
    }

    if OPT.help_methods {
        help_methods();
    }

    if OPT.help_tags {
        help_tags();
    }

    if !OPT.help_method.is_empty() {
        help_method(&OPT.help_method);
    }

    if !OPT.find_tags.is_empty() {
        find_tags(&OPT.find_tags);
    }


    std::process::exit(0);
}

///shows help about dice codes 
fn help_dicecodes() {
    println!("{}", DICECODES_HELP_MSG);
}

///shows help about methods 
pub fn help_methods() {
    println!("{}", METHODS_MESSAGE);
    for (key, val) in METHODSMAP.iter() {
        println!("  {} - {}", key, val.get_desc());
    }
}

///shows help about tags 
pub fn help_tags() {
    println!("{}", TAGS_MESSAGE);
    for val in TAGSET.iter() {
        print!("{} ", val);
    }
    println!("");
}

///shows help about methods 
fn help_method(name: &str) {
    match METHODSMAP.get(name) {
        Some(method) => {
            println!("{}", method.get_desc_long());
            println!("Tags: {:?}", method.get_tags())
        },
        None => {
            cant_find_method(name)
        }
    }
}

///find methods by tags 
fn find_tags(tags: &str) {
    let mut is_found = false;
    for (key, val) in METHODSMAP.iter().filter(|(_, m)| m.check_tags(tags)) {
        println!("  {} - {}", key, val.get_desc());
        is_found = true;
    }

    if !is_found {
        println!("{}", NO_TAGS_MESSAGE);
        help_tags();
    }
}