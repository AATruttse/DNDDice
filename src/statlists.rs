// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;

/// Type for stat list (D&D, Cyberpunk etc...)
pub type StatList = Vec<&'static str>;

/// Type for stat lists' BTreeMap 
type StatListsMap = BTreeMap<&'static str, StatList>;

// BTreeMap for stat lists
lazy_static! {
    pub static ref STATLISTSMAP: StatListsMap = {
        let mut m = StatListsMap::new();
        
        m.insert("d&d", vec!["STR","DEX","CON","INT","WIS","CHR"]);
        m.insert("d&dreroll", vec!["STR","DEX","CON","INT","WIS","CHR","reroll"]);
        m.insert("cyberpunk-stat", vec!["INT","REF","CL","TECH","LK","ATT","MA","EMP","BODY"]);
        m.insert("cyberpunk-cp", vec!["Character points"]);
        m.insert("arsmagica", vec!["Int/Per","Str/Sta","Pre/Com","Dex/Qik"]);
        m.insert("warhammer", vec!["WS","BS","S","T","Ag","Int","Per","WP","Fel"]);
        m.insert("warhammer-reroll", vec!["WS","BS","S","T","Ag","Int","Per","WP","Fel", "reroll"]);

        m
    };
}
