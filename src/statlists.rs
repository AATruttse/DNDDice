use std::collections::BTreeMap;

/// Type for stat list (D&D, Cyberpunk etc...)
pub type StatList = Vec<&'static str>;

/// Type for stat lists' BTreeMap 
type StatListsMap = BTreeMap<&'static str, StatList>;

lazy_static! {
    pub static ref STATLISTSMAP: StatListsMap = {
        let mut m = StatListsMap::new();
        
        m.insert("d&d", vec!["STR","DEX","CON","INT","WIS","CHR"]);
        m.insert("d&dreroll", vec!["STR","DEX","CON","INT","WIS","CHR","reroll"]);
        m.insert("cyberpunk-stat", vec!["INT","REF","CL","TECH","LK","ATT","MA","EMP","BODY"]);
        m.insert("cyberpunk-cp", vec!["Character points"]);
        m.insert("arsmagica", vec!["Int/Per","Str/Sta","Pre/Com","Dex/Qik"]);
        m.insert("cyberspace", vec!["Co","Ag","SD","Re","Me","St","Qu","Em","In","Pr","Ap"]);

        m
    };
}
