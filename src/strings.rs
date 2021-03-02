// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub static METHODS_MESSAGE: &str = "Generation methods:";

pub static DICEERROR_DICES0: &str = "Can't throw 0 dices!";
pub static DICEERROR_SIDES0: &str = "Can't throw 0-sided dice!";
pub static DICEERROR_BADDROP: &str = "Can't drop {drop} dices from {n} dices!";
pub static DICEERROR_BADCROP: &'static str = "Can't crop {crop} dices from {n} dices!";

pub static BADDICECODE_ERROR_MSG: &str = "Can't parse the dice code:";
pub static DICECODEDECRYPTION_ERROR_MSG: &str = "Dice code decription internal error.";

pub static UNKNOWNMETHOD_ERROR_MSG: &str = "Unknown method";
pub static UNKNOWNSTATLIST_ERROR_MSG: &str = "Internal error: Unknown stat list for method.";
pub static ZEROSTAT_ERROR_MSG: &str = "Can't calculate statistics from zero-length slice";

pub static DICECODES_HELP_MSG: &str = "Dice codes format:
DiceCodes = DiceCode{ DiceCode}
DiceCode = SingleDice{(+|-|*|/|%|^)SingleDice}
SingleDice = [NUM1]d[NUM2][(drop|d)NUM3(crop|c)NUM4|((drop|d)NUM3|(crop|c)NUM4|(greatest|g)NUM5|(lowest|l)NUM6)][(plus|p)NUM7][(minus|m)NUM8]|NUM9

Where:
	NUM1 is number of dices (default = 1);
	NUM2 is sides of dices (default = 6);
	NUM3 is number of lowest dice to be dropped;
	NUM4 is number of greatest dice to be dropped;
	NUM5 is number of greatest dice to be kept;
	NUM6 is number of lowest dice to be kept;
	NUM7 is value to be added to sum;
	NUM8 is value to be subtracted to sum;
	NUM9 is constant

Examples:
    10 - 10
	d - one 6-sided dice
	1d - one 6-sided dice
	2d - two 6-sided dices
	d4 - one 4-sided dice
	d20 - one 20-sided dice
	2d8 - two 8-sided dices
	2d20drop1 - greatest of two 20-sided dices
	2d20d1 - greatest of two 20-sided dices
	2d20crop1 - lowest of two 20-sided dices
	2d20c1 - lowest of two 20-sided dices
	4ddrop1 - greatest three of four 6-sided dices
	4dgreatest3 - greatest three of four 6-sided dices
	4dg3 - greatest three of four 6-sided dices
	4dlowest3 - lowest three of four 6-sided dices
	4dl3 - lowest three of four 6-sided dices
	2d8plus4 - two 8-sided dices, plus 4 to sum
	2d8p4 - two 8-sided dices, plus 4 to sum
	4dminus2 - four 6-sided dices, minus 2 to sum
	4dm2 - four 6-sided dices, minus 2 to sum
	6d8drop1crop2 - throw six 8-sided dices, drop lowest and two greatest
	6d8drop1c2p10minus20 - throw six 8-sided dices, drop lowest and two greatest, adds remaining, plus 10 and minus 20 to sum
	d+d - roll two 6-sided dice and add results
	2d*d8 - roll two 6-sided dice and multiply theirs sum by result of 8-sided dice roll
	6-2d*d8 - 6 minus (roll two 6-sided dice and multiply theirs sum by result of 8-sided dice roll)
	6d8drop1c2p10minus20%6d8drop1c2p10minus20+6d8drop1c2p10minus20^6d8drop1c2p10minus20 - 
		throw six 8-sided dices, drop lowest and two greatest, adds remaining, plus 10 and minus 20 to sum,
		takes the remainder of division the result of latter operation by the result of the same dice roll
		then roll the same set of dice (third!) and power the result to the result of fourth roll of the same set of dices
		then add former and latter results.

	And remember, you can use all dice codes with space divisor! For example: 2d8 2d8 2d8
	";