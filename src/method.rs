// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::dices::IntValue;
use crate::errors::DiceError;

/// Type for generation method's function ptr
pub type GenMethod = fn (&mut Vec<IntValue>) -> Result<(), DiceError>;

/// struct for generation method
pub struct Method {
    /// used in this method statistics list
    statlist:  &'static str,

    /// is method gives ordered stats?
    is_ordered: bool,

    /// string with method's short description
    desc:      &'static str,
    /// string with method's long description
    desc_long: &'static str,

    /// method's function ptr
    method:    GenMethod
}

impl Method {
    pub fn get_statlist(&self) -> &str {
        self.statlist
    }

    pub fn is_ordered(&self) -> bool {
        self.is_ordered
    }

    pub fn get_desc(&self) -> &str {
        self.desc
    }

    pub fn get_desc_long(&self) -> &str {
        self.desc_long
    }

    pub fn get_method(&self) -> GenMethod {
        self.method
    }

    pub fn new(
        statlist:   &'static str,
        is_ordered: bool,
        desc:       &'static str,
        desc_long:  &'static str,
        method:     GenMethod
    ) -> Self {
        let new_method = Method {
            statlist: statlist,
            is_ordered: is_ordered,
            desc: desc,
            desc_long: desc_long,
            method: method
        };

        new_method
    }
}
