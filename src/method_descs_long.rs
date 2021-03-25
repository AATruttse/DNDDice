// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub static DND_HELP: &str = "Roll 3d6, use the result for character's Strength ability score. Repeat for Dexterity, Constitution, Intelligence, Wisdom, and Charisma.";

pub static DNDREALMAN_HELP: &str = "D&D for Real Men - Roll 4d6 and discard the lowest die. Repeat six times. Assign the results to your character's six abilities as you wish.";
pub static DNDCRAZYLOONIE_HELP: &str = "D&D for Crazy Loonies -  Roll 1d20, use the result for character's Strength ability score. Repeat for Dexterity, Constitution, Intelligence, Wisdom, and Charisma.";
pub static DNDMUNCHKIN_HELP: &str = "D&D for Munchkins - Use 25 for each stat.";
pub static DNDEVILCHAMPION_HELP: &str = "D&D for Evil Champions - Roll 4d6, reroll all 1's, discard the lowest die. Repeat six times. Assign the results to your character's six abilities as you wish.";
pub static DNDNEWBIE_HELP: &str = "D&D for Newbies - Roll 3d6, use the result for character's Strength ability score. Repeat for Dexterity, Constitution, Intelligence, Wisdom, and Charisma.";

pub static ADND1_HELP: &str = DND_HELP;
pub static ADND2_HELP: &str = "Roll two 3d6, use the best result for character's Strength ability score. Repeat for Dexterity, Constitution, Intelligence, Wisdom, and Charisma.";
pub static ADND3_HELP: &str = "Roll 3d6 six times. Assign the results to your character's six abilities as you wish.";
pub static ADND4_HELP: &str = "Roll 3d6 twelve times. Assign the best six results to your character's six abilities as you wish.";
pub static ADND5_HELP: &str = "Roll 4d6 and discard the lowest die. Repeat six times. Assign the results to your character's six abilities as you wish.";

pub static ADND_CBON1_HELP: &str = "One of six '3d6 without choice' pregenerated sets for real necromancer";
pub static ADND_CBON2_HELP: &str = "One of six 'Best of two 3d6 without choice' pregenerated sets for real necromancer";
pub static ADND_CBON3_HELP: &str = "One of six 'Six 3d6 with choice' pregenerated sets for real necromancer";
pub static ADND_CBON4_HELP: &str = "One of six 'Best six of 12 3d6 with choice' pregenerated sets for real necromancer";
pub static ADND_CBON5_HELP: &str = "One of six 'Six 4d6 drop lowest with choice' pregenerated sets for real necromancer";
pub static ADND_CBON6_HELP: &str = "One of six '8+7d6' pregenerated sets for real necromancer";
pub static ADND_CBON7_HELP: &str = "Roll 10+1d8, Repeat six times. Assign the results to your character's six abilities as you wish.";
pub static ADND_CBON8_HELP: &str = "Roll 10+1d8, Repeat six times. Assign the results to your character's six abilities as you wish. +1 to INT&WIS (max 18).";
pub static ADND_CBON7WOCHOICE_HELP: &str = "Roll 10+1d8, use the result for character's Strength ability score. Repeat for Dexterity, Constitution, Intelligence, Wisdom, and Charisma.";
pub static ADND_CBON8WOCHOICE_HELP: &str = "Roll 10+1d8, use the result for character's Strength ability score. Repeat for Dexterity, Constitution, Intelligence, Wisdom, and Charisma. +1 to INT&WIS (max 18).";

pub static ADND1_1_HELP: &str = ADND5_HELP;
pub static ADND1_2_HELP: &str = ADND4_HELP;
pub static ADND1_3_HELP: &str = "Roll 3d6 six times, use the best result for character's Strength ability score. Repeat for Dexterity, Constitution, Intelligence, Wisdom, and Charisma.";
pub static ADND1_4_HELP: &str = "Roll 3d6, use the result for character's Strength ability score. Repeat for Dexterity, Constitution, Intelligence, Wisdom, and Charisma. Repeat this 12 times, use the best set.";

pub static DND3_HELP: &str = ADND5_HELP;
pub static DND3ORGANIC_HELP: &str = "Roll 4d6 and discard the lowest die. Repeat six times. Place in order, reroll any one ability, switch any two ability.";
pub static DND3CA_HELP: &str = "Roll 3d6 six times. Assign the results to your character's six abilities as you wish. Reroll if there isn't any scores of 12 or better, or ability modifiers total -3 or lower.";
pub static DND3RA_HELP: &str = "Roll 3d6, use the result for character's Strength ability score. Repeat for Dexterity, Constitution, Intelligence, Wisdom, and Charisma. Reroll if there isn't any scores of 12 or better, or ability modifiers total -3 or lower.";
pub static DND3HP_HELP: &str = "Roll 5d6 and discard the two lowest dices. Repeat six times. Assign the results to your character's six abilities as you wish. Reroll if there isn't any scores of 15 or better, or ability modifiers total +2 or lower.";

