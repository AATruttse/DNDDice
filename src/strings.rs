// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub static BADDICECODE_ERROR_MSG: &str = "Can't parse the dice code:";
pub static UNKNOWNMETHOD_ERROR_MSG: &str = "Unknown method";
pub static UNKNOWNSTATLIST_ERROR_MSG: &str = "Internal error: Unknown stat list for method.";
pub static ZEROSTAT_ERROR_MSG: &str = "Can't calculate statistics from zero-length slice";
pub static DICECODES_HELP_MSG: &str = "Dice code format: [NUM1]d[NUM2][dropNUM3][cropNUM4][+NUM5][-NUM6]

Where:
	NUM1 is number of dices (default = 1);
	NUM2 is sides of dices (default = 6);
	NUM3 is lowest dice to be dropped;
	NUM4 is greatest dice to be dropped;
	NUM5 is value to be added to sum;
	NUM6 is value to be subtracted to sum;

Examples:
	d - one 6-sided dice
	1d - one 6-sided dice
	2d - two 6-sided dices
	d4 - one 4-sided dice
	d20 - one 20-sided dice
	2d8 - two 8-sided dices
	2d20drop1 - greatest of two 20-sided dices
	2d20crop1 - lowest of two 20-sided dices
	4ddrop1 - greatest three of four 6-sided dices
	2d8+4 - two 8-sided dices, plus 4 to sum
	4d-2 - four 6-sided dices, minus 2 to sum
	6d8drop1crop2+10-20 - throw six 8-sided dices, drop greatest and two lowest, adds remaining, plus 10 and minus 20 to sum";