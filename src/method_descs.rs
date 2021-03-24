// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub static DND_DESC: &str = "3d6 for each stat without choice";

pub static DNDREALMAN_DESC:     &str = "D&D for Real Men:) - Six 4d6 drop lowest with choice";
pub static DNDCRAZYLOONIE_DESC: &str = "D&D for Crazy Loonies:) - 1d20 for each stat without choice";
pub static DNDMUNCHKIN_DESC:    &str = "D&D for Munchkins:) - 25 for each stat. With choice:)";
pub static DNDEVILCHAMPION_DESC: &str = "D&D for Evil Champions:) - Six 4d6, reroll 1's, drop lowest with choice";
pub static DNDNEWBIE_DESC:      &str = "D&D for Newbies:) - 3d6 for each stat without choice";

pub static ADND1_DESC: &str = DND_DESC;
pub static ADND2_DESC: &str = "Best of two 3d6 for each without choice";
pub static ADND3_DESC: &str = "Six 3d6 with choice";
pub static ADND4_DESC: &str = "Best six of 12 3d6 with choice";
pub static ADND5_DESC: &str = "Six 4d6 drop lowest with choice";

pub static ADND1_1_DESC: &str = ADND5_DESC;
pub static ADND1_2_DESC: &str = ADND4_DESC;
pub static ADND1_3_DESC: &str = "Best of six 3d6 for each ability";
pub static ADND1_4_DESC: &str = "Best of 12 '3d6 without choice' sets";

pub static DND3_DESC: &str = ADND5_DESC;
pub static DND3ORGANIC_DESC: &str = "Six 4d6 drop lowest without choice, reroll one, switch any two";
pub static DND3CA_DESC: &str = "Six 3d6 with choice, reroll very bad stats";
pub static DND3RA_DESC: &str = "3d6 for each stat without choice, reroll very bad stats";
pub static DND3HP_DESC: &str = "Six 5d6 drop lowest two with choice, reroll below average stats";

pub static DND35_DESC: &str = ADND5_DESC;
pub static DND35ORGANIC_DESC: &str = DND3ORGANIC_DESC;
pub static DND35CA_DESC: &str = DND3CA_DESC;
pub static DND35RA_DESC: &str = DND3RA_DESC;
pub static DND35HP_DESC: &str = DND3HP_DESC;
pub static DND35ELITE_DESC: &str = "15,14,13,12,10,8 with choice";

pub static DND4_DESC: &str = "Six 4d6 drop lowest with choice, reroll bad stats";
pub static DND4STANDARD_DESC: &str = "16,14,13,12,11,10 with choice";

pub static DND5_DESC: &str = ADND5_DESC;
pub static DND5STANDARD_DESC: &str = DND35ELITE_DESC;

pub static PFSTANDARD_DESC: &str = ADND5_DESC;
pub static PFCLASSIC_DESC: &str = ADND3_DESC;
pub static PFHEROIC_DESC: &str = "Six 2d6+6 with choice";

pub static PF2_DESC: &str = ADND5_DESC;

pub static SF_DESC: &str = ADND5_DESC;
pub static SFFOCUSED_DESC: &str = "18,14,11,10,10,10 with choice";
pub static SFSPLIT_DESC: &str = "16,16,11,10,10,10 with choice";
pub static SFVERSATILE_DESC: &str = "14,14,14,11,10,10 with choice";

pub static CP2013_1_DESC: &str = "9d10 character points";
pub static CP2013_2_DESC: &str = "30+6d10 character points";
pub static CP2020_1_DESC: &str = CP2013_1_DESC;
pub static CP2020_2_DESC: &str = "Nine 1d10 reroll 1,2 with choice";
pub static CP2020_3_DESC: &str = "Nine 1d10 reroll 1,2 without choice";

pub static CYBERSPACE_1_DESC: &str = "Eleven 1d100 with choice";
pub static CYBERSPACE_2_DESC: &str = "Eleven 1d100 without choice";

pub static ARM1_DESC: &str = "1d10-1d10 for each pair of characteristics with choice";
pub static ARM2_DESC: &str = "1d10-1d10 for each pair of characteristics without choice";

pub static WH40K_DESC: &str = "2d10 for each characteristic without choice";
pub static WH40KREROLL_DESC: &str = "2d10 for each characteristic without choice, reroll one";
pub static WH40KCHOICE_DESC: &str = "2d10 for each characteristic with choice";
pub static WH40KCHOICEREROLL_DESC: &str = "2d10 for each characteristic with choice, reroll one";

pub static SWN_DESC: &str = "3d6 for each stat without choice, then you can change one ability to 14";
pub static SWNSTANDARD_DESC: &str = "14,12,11,10,9,7 with choice";

pub static RQ6_DESC: &str = "3d6 for STR, CON, DEX, POW and CHA; 2d6+6 for SIZ and INT";
pub static RQ6CHOICE_DESC: &str = "3d6 for STR, CON, DEX, POW and CHA with choice; 2d6+6 for SIZ and INT with choice";

