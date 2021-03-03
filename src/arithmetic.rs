use std::collections::BTreeMap;

use crate::dices::IntValue;
use crate::strings::DICECODEDECRYPTION_ERROR_MSG;

pub type Arythmetic = (&'static str, IntValue);

/// Type for arithmetic operation's ptr
type ArithmeticOp = fn (IntValue, IntValue) -> IntValue;

/// Type for arithmetic operations' BTreeMap 
type OpsMap = BTreeMap<&'static str, ArithmeticOp>;

lazy_static! {
    /// Arithmetic operations' groups, divided by order in which they executed
    #[derive(Copy, Clone)]
    static ref OPS_GROUPS: Vec<Vec<&'static str>> = vec![
        vec!["+", "-"],
        vec!["*", "/", "%"],
        vec!["^"]
    ];

    /// BTreeMap for arithmetics operations which pairs operation's symbol with function ptr
    pub static ref OPSMAP: OpsMap = {
        let mut m = OpsMap::new();

        m.insert("+", plus);
        m.insert("-", minus);
        m.insert("*", mult);
        m.insert("/", div);
        m.insert("%", modul);
        m.insert("^", power);

        m
    };
}

/// Public function arithmetic operations' slice processing
pub fn process_arithmetic(ar: &[Arythmetic]) -> IntValue {
    calc_arithmetic(ar, 0)
}

/// Recursive function which process arithmetic ops' slice in right order
fn calc_arithmetic(ar: &[Arythmetic], order: usize) -> IntValue {
//    let mut chains: Vec<&[Arythmetic]> = Vec::new();
    let mut results: Vec<Arythmetic> = Vec::new();
    
    let mut idx: usize = 0;

    // build vector of local operations, calling recursively this function if see ops with greater priority
    for i in 0..ar.len() {
        if i == (ar.len()-1) || ops_order(ar[i+1].0) == order {
            let chain = &ar[idx..i+1];
//            chains.push(chain);
            results.push(match chain.len() {
                    1 => chain[0],
                    _ => (chain[0].0, calc_arithmetic(chain, order+1))
                }
            );
            idx = i+1;            
        }
    }

//    println!("Chains: {:?}", chains);    
//    println!("Results: {:?}", results);
    // process local operations one by one    
    let mut result = results[0].1;
    for pair in results.windows(2) {
        result = OPSMAP.get(pair[1].0).expect(DICECODEDECRYPTION_ERROR_MSG)(result, pair[1].1);
    }

    //println!("{}", result);

    result
}

/// Find arithmetic operaion's order
fn ops_order(op: &str) -> usize {
    for (i, ops_group) in OPS_GROUPS.iter().enumerate() {
        if ops_group.contains(&op) {
            return i;
        }
    }

    return usize::MAX;
}

/// Function for arithmetic operation - plus
#[inline(always)]
fn plus (a: IntValue, b: IntValue) -> IntValue {
    a+b
}

/// Function for arithmetic operation - minus
#[inline(always)]
fn minus (a: IntValue, b: IntValue) -> IntValue {
    a-b
}

/// Function for arithmetic operation - multiplication
#[inline(always)]
fn mult (a: IntValue, b: IntValue) -> IntValue {
    a*b
}

/// Function for arithmetic operation - division
#[inline(always)]
fn div (a: IntValue, b: IntValue) -> IntValue {
    a/b
}

/// Function for arithmetic operation - modul
#[inline(always)]
fn modul (a: IntValue, b: IntValue) -> IntValue {
    a%b
}

/// Function for arithmetic operation - power
#[inline(always)]
fn power (a: IntValue, b: IntValue) -> IntValue {
    a.pow(b as u32)
}