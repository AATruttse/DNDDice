// Copyright (c) 2021 Anton A. Truttse (Dargot) <dargot@yandex.ru>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rand::Rng;
use rand::distributions::{Uniform};

use crate::errors::DiceError;
use crate::log::{log_dices, log_dices_res, log_dices_title};
use crate::render::{render_dices, render_dices_res, render_dices_title};

/// Result int type for all dices
pub type IntValue = i32;

/// Rolls n dices with d sides, drops drop lowest, crop greatest, and adds plus
#[inline(always)]
pub fn n_d_drop_crop_plus(n: usize, d: usize, plus: IntValue, drop: usize, crop: usize) -> Result<IntValue, DiceError> {
    n_d_reroll_drop_crop_plus(n, d, &[], plus, drop, crop)
}


/// Rolls n dices with d sides, drops drop lowest and adds plus
#[inline(always)]
pub fn n_d_drop_plus(n: usize, d: usize, plus: IntValue, drop: usize) -> Result<IntValue, DiceError> {
    n_d_drop_crop_plus(n, d, plus, drop, 0)
}

/// Rolls n dices with d sides, drops crop greatest and adds plus
#[inline(always)]
pub fn n_d_crop_plus(n: usize, d: usize, plus: IntValue, crop: usize) -> Result<IntValue, DiceError> {
    n_d_drop_crop_plus(n, d, plus, 0, crop)
}

/// Rolls n dices with d sides, drops drop lowest
#[inline(always)]
pub fn n_d_drop(n: usize, d: usize, drop: usize) -> Result<IntValue, DiceError> {
    n_d_drop_plus(n, d, 0, drop)
}

/// Rolls _n dices with _d sides, drops _crop greatest
#[inline(always)]
pub fn n_d_crop(n: usize, d: usize, crop: usize) -> Result<IntValue, DiceError> {
    n_d_crop_plus(n, d, 0, crop)
}

/// Rolls _n dices with _d sides and adds _plus
#[inline(always)]
pub fn n_d_plus(n: usize, d: usize, plus: IntValue) -> Result<IntValue, DiceError> {
    n_d_drop_plus(n, d, plus, 0)
}

/// Rolls _n dices with _d sides
#[inline(always)]
pub fn n_d(n: usize, d: usize) -> Result<IntValue, DiceError> {
    n_d_plus(n, d, 0)
}

/// Rolls dice and rerolls if result is in reroll range
#[inline(always)]
fn dice_reroll(rng: &mut rand::rngs::ThreadRng, dice: &Uniform<usize>, reroll: &[usize]) -> usize {

    let mut _result = 0;
    loop {
        _result = rng.sample(&dice);
        if !reroll.iter().any(|x| *x == _result)
        {
            break;
        }
    }

    _result
}

/// Rolls n dices with d sides, rerolls reroll, drops drop lowest, crop greatest, and adds plus
#[inline(always)]
pub fn n_d_reroll_drop_crop_plus(n: usize, d: usize, reroll: &[usize], plus: IntValue, drop: usize, crop: usize) -> Result<IntValue, DiceError> {
    if n == 0 {
        return Err(DiceError::Dices0);
    }

    if d == 0 {
        return Err(DiceError::Sides0);
    }    

    render_dices_title(n, d, reroll, plus, drop, crop);
    log_dices_title(n, d, reroll, plus, drop, crop);

    let mut rng = rand::thread_rng();
    let dice = Uniform::new(1, d + 1);

    let mut dices : Vec<usize> = match reroll.is_empty() {
        true => (0..n).map(|_| rng.sample(&dice)).collect(),
        false => (0..n).map(|_| dice_reroll(&mut rng, &dice, reroll)).collect()
    };
  
    render_dices(&dices);
    log_dices(&dices);

    if drop > 0 {
        if drop >= dices.len() {
            return Err(DiceError::BadDrop{
                n: dices.len(),
                drop: drop
            });
        }

        dices.sort();
        dices.reverse();
        dices.truncate(dices.len() - drop);
    }

    if crop > 0 {
        if crop >= dices.len() {
            return Err(DiceError::BadCrop{
                n: dices.len(),
                crop: crop
            });
        }

        dices.sort();
        dices.truncate(dices.len() - crop);
    }


    let sum : usize = dices.iter().sum();
    let result = plus + sum as IntValue;

    render_dices_res(result);
    log_dices_res(result);

    Ok(result)
}


/// Rolls n dices with d sides, rerolls reroll, drops drop lowest and adds plus
#[inline(always)]
pub fn n_d_reroll_drop_plus(n: usize, d: usize, reroll: &[usize], plus: IntValue, drop: usize) -> Result<IntValue, DiceError> {
    n_d_reroll_drop_crop_plus(n, d, reroll, plus, drop, 0)
}

/// Rolls n dices with d sides, rerolls reroll, drops crop greatest and adds plus
#[inline(always)]
pub fn n_d_reroll_crop_plus(n: usize, d: usize, reroll: &[usize], plus: IntValue, crop: usize) -> Result<IntValue, DiceError> {
    n_d_reroll_drop_crop_plus(n, d, reroll, plus, 0, crop)
}

/// Rolls n dices with d sides, rerolls reroll, drops drop lowest
#[inline(always)]
pub fn n_d_reroll_drop(n: usize, d: usize, reroll: &[usize], drop: usize) -> Result<IntValue, DiceError> {
    n_d_reroll_drop_plus(n, d, reroll, 0, drop)
}

/// Rolls _n dices with _d sides, rerolls reroll, drops _crop greatest
#[inline(always)]
pub fn n_d_reroll_crop(n: usize, d: usize, reroll: &[usize], crop: usize) -> Result<IntValue, DiceError> {
    n_d_reroll_crop_plus(n, d, reroll, 0, crop)
}

/// Rolls _n dices with _d sides, rerolls reroll and adds _plus
#[inline(always)]
pub fn n_d_reroll_plus(n: usize, d: usize, reroll: &[usize], plus: IntValue) -> Result<IntValue, DiceError> {
    n_d_reroll_drop_plus(n, d, reroll, plus, 0)
}

/// Rolls _n dices with _d sides, rerolls reroll
#[inline(always)]
pub fn n_d_reroll(n: usize, d: usize, reroll: &[usize]) -> Result<IntValue, DiceError> {
    n_d_reroll_plus(n, d, reroll, 0)
}