pub static DND35_HELP: &str = ADND5_HELP;
pub static DND35ORGANIC_HELP: &str = DND3ORGANIC_HELP;
pub static DND35CA_HELP: &str = DND3CA_HELP;
pub static DND35RA_HELP: &str = DND3RA_HELP;
pub static DND35HP_HELP: &str = DND3HP_HELP;
pub static DND35ELITE_HELP: &str = "15,14,13,12,10,8. Assign to your character's six abilities as you wish.";

pub static DND4_HELP: &str = "Roll 4d6 and discard the lowest die. Repeat six times. Assign the results to your character's six abilities as you wish. Reroll if ability modifiers total lower than +4.";
pub static DND4STANDARD_HELP: &str = "16,14,13,12,11,10. Assign to your character's six abilities as you wish.";

pub static DND5_HELP: &str = ADND5_HELP;
pub static DND5STANDARD_HELP: &str = DND35ELITE_HELP;

pub static PFSTANDARD_HELP: &str = ADND5_HELP;
pub static PFCLASSIC_HELP: &str = "Roll 3d6 six times. Assign the results to your character's six abilities as you wish.";
pub static PFHEROIC_HELP: &str = ADND3_HELP;

pub static PF2_HELP: &str = ADND5_HELP;

pub static SF_HELP: &str = ADND5_HELP;
pub static SFFOCUSED_HELP: &str = "18,14,11,10,10,10. Assign to your character's six abilities as you wish.";
pub static SFSPLIT_HELP: &str = "16,16,11,10,10,10. Assign to your character's six abilities as you wish.";
pub static SFVERSATILE_HELP: &str = "14,14,14,11,10,10. Assign to your character's six abilities as you wish.";

pub static CP2013_1_HELP: &str = "Roll 9d10, use result for Character Points.";
pub static CP2013_2_HELP: &str = "Roll 30+6d10, use result for Character Points.";
pub static CP2020_1_HELP: &str = CP2013_1_HELP;
pub static CP2020_2_HELP: &str = "Roll 1d10 nine times, rerolling any result of 2 or less. Assign the results to your character's nine abilities as you wish.";
pub static CP2020_3_HELP: &str = "Roll 1d10 nine times, rerolling any result of 2 or less, and use the result for character's INT ability. Repeat for REF, CL, TECH, LK, ATT, MA, EMP, and BODY.";

pub static CYBERSPACE_1_HELP: &str = "Roll 1d100 eleven times. Assign the results to your character's eleven stats as you wish.";
pub static CYBERSPACE_2_HELP: &str = "Roll 1d100 eleven times and use the result for character's Co stat. Repeat for Ag, SD, Re, Me, St, Qu, Em, In, Pr, and Ap.";

pub static ARM1_HELP: &str = "Roll two 1d10, subtract latter from former. Repeat three times and assign results to your character's four pair of characteristics as you wish.";
pub static ARM2_HELP: &str = "Roll two 1d10, subtract latter from former, and use the result for character's Int/Per pair of characteristic. Repeat for Str/Sta, Pre/Com, and Dex/Qik";

pub static WH40K_HELP: &str = "Roll 2d10 for each characteristic, place in order.";
pub static WH40KREROLL_HELP: &str = "Roll 2d10 for each characteristis, place in order. Reroll one.";
pub static WH40KCHOICE_HELP: &str = "Roll nine 2d10. Assign the results to your characteristics as you wish.";
pub static WH40KCHOICEREROLL_HELP: &str = "Roll nine 2d10. Reroll one. Assign the results to your characteristics as you wish.";

pub static SWN_HELP: &str = "Roll 3d6, use the result for character's Strength ability score. Repeat for Dexterity, Constitution, Intelligence, Wisdom, and Charisma. Then you can change one ability to 14.";
pub static SWNSTANDARD_HELP: &str = "14,12,11,10,9,7. Assign to your character's six abilities as you wish.";

pub static RQ6_HELP: &str = "Roll 3d6, use the result for character's STR. Repeat for CON, DEX, POW and CHA; then roll 2d6+6 for SIZ and INT.";
pub static RQ6CHOICE_HELP: &str = "Roll 3d6 five times, assign the results to STR, CON, DEX, POW and CHA as you wish; then roll 2d6+6 two times and assign the results to SIZ and INT as you wish.";
