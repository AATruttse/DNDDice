// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeSet;

use crate::dices::IntValue;
use crate::errors::DiceError;
use crate::method_comments::ASYOUWISH_COMMENT;

/// Type for method's tag list
pub type Tags = BTreeSet<&'static str>;

/// Type for generation method's function ptr
pub type GenMethod = fn(&mut Vec<IntValue>) -> Result<(), DiceError>;

/// struct for generation method
pub struct Method {
    /// used in this method statistics list
    statlist: &'static str,

    /// is method gives ordered stats?
    is_ordered: bool,

    /// string with method's short description
    desc: &'static str,
    /// string with method's long description
    desc_long: &'static str,
    /// string with comment to show to user
    comment: &'static str,

    /// method's function ptr
    method: GenMethod,
    /// number of times the method's function is used
    num: usize,

    /// method's tags
    tags: Tags,
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

    pub fn get_comment(&self) -> &str {
        self.comment
    }

    pub fn get_method(&self) -> GenMethod {
        self.method
    }

    pub fn get_num(&self) -> usize {
        self.num
    }

    pub fn get_tags(&self) -> &Tags {
        &self.tags
    }

    /// checks if method corresponds to given tags' string
    pub fn check_tags(&self, tag_string: &str) -> bool {
        if tag_string.is_empty() {
            return false;
        }

        for tag in tag_string.split(",") {
            if !self.tags.contains(&*tag.to_lowercase()) {
                return false;
            }
        }

        true
    }

    /// creates new method
    pub fn new(
        statlist: &'static str,
        is_ordered: bool,
        desc: &'static str,
        desc_long: &'static str,
        method: GenMethod,
        tags: &[&'static str],
    ) -> Self {
        let new_method = Method {
            statlist: statlist,
            is_ordered: is_ordered,
            desc: desc,
            desc_long: desc_long,
            comment: match is_ordered {
                true => "",
                false => ASYOUWISH_COMMENT,
            },
            method: method,
            num: 1,
            tags: tags.iter().copied().collect(),
        };

        new_method
    }

    /// creates new method with given comment
    pub fn new_w_comment(
        statlist: &'static str,
        is_ordered: bool,
        desc: &'static str,
        desc_long: &'static str,
        comment: &'static str,
        method: GenMethod,
        tags: &[&'static str],
    ) -> Self {
        let mut new_method = Method::new(statlist, is_ordered, desc, desc_long, method, tags);

        new_method.comment = comment;
        new_method
    }

    /// creates new method that rolls several times
    pub fn new_w_num(
        statlist: &'static str,
        is_ordered: bool,
        desc: &'static str,
        desc_long: &'static str,
        method: GenMethod,
        num: usize,
        tags: &[&'static str],
    ) -> Self {
        let mut new_method = Method::new(statlist, is_ordered, desc, desc_long, method, tags);

        new_method.num = num;
        new_method
    }

    /// creates new method that rolls several times with comment
    pub fn new_w_num_comment(
        statlist: &'static str,
        is_ordered: bool,
        desc: &'static str,
        desc_long: &'static str,
        comment: &'static str,
        method: GenMethod,
        num: usize,
        tags: &[&'static str],
    ) -> Self {
        let mut new_method =
            Method::new_w_comment(statlist, is_ordered, desc, desc_long, comment, method, tags);

        new_method.num = num;
        new_method
    }
}
