use rand::Rng;
use rand::distributions::{Uniform};

use crate::errors::DiceError;
use crate::init::OPT;

/// Result int type for all dices
pub type IntValue = i32;

/// Throws n dices with d sides, drops drop lowest and adds plus
#[inline(always)]
pub fn n_d_drop_plus(n: usize, d: usize, plus: IntValue, drop: usize) -> Result<IntValue, DiceError> {
    if n == 0 {
        return Err(DiceError::Dices0);
    }

    if d == 0 {
        return Err(DiceError::Sides0);
    }    

    let mut rng = rand::thread_rng();
    let dice = Uniform::new(1, d + 1);
    let mut dices: Vec<usize> = (0..n).map(|_| rng.sample(&dice)).collect();

    if OPT.debug || OPT.verbose > 0 {
        println!("{:?}", dices);
    }

    if drop > 0 {
        if drop >= n {
            return Err(DiceError::BadDrop{
                n: n,
                drop: drop
            });
        }

        dices.sort();
        dices.reverse();
        dices.truncate(n - drop);
    }

    let sum : usize = dices.iter().sum();
    Ok(plus + sum as IntValue)
}

/// Throws n dices with d sides, drops crop greatest and adds plus
#[inline(always)]
pub fn n_d_crop_plus(n: usize, d: usize, plus: IntValue, crop: usize) -> Result<IntValue, DiceError> {
    if n == 0 {
        return Err(DiceError::Dices0);
    }

    if d == 0 {
        return Err(DiceError::Sides0);
    }    

    let mut rng = rand::thread_rng();
    let dice = Uniform::new(1, d + 1);
    let mut dices: Vec<usize> = (0..n).map(|_| rng.sample(&dice)).collect();

    if OPT.debug || OPT.verbose > 0 {
        println!("{:?}", dices);
    }

    if crop > 0 {
        if crop >= n {
            return Err(DiceError::BadCrop{
                n: n,
                crop: crop
            });
        }

        dices.sort();
        dices.truncate(n - crop);
    }

    let sum : usize = dices.iter().sum();
    Ok(plus + sum as IntValue)
}

/// Throws n dices with d sides, drops drop lowest
#[inline(always)]
pub fn n_d_drop(n: usize, d: usize, drop: usize) -> Result<IntValue, DiceError> {
    n_d_drop_plus(n, d, 0, drop)
}

/// Throws _n dices with _d sides, drops _crop greatest
#[inline(always)]
pub fn n_d_crop(n: usize, d: usize, crop: usize) -> Result<IntValue, DiceError> {
    n_d_crop_plus(n, d, 0, crop)
}

/// Throws _n dices with _d sides and adds _plus
#[inline(always)]
pub fn n_d_plus(n: usize, d: usize, plus: IntValue) -> Result<IntValue, DiceError> {
    n_d_drop_plus(n, d, plus, 0)
}

/// Throws _n dices with _d sides
#[inline(always)]
pub fn n_d(n: usize, d: usize) -> Result<IntValue, DiceError> {
    n_d_plus(n, d, 0)
}