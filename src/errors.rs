use custom_error::custom_error;

custom_error!{pub DiceError
    Dices0      = "Can't throw 0 dices!",
    Sides0     = "Can't throw 0-sided dice!",
    BadDrop{n:usize, drop:usize} = "Can't drop {drop} dices from {n} dices!",
    BadCrop{n:usize, crop:usize} = "Can't crop {crop} dices from {n} dices!"
